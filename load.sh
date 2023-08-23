#!/bin/bash

URL=$(minikube service production-client -n orders --url)
while true
do
    curl -X POST $URL/pg/customer -d '{"customer_name":"mark"}' -H "Content-Type: application/json"
    curl $URL/pg/customer?customer_name=mark
done

