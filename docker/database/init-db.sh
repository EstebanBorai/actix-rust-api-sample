#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
		CREATE DATABASE rust_webapi;
		GRANT ALL PRIVILEGES ON DATABASE rust_webapi TO admin;
EOSQL
