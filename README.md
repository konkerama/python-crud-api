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
GET /pg/customer?customer_name="mark"
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


### Postges query
#### Traditional way
``` python
def pg_connection():
    if conn.closed():
        return conn
    conn = psycopg2.connect(database=POSTGRES_DB,
                            host=POSTGRES_URL,
                            user=POSTGRES_USER,
                            password=POSTGRES_PASSWORD,
                            port="5432")
    return conn

# query
cursor = conn.cursor()
command = f"SELECT * FROM customers WHERE customer_name='{customer_name}'"
cursor.execute(command)
cursor.close()
conn.commit()
result = cursor.fetchall()
logger.info(result)
return result

# post
cursor = conn.cursor()
postgres_insert_query = "INSERT INTO customers (customer_name) VALUES (%s)"
record_to_insert = (customer_name,)
cursor.execute(postgres_insert_query, record_to_insert)
conn.commit()
```

in the current implementation the python `flask_sqlalchemy` library is used as it is a more managed way to communicate with postgres from flask

flask_sqlalchemy urls:
- https://stackoverflow.com/questions/55523299/best-practices-for-persistent-database-connections-in-python-when-using-flask
- https://flask-sqlalchemy.palletsprojects.com/en/3.0.x/quickstart/#installation
- https://flask-sqlalchemy.palletsprojects.com/en/3.0.x/queries/
- https://python-adv-web-apps.readthedocs.io/en/latest/flask_db1.html
- https://stackabuse.com/using-sqlalchemy-with-flask-and-postgresql/
- https://stackoverflow.com/questions/42225127/how-to-do-a-select-query-using-flask-and-sqlalchemy
- https://flask-sqlalchemy.palletsprojects.com/en/2.x/queries/
- https://www.digitalocean.com/community/tutorials/how-to-use-flask-sqlalchemy-to-interact-with-databases-in-a-flask-application
- 