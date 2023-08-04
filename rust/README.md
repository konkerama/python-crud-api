# Rust Crud

status: 

- basic api running

todo:

- implement crud
- connect to postgres
- tracing using opentelemetry
- testing

https://codevoweb.com/build-a-simple-api-in-rust/

## mongo db
https://codevoweb.com/build-a-crud-api-with-rust-and-mongodb/
https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/#retrieve-data-from-a-collection
https://github.com/zupzup/rust-web-mongodb-example/blob/main/src/db.rs

create
curl -X POST http://localhost:8000/api/notes -d '{"id": "123","title": "asdads","content": "String","category": "Category"}' -H "Content-Type: application/json" -s | jq

list
curl http://localhost:8000/api/notes/ -s | jq

get
curl http://localhost:8000/api/notes/64ca59ece626c213c8d393c7 -s | jq

delete
curl -X DELETE http://localhost:8000/api/notes/64ca59ece626c213c8d393c7 -s | jq

patch
curl -X PATCH http://localhost:8000/api/notes/64ca59ece626c213c8d393c7~ -d '{"title": "newasdads","content": "String","category": "Category"}' -H "Content-Type: application/json"

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