# Rust Crud api

Sample implementation of a rust web server using axum that integrates with posgresql and mongo db databases

## API Description

Exposes 2 separate crud implementations on `/api/pg` & `/api/mongo` for the PostgreSQL and MongoDB implementations respectively.
You can list/create/update/delete customers by targeting the `/api/pg` path with attributes of `customer_name` & `customer_surname` in the request body.
The same can be applied for MongoDB on the `/api/mongo` path.

## Deployment

The application is packaged on a container for easy reuse on multiple environments. Liquibase is used for managing the PostgreSQL schema.

## How to use

### Prerquisites

Install rust on your system.

### Test

``` bash
cargo install sqlx-cli
./test.sh
```

### Run API locally

``` bash
docker compose up --build --force-recreate -V
```

`docker compose` performs the following steps:

- builds the container
- creates the mongodb and postgesql containers
- creates and runs the liquibase containers that configures the postgresql schema
- creates the pgadming and mongoexpress containers for easy debugging of the databases.

You can target the api using the following example `curl` commands:

``` bash
# health check
curl http://localhost:8000/api/healthchecker -s | jq

# POST create customer 
curl -X POST http://localhost:8000/api/pg -d '{"customer_name": "john","customer_surname": "doe"}' -H "Content-Type: application/json" -s | jq

# GET customer (replace <id> with your customer id)
curl http://localhost:8000/api/pg/<id> -s | jq

# LIST customers
curl http://localhost:8000/api/pg -s | jq

# DELETE customer (replace <id> with your customer id)
curl -X DELETE http://localhost:8000/api/pg/<id> -s | jq

# PATCH customer (replace <id> with your customer id)
curl -X PATCH http://localhost:8000/api/pg/<id> -d '{"customer_name": "mark","customer_surname": "green"}' -H "Content-Type: application/json" -s | jq

# POST order
curl -X POST http://localhost:8000/api/mongo -d '{"customer_name":"mark", "product_name":"apple"}' -H "Content-Type: application/json" -s | jq

# LIST orders
curl http://localhost:8000/api/mongo -s | jq

# GET order (replace <id> with your order id)
curl http://localhost:8000/api/mongo/<id> -s | jq

# PATCH order (replace <id> with your order id)
curl -X PATCH http://localhost:8000/api/mongo/<id> -d '{"customer_name":"paul", "product_name":"banana"}' -H "Content-Type: application/json" -s | jq

# DELETE order (replace <id> with your order id)
curl -X DELETE http://localhost:8000/api/mongo/<id> -s | jq

```

## Todo

- implement tracing using opentelemetry
- modify dependency loading with dotenv file rather than env vars

## Commands

``` bash
# prepare
./prepare

# build and run
docker compose up --build --force-recreate -V

# optionally to clean up everything
docker compose down

# health check
curl http://localhost:8000/api/healthchecker -s | jq


# create customer
curl -X POST http://localhost:8000/api/pg -d '{"customer_name": "paul","customer_surname": "doe"}' -H "Content-Type: application/json" -s | jq

# get customer
curl http://localhost:8000/api/pg/id -s | jq

# list 
curl http://localhost:8000/api/pg -s | jq

# delete 
curl -X DELETE http://localhost:8000/api/pg/paul -s | jq

# update
curl -X PATCH http://localhost:8000/api/pg/paul -d '{"customer_name": "mark","customer_surname": "green"}' -H "Content-Type: application/json" -s | jq

# create order
curl -X POST http://localhost:8000/api/mongo -d '{"customer_name":"mark", "product_name":"apple"}' -H "Content-Type: application/json" -s | jq

# list orders
curl http://localhost:8000/api/mongo -s | jq

# get order
curl http://localhost:8000/api/mongo/id -s | jq

# update order
curl -X PATCH http://localhost:8000/api/mongo/id -d '{"customer_name":"paul", "product_name":"banana"}' -H "Content-Type: application/json" -s | jq

# delete order
curl -X DELETE http://localhost:8000/api/mongo/id -s | newasdadsjq

```

https://codevoweb.com/build-a-simple-api-in-rust/

## mongo db

https://codevoweb.com/build-a-crud-api-with-rust-and-mongodb/
https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/#retrieve-data-from-a-collection
https://github.com/zupzup/rust-web-mongodb-example/blob/main/src/db.rs

create
curl -X POST http://localhost:8000/api/notes -d '{"id": "123","title": "name","content": "String","category": "Category"}' -H "Content-Type: application/json" -s | jq

list
curl http://localhost:8000/api/notes/ -s | jq

get
curl http://localhost:8000/api/notes/64ca59ece626c213c8d393c7 -s | jq

delete
curl -X DELETE http://localhost:8000/api/notes/64ca59ece626c213c8d393c7 -s | jq

patch
curl -X PATCH http://localhost:8000/api/notes/64ca59ece626c213c8d393c7~ -d '{"title": "name","content": "String","category": "Category"}' -H "Content-Type: application/json"

## postgres

proposed solution for calling postgres from Rust
https://github.com/launchbadge/sqlx
https://stackoverflow.com/questions/71202762/rust-warpsqlx-service-idiomatic-way-of-passing-dbpool-from-main-to-handlers

other resources:
https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html
https://tms-dev-blog.com/postgresql-database-with-rust-how-to/
https://blog.logrocket.com/async-crud-web-service-rust-warp/
https://github.com/zupzup/warp-postgres-example/blob/main/src/db.rs
https://www.reddit.com/r/rust/comments/goafhk/right_approach_to_use_postgres_with_warp/
https://blog.logrocket.com/async-crud-web-service-rust-warp/
https://stackoverflow.com/questions/61945533/how-do-i-use-tokio-postgres-with-warp

curl -X POST http://localhost:8000/api/pg -d '{"customer_name": "paul","customer_surname": "doe"}' -H "Content-Type: application/json" -s | jq

## axum

- https://github.com/davidpdrsn/realworld-axum-sqlx/tree/main
- https://blog.logrocket.com/rust-axum-error-handling/
- https://github.com/tokio-rs/axum/blob/main/examples/global-404-handler/src/main.rs
- https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs
- https://github.com/tokio-rs/axum/blob/main/examples/error-handling-and-dependency-injection/src/main.rs
- https://carlosmv.hashnode.dev/getting-started-with-axum-rust
- https://github.com/carlosm27/blog/blob/main/axum_crud_api/src/errors.rs
- https://github.com/carlosm27/blog/blob/main/axum_crud_api/src/controllers/task.rs
- https://github.com/wpcodevo/simple-api-rust-axum/tree/master
- https://github.com/tokio-rs/axum
