# Rust Crud

status:

- basic api running

todo:

- tracing using opentelemetry
- testing

axum todo:

- mongo list/put add id on the response
- try implementing dependencies using dotenv as it is a best practice according to k8s

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
