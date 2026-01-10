import orders
from pytest_mock_resources import create_mongo_fixture


mongo = create_mongo_fixture()


def test_insert_into_orders_collection(mongo):
    output = orders.post_order(mongo, "foo", "bar")
    assert output["status"] == "inserted"

    collection = mongo["orders"]
    returned = collection.find_one()

    assert {
        "customer_id": returned["customer_id"],
        "product_name": returned["product_name"],
    } == {"customer_id": "foo", "product_name": "bar"}
