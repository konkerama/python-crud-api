#!/bin/bash
docker compose -f liquibase/sqlx-docker-compose.yaml up -d

cargo sqlx prepare --database-url "postgresql://postgres:postgres@localhost:5432/postgres"

docker compose -f liquibase/sqlx-docker-compose.yaml down
