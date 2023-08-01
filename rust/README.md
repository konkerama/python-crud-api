# Rust Crud

status: 

- basic api running

todo:

- implement crud
- connect to mongodb
- connect to postgres
- logging?
- tracing using opentelemetry
- testing


curl -X POST http://localhost:8000/api/todos -d '{"title": "Build a Simple CRUD API in Rust","content": "This tutorial is the best"}' -H "Content-Type: application/json"


curl -X PATCH http://localhost:8000/api/todos/1f483e31-c1a6-4721-a967-779b5ff0b7b5 -d '{"title": "The new title of the Todo item","completed": true}' -H "Content-Type: application/json"


curl http://localhost:8000/api/todos/1f483e31-c1a6-4721-a967-779b5ff0b7b5 -s | jq

curl http://localhost:8000/api/todos?page=1&limit=10 -s | jq
