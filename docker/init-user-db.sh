#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
		CREATE DATABASE rust_actix_postgres;
		GRANT ALL PRIVILEGES ON DATABASE rust_actix_postgres TO admin;
EOSQL
