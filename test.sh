#!/bin/bash

URL="localhost:8080"

# URL=$(minikube service dev-python-app -n orders --url)
curl -X POST $URL/pg/customer -d '{"customer_name":"mark"}' -H "Content-Type: application/json" -s | jq -c
curl $URL/pg/customer?customer_name=mark -s | jq -c
curl -X DELETE $URL/pg/customer -s | jq -c
curl -X POST $URL/mongo/orders -d '{"customer_id":"2", "product_name":"apple"}' -H "Content-Type: application/json" -s | jq -c
curl $URL/mongo/orders?product_name=apple -s | jq -c
curl -X DELETE $URL/mongo/orders -s | jq -c 