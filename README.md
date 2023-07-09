# k8s-application

```
pipenv install ...
pipenv run pip freeze > requirements.txt
```

skaffold dev --trigger=manual


python local testing 
``` bash 
docker compose up --build 
```

python status:
- done basic connection to mongodb and added some items

todo:
- retrieve items from mongo db and send them back via api
- read a bit in general about how mongo db works
  - https://www.guru99.com/what-is-mongodb.html
  - https://www.mongodb.com/languages/python
  - https://www.mongodb.com/basics/database-index
- do the same for postgresdb



## API spec
### Postgres
```
POST /pg/customer
{
    customer_name="sdf"
}
curl -X POST localhost:8080/pg/customer -d '{"customer_name":"mark"}' -H "Content-Type: application/json"
```
```
GET /pg/customer?customer_name="sdf"
curl localhost:8080/pg/customer?customer_name=mark
```

### Mongo
```
GET /mongo/orders?product_name="asdf"
curl localhost:8080/mongo/orders?product_name=banana
```
```
POST /mongo/orders
{
    customer_id="sdf",
    product_name="asd"
}
curl -X POST localhost:8080/mongo/orders -d '{"customer_id":"2", "product_name":"apple"}' -H "Content-Type: application/json"
```

## todo:
handle gp db connections properly using SQLAlchemy
https://stackoverflow.com/questions/55523299/best-practices-for-persistent-database-connections-in-python-when-using-flask
https://python-adv-web-apps.readthedocs.io/en/latest/flask_db1.html
https://flask-sqlalchemy.palletsprojects.com/en/3.0.x/quickstart/#installation
https://docs.sqlalchemy.org/en/20/intro.html



urls:
- https://www.dataquest.io/blog/tutorial-connect-install-and-query-postgresql-in-python/
- https://pynative.com/python-postgresql-insert-update-delete-table-data-to-perform-crud-operations/#h-python-postgresql-insert-into-database-table
- https://www.geeksforgeeks.org/postgresql-serial/
- https://stackoverflow.com/questions/21524482/psycopg2-typeerror-not-all-arguments-converted-during-string-formatting


update dict:
- https://www.freecodecamp.org/news/add-to-dict-in-python/

mongo jsonify (we use the bjon library that is built in the psycopg2-binary):
- https://stackoverflow.com/questions/16586180/typeerror-objectid-is-not-json-serializable