"""Using flask to make an api"""

# import necessary libraries and functions
import os
from loguru import logger
from pathlib import Path

# from flask import Flask, jsonify, request
from contextlib import asynccontextmanager
from typing import Union
from fastapi import Depends, FastAPI, HTTPException
from pydantic import BaseModel
from sqlmodel import Field, Session, SQLModel, create_engine, select
from app import helper, orders
import json

from pymongo import MongoClient
from bson import json_util

from app import telemetry

# Local development convenience: load env vars from .env.local if present.
# This is a no-op in Docker/K8s where env vars are provided by the runtime.
try:
    from dotenv import load_dotenv

    repo_root = Path(__file__).resolve().parents[1]
    dotenv_path = repo_root / ".env.local"
    if dotenv_path.exists():
        load_dotenv(dotenv_path=dotenv_path, override=False)
except ModuleNotFoundError:
    # python-dotenv is expected in local dev; keep runtime working even if absent.
    pass

config = helper.read_config()


def _parse_bool_env(name: str, default: bool = False) -> bool:
    raw = os.getenv(name)
    if raw is None:
        return default

    value = raw.strip().lower()
    if value in {"1", "true", "t", "yes", "y", "on"}:
        return True
    if value in {"0", "false", "f", "no", "n", "off", ""}:
        return False

    raise RuntimeError(
        f"Invalid boolean for {name}: {raw!r}. Use true/false, 1/0, yes/no, on/off."
    )


ENABLE_TELEMETRY = _parse_bool_env("ENABLE_TELEMETRY", default=False)


def _require_env(name: str) -> str:
    value = os.getenv(name)
    if value is None or value == "":
        raise RuntimeError(
            f"Missing required environment variable: {name}. "
            "Set it in your shell, Docker/K8s env, or in .env.local."
        )
    return value


if ENABLE_TELEMETRY:
    logger.info("ENABLE_TELEMETRY=True, enabling telemetry...")
else:
    logger.info("ENABLE_TELEMETRY=False, telemetry is disabled")


MONGODB_USERNAME = _require_env("ME_CONFIG_MONGODB_ADMINUSERNAME")
MONGODB_PASSWD = _require_env("ME_CONFIG_MONGODB_ADMINPASSWORD")
ME_CONFIG_MONGODB_SERVER = _require_env("ME_CONFIG_MONGODB_SERVER")
POSTGRES_USER = _require_env("POSTGRES_USER")
POSTGRES_PASSWORD = _require_env("POSTGRES_PASSWORD")
POSTGRES_DB = _require_env("POSTGRES_DB")
POSTGRES_URL = _require_env("POSTGRES_URL")


@asynccontextmanager
async def lifespan(app: FastAPI):
    init_db()
    yield


app = FastAPI(lifespan=lifespan)


def _postgres_dsn() -> str:
    # In docker-compose.yaml POSTGRES_URL is the hostname (e.g. "postgres").
    # Allow overriding with a full DSN (e.g. "postgresql+psycopg2://user:pass@host:5432/db").
    if "://" in POSTGRES_URL:
        return POSTGRES_URL

    host = POSTGRES_URL
    port = 5432
    if ":" in host:
        host, port_str = host.rsplit(":", 1)
        try:
            port = int(port_str)
        except ValueError:
            raise RuntimeError(f"Invalid POSTGRES_URL (bad port): {POSTGRES_URL}")

    return f"postgresql+psycopg2://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{host}:{port}/{POSTGRES_DB}"


engine = create_engine(_postgres_dsn(), pool_pre_ping=True)

telemetry.configure_telemetry(app=app, engine=engine, enabled=ENABLE_TELEMETRY)


class Customer(SQLModel, table=True):
    __tablename__ = "customers"

    customer_id: int | None = Field(default=None, primary_key=True)
    customer_name: str


def init_db() -> None:
    SQLModel.metadata.create_all(engine)


def get_session():
    with Session(engine) as session:
        yield session


class CustomerCreate(SQLModel):
    customer_name: str


@app.get("/pg/customer")
def get_pg_customer(customer_name: str, session: Session = Depends(get_session)):
    customer = session.exec(
        select(Customer).where(Customer.customer_name == customer_name)
    ).first()
    if customer is None:
        raise HTTPException(status_code=404, detail="Customer not found")
    return {
        "customer_name": customer.customer_name,
        "customer_id": customer.customer_id,
        "api_version": "v1",
    }


@app.post("/pg/customer", status_code=201)
def post_pg_customer(payload: CustomerCreate, session: Session = Depends(get_session)):
    new_customer = Customer(customer_name=payload.customer_name)
    session.add(new_customer)
    session.commit()
    session.refresh(new_customer)
    return {
        "customer": new_customer.customer_name,
        "status": "inserted",
        "api_version": "v1",
    }


def get_database():
    CONNECTION_STRING = (
        f"mongodb://{MONGODB_USERNAME}:{MONGODB_PASSWD}@{ME_CONFIG_MONGODB_SERVER}/"
    )
    client = MongoClient(CONNECTION_STRING)
    return client["orders"]


def parse_json(data):
    return json.loads(json_util.dumps(data))


class Order(BaseModel):
    customer_id: str
    product_name: str


@app.get("/mongo/orders")
def get_mongo_orders(product_name: Union[str, None] = None):
    dbname = get_database()
    collection_name = dbname["orders"]
    query = {"product_name": product_name} if product_name is not None else {}
    items = list(collection_name.find(query))
    return parse_json(items)


@app.post("/mongo/orders", status_code=201)
def post_mongo_orders(order: Order):
    dbname = get_database()
    output = orders.post_order(
        dbname,
        customer_id=order.customer_id,
        product_name=order.product_name,
    )
    return output


@app.get("/home/{num}")
def disp(num: int):
    return {"data": num**2}


@app.get("/health")
def health():
    return {"status": "healthy"}

