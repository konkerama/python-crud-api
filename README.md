# k8s-application

## TODO:
- ivestigate proper flows in managing container image versions on gitops
- Write github actions to implement CI:
  - test python Application
  - automatically open pr when there is a new version??
    - first use this to clone the project repo locally in ghactions: https://github.com/actions/checkout
    - then use this to update yaml https://github.com/fjogeleit/yaml-update-action
    - and then using the gh api to create a pr

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


## Luiquibase
https://www.liquibase.com/blog/using-liquibase-in-kubernetes
Create config map
``` bash
kubectl create configmap liquibase-changelog --from-file=liquibase/changelog.xml -n orders -o yaml --dry-run=client  | kubectl apply -f -
```

urls:
- https://www.liquibase.org/get-started/quickstart
- https://docs.liquibase.com/concepts/changelogs/attributes/home.html

liquibase fixing forward is better than rollback
https://www.liquibase.com/blog/roll-back-database-fix-forward

rolling update: 
- https://kubernetes.io/docs/concepts/workloads/controllers/deployment/
- https://www.bluematador.com/blog/kubernetes-deployments-rolling-update-configuration
- https://phoenixnap.com/kb/kubernetes-rolling-update
- https://kubernetes.io/docs/concepts/workloads/controllers/deployment/

## K8s pull from private docker hub
https://kubernetes.io/docs/tasks/configure-pod-container/pull-image-private-registry/
``` bash
kubectl create secret generic regcred \
    --from-file=.dockerconfigjson=$HOME/.docker/config.json \
    --type=kubernetes.io/dockerconfigjson \
    -n orders

```