#!/bin/bash

set -e

host="timescaledb"
user="liquibase"
password="PsWDgxZb"
database="postgres"

until PGPASSWORD=$password psql -h "$host" -U "$user" -d "$database" -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up - executing command"