import os
import pytest
from main import app, db, CustomersModel
from unittest.mock import patch, MagicMock

@pytest.fixture
def client():
    app.config['TESTING'] = True
    app.config['ENV'] = 'dev'
    client = app.test_client()

    with app.app_context():
        db.create_all()

    yield client

    with app.app_context():
        db.drop_all()

@pytest.fixture
def mock_mongo_client():
    # Mock the MongoClient to avoid actual connections
    with patch("app.MongoClient") as mock_client:
        yield mock_client

@pytest.fixture
def mock_postgres_customer():
    # Mock the PostgreSQL database query to avoid actual connections
    with patch.object(CustomersModel, 'query') as mock_query:
        yield mock_query

def test_home_endpoint(client):
    response = client.get('/')
    assert response.status_code == 200
    data = response.json
    assert 'data' in data
    assert 'lambda-response' in data

def test_pg_customer_get(client, mock_postgres_customer, monkeypatch):
    # Mock the result of the PostgreSQL query
    mock_postgres_customer.filter_by.return_value.first.return_value = CustomersModel(customer_name='Test Customer', customer_id=1)

    # Set the required environment variables for PostgreSQL
    monkeypatch.setenv('POSTGRES_USER', 'your_pg_user')
    monkeypatch.setenv('POSTGRES_PASSWORD', 'your_pg_password')
    monkeypatch.setenv('POSTGRES_DB', 'your_pg_db')
    monkeypatch.setenv('POSTGRES_URL', 'your_pg_url')

    response = client.get('/pg/customer?customer_name=Test Customer')
    assert response.status_code == 200
    data = response.json
    assert 'customer_name' in data
    assert 'customer_id' in data
    assert data['customer_name'] == 'Test Customer'
    assert data['customer_id'] == 1

def test_pg_customer_post(client, mock_postgres_customer, monkeypatch):
    # Mock the database session add and commit methods
    mock_session = MagicMock()
    db.session.add.return_value = mock_session

    # Set the required environment variables for PostgreSQL
    monkeypatch.setenv('POSTGRES_USER', 'your_pg_user')
    monkeypatch.setenv('POSTGRES_PASSWORD', 'your_pg_password')
    monkeypatch.setenv('POSTGRES_DB', 'your_pg_db')
    monkeypatch.setenv('POSTGRES_URL', 'your_pg_url')

    response = client.post('/pg/customer', json={'customer_name': 'New Customer'})
    assert response.status_code == 200
    data = response.json
    assert 'customer' in data
    assert 'status' in data
    assert data['customer'] == 'New Customer'
    assert data['status'] == 'inserted'

def test_mongo_orders_get(client, mock_mongo_client, monkeypatch):
    # Mock the MongoDB collection find method
    mock_collection = MagicMock()
    mock_collection.find.return_value = [{'product_name': 'Product A', 'customer_id': 1}, {'product_name': 'Product B', 'customer_id': 2}]
    mock_db = MagicMock()
    mock_db.__getitem__.return_value = mock_collection
    mock_mongo_client.return_value = mock_db

    # Set the required environment variables for MongoDB
    monkeypatch.setenv('ME_CONFIG_MONGODB_ADMINUSERNAME', 'your_mongo_user')
    monkeypatch.setenv('ME_CONFIG_MONGODB_ADMINPASSWORD', 'your_mongo_password')
    monkeypatch.setenv('ME_CONFIG_MONGODB_SERVER', 'your_mongo_server')

    response = client.get('/mongo/orders?product_name=Product A')
    assert response.status_code == 200
    data = response.json
    assert len(data) == 2
    assert {'product_name': 'Product A', 'customer_id': 1} in data
    assert {'product_name': 'Product B', 'customer_id': 2} in data

def test_mongo_orders_post(client, mock_mongo_client, monkeypatch):
    # Mock the MongoDB collection insert_one method
    mock_collection = MagicMock()
    mock_db = MagicMock()
    mock_db.__getitem__.return_value = mock_collection
    mock_mongo_client.return_value = mock_db

    # Set the required environment variables for MongoDB
    monkeypatch.setenv('ME_CONFIG_MONGODB_ADMINUSERNAME', 'your_mongo_user')
    monkeypatch.setenv('ME_CONFIG_MONGODB_ADMINPASSWORD', 'your_mongo_password')
    monkeypatch.setenv('ME_CONFIG_MONGODB_SERVER', 'your_mongo_server')

    response = client.post('/mongo/orders', json={'customer_id': 1, 'product_name': 'Product C'})
    assert response.status_code == 200
    data = response.json
    assert 'customer_id' in data
    assert 'product_name' in data
    assert 'status' in data
    assert data['customer_id'] == 1
    assert data['product_name'] == 'Product C'
    assert data['status'] == 'inserted'
