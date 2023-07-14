#!/bin/bash

URL=$(minikube service client -n orders --url)
curl -X POST $URL/pg/customer -d '{"customer_name":"mark"}' -H "Content-Type: application/json"
curl $URL/pg/customer?customer_name=mark
curl -X POST $URL/mongo/orders -d '{"customer_id":"2", "product_name":"apple"}' -H "Content-Type: application/json"
curl $URL/mongo/orders?product_name=apple
