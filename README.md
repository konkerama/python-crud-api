# python-crud-api

Sample python crud api using flask that integrates with mongodb and postgresql. Currently full crud is not implemented.
CICD creates and pushes a docker container that can also run in k8s.
Github actions also creates a pr to be used in gitops.


## Environment Information

### Install a new python package

``` bash
pipenv install ...
pipenv run pip freeze > requirements.txt
```

### Deploy App to K8s 
``` bash
skaffold dev --trigger=manual
```

### Application Testing

``` bash
URL=$(minikube service client -n orders --url)
# Postgres
# POST /pg/customer
# { customer_name="sdf" }
curl -X POST $URL/pg/customer -d '{"customer_name":"mark"}' -H "Content-Type: application/json"
# GET /pg/customer?customer_name="mark"
curl $URL/pg/customer?customer_name=mark

# Mongo
# POST /mongo/orders
# { customer_id="sdf", product_name="asd" }
curl -X POST $URL/mongo/orders -d '{"customer_id":"2", "product_name":"apple"}' -H "Content-Type: application/json"
# GET /mongo/orders?product_name="asdf"
curl $URL/mongo/orders?product_name=apple
```

### Testing Using docker compose

``` bash 
docker compose up --build 
```

## TODO:

- Implement full crud ability
- Implement tests for python application