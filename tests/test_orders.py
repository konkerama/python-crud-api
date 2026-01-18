from app import orders
import mongomock


def test_insert_into_orders_collection():
    mongo = mongomock.MongoClient()["orders"]
    output = orders.post_order(mongo, "foo", "bar")
    assert output["status"] == "inserted"

    collection = mongo["orders"]
    returned = collection.find_one()

    assert {
        "customer_id": returned["customer_id"],
        "product_name": returned["product_name"],
    } == {"customer_id": "foo", "product_name": "bar"}
