#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
		CREATE DATABASE actix_rentals;
		GRANT ALL PRIVILEGES ON DATABASE actix_rentals TO admin;
EOSQL
