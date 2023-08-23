''' Using flask to make an api '''
# import necessary libraries and functions
import os
import logging
from flask import Flask, jsonify, request
import helper
from sys import stdout
from opentelemetry.instrumentation.flask import FlaskInstrumentor
from opentelemetry.instrumentation.wsgi import OpenTelemetryMiddleware
import json
from ast import literal_eval
from flask_sqlalchemy import SQLAlchemy


from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.resources import SERVICE_NAME, Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor

from pymongo import MongoClient
from pandas import DataFrame
from bson import json_util
from opentelemetry.instrumentation.requests import RequestsInstrumentor

# Logging Configuration
logging.basicConfig(level=logging.INFO, format="[%(asctime)s] %(name)-12s %(levelname)-8s %(filename)s:%(funcName)s %(message)s")
logFormatter = logging.Formatter("[%(asctime)s] %(name)-12s %(levelname)-8s %(filename)s:%(funcName)s %(message)s")
logger = logging.getLogger('werkzeug')
logger.setLevel(logging.INFO)
consoleHandler = logging.StreamHandler(stdout) #set streamhandler to stdout
consoleHandler.setFormatter(logFormatter)
# logger.addHandler(consoleHandler)
config = helper.read_config()

ENABLE_TELEMETRY= literal_eval(os.environ['ENABLE_TELEMETRY'])

if ENABLE_TELEMETRY:
    logger.info('ENABLE_TELEMETRY=True, enabling tracing...')
    # Tracing Configuration
    resource = Resource(attributes={
        SERVICE_NAME: "client",
        "application.name": "client",
        "env.name": "prod"
        })

    provider = TracerProvider(resource=resource)
    processor = BatchSpanProcessor(OTLPSpanExporter(endpoint="http://opentelemetry-collector.monitoring.svc.cluster.local:4317"))
    provider.add_span_processor(processor)
    trace.set_tracer_provider(provider)

    app = Flask(__name__)
    FlaskInstrumentor().instrument_app(app)
    app.wsgi_app = OpenTelemetryMiddleware(app.wsgi_app, tracer_provider=provider)

    RequestsInstrumentor().instrument()
else:
    logger.info('ENABLE_TELEMETRY=False, tracing is disabled')
    app = Flask(__name__)

MONGODB_USERNAME= os.environ['ME_CONFIG_MONGODB_ADMINUSERNAME']
MONGODB_PASSWD= os.environ['ME_CONFIG_MONGODB_ADMINPASSWORD']
ME_CONFIG_MONGODB_SERVER=os.environ['ME_CONFIG_MONGODB_SERVER']
POSTGRES_USER= os.environ['POSTGRES_USER']
POSTGRES_PASSWORD= os.environ['POSTGRES_PASSWORD']
POSTGRES_DB= os.environ['POSTGRES_DB']
POSTGRES_URL= os.environ['POSTGRES_URL']

app.config['SQLALCHEMY_DATABASE_URI'] = f"postgresql://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{POSTGRES_URL}:5432/{POSTGRES_DB}"
db = SQLAlchemy(app)

# pylint: disable=too-few-public-methods
class CustomersModel(db.Model):
    __tablename__ = 'customers'

    customer_id = db.Column(db.Integer, primary_key=True)
    customer_name = db.Column(db.String())

    def __init__(self, name):
        self.customer_name = name

    def __repr__(self):
        return f"<Customer {self.customer_name}>"

with app.app_context():
    db.create_all()

def get_database():
    CONNECTION_STRING = f"mongodb://{MONGODB_USERNAME}:{MONGODB_PASSWD}@{ME_CONFIG_MONGODB_SERVER}/"
    client = MongoClient(CONNECTION_STRING)
    return client['orders']

def parse_json(data):
    return json.loads(json_util.dumps(data))

@app.route('/', methods = ['GET', 'POST'])
def home():
    if(request.method == 'GET'):
        logger.info('sample log')
        data = "Hello scaffold from asd"
        return jsonify({'data': data,'lambda-response': 'asd'})
    return jsonify({'request': 'POST'})

@app.route('/pg/customer', methods = ['GET', 'POST'])
def pg_customer():
    if(request.method == 'GET'):
        logger.info('pg customer get')
        logger.info(print(request.query_string))
        customer_name = request.args.get('customer_name')
        logger.info ("querying postgres for customer: %s", customer_name)
        customer = CustomersModel.query.filter_by(customer_name=customer_name).first()
        customer_info = {'customer_name': customer.customer_name,'customer_id': customer.customer_id, 'api_version': 'v1'}
        logger.info (customer_info)
        return(jsonify(customer_info))
    logger.info('pg customer post')
    logger.info(request.form)
    logger.info(request.json)
    customer_name = request.json.get('customer_name')
    logger.info ("creating customer: %s" ,customer_name)
    new_customer = CustomersModel(name=customer_name)
    db.session.add(new_customer)
    db.session.commit()
    logger.info("customer %s inserted", customer_name)
    return jsonify({'customer': customer_name,'status': 'inserted', 'api_version': 'v1'})

@app.route('/mongo/orders', methods = ['GET', 'POST'])
def mongo_orders():
    if(request.method == 'GET'):
        dbname = get_database()
        collection_name = dbname["orders"]
        product_name = request.args.get('product_name')
        item_details = collection_name.find({"product_name" : product_name})
        items_df = DataFrame(item_details).transpose()
        output = parse_json(items_df.to_dict())
        logger.info(output)
        return output
    dbname = get_database()
    collection_name = dbname["orders"]
    customer_id = request.json.get('customer_id')
    product_name = request.json.get('product_name')
    item = {
    "customer_id" : customer_id,
    "product_name" : product_name,
    }
    collection_name.insert_one(item)
    logger.info("item %s inserted", item)
    output=parse_json(dict(item, status="inserted"))
    logger.info(output)
    return output

@app.route('/home/<int:num>', methods = ['GET'])
def disp(num):
    return jsonify({'data': num**2})

@app.route('/health', methods = ['GET'])
def health():
    return jsonify({'status': 'healthy'})

if __name__ == '__main__':
    app.run(debug = True,  host="0.0.0.0", port = config['General']['port'])