from fastapi.testclient import TestClient
import mongomock


def _mongo_db():
    return mongomock.MongoClient()["orders"]


def test_health(main_module):
    with TestClient(main_module.app) as client:
        resp = client.get("/health")
    assert resp.status_code == 200
    assert resp.json() == {"status": "healthy"}


def test_home_squares(main_module):
    with TestClient(main_module.app) as client:
        resp = client.get("/home/7")
    assert resp.status_code == 200
    assert resp.json() == {"data": 49}


def test_post_and_get_pg_customer(main_module):
    with TestClient(main_module.app) as client:
        created = client.post("/pg/customer", json={"customer_name": "mark"})
        assert created.status_code == 201
        assert created.json()["status"] == "inserted"

        fetched = client.get("/pg/customer", params={"customer_name": "mark"})
        assert fetched.status_code == 200
        body = fetched.json()
        assert body["customer_name"] == "mark"
        assert isinstance(body["customer_id"], int)


def test_get_pg_customer_404(main_module):
    with TestClient(main_module.app) as client:
        resp = client.get("/pg/customer", params={"customer_name": "does-not-exist"})
    assert resp.status_code == 404


def test_mongo_orders_post_and_get(main_module, monkeypatch):
    # Route calls `get_database()` which normally builds a real MongoClient.
    # Patch it to use the mocked Mongo connection from pytest-mock-resources.
    mongo = _mongo_db()
    monkeypatch.setattr(main_module, "get_database", lambda: mongo)

    with TestClient(main_module.app) as client:
        created = client.post(
            "/mongo/orders",
            json={"customer_id": "2", "product_name": "apple"},
        )
        assert created.status_code == 201
        assert created.json()["status"] == "inserted"

        fetched = client.get("/mongo/orders", params={"product_name": "apple"})
        assert fetched.status_code == 200
        payload = fetched.json()

    # The API returns a pandas->dict shaped payload; just assert our values are present.
    flattened = str(payload)
    assert "apple" in flattened
    assert "2" in flattened
