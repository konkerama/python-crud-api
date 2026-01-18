# python-crud-api

Sample python crud api using flask that integrates with mongodb and postgresql. Currently full crud is not implemented.
CICD creates and pushes a docker container that can also run in k8s.
Github actions also creates a pr to be used in gitops.

Local Development
``` bash
# Spin up dependency containers
docker compose -f docker-compose-local.yml up
# Run as a module so `app.*` imports work correctly
uv run -m app.main

docker compose --build -f docker-compose.yml up

# to run locally do:
docker compose up --build

# run tests
uv run -m pytest -q
```

## Telemetry (OpenTelemetry)

This service supports OpenTelemetry tracing via auto-instrumentation (FastAPI + requests + SQLAlchemy + PyMongo).

- Disable completely (default): set `ENABLE_TELEMETRY=False` and nothing is configured/instrumented.
- Enable: set `ENABLE_TELEMETRY=True` plus an OTLP endpoint (examples below).

Common environment variables:

- `ENABLE_TELEMETRY=True|False`
- `OTEL_SERVICE_NAME=python-crud-api`
- `OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317` (OTLP/gRPC)
- `OTEL_TRACES_SAMPLER=always_on|always_off|traceidratio`
- `OTEL_TRACES_SAMPLER_ARG=0.1` (only for `traceidratio`)

Run application as container locally:
``` bash 
docker compose up --build
```

build container image
``` bash
docker build -t konkerama/go-crud-api:latest . && docker push konkerama/go-crud-api:latest
```





## Environment Information

### Install a new python package

``` bash
uv add <package>
uv lock
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