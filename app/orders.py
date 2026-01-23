import logging
import json
from bson import json_util

logger = logging.getLogger("werkzeug")
logger.setLevel(logging.INFO)


def parse_json(data):
    return json.loads(json_util.dumps(data))


def post_order(connection, customer_id, product_name):
    collection_name = connection["orders"]
    item = {
        "customer_id": customer_id,
        "product_name": product_name,
    }
    collection_name.insert_one(item)
    output = parse_json(dict(item, status="inserted"))

    return output

def delete_orders(connection):
    collection_name = connection["orders"]
    deleted_count = collection_name.delete_many({}).deleted_count
    output = parse_json(dict(status="deleted", deleted_count=deleted_count))

    return output