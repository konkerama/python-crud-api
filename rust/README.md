# Rust Crud

status: 

- basic api running

todo:

- implement crud
- connect to postgres
- tracing using opentelemetry
- testing

## mongo db
https://codevoweb.com/build-a-crud-api-with-rust-and-mongodb/

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
