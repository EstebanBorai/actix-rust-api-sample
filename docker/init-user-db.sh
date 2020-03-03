#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
		CREATE DATABASE actix_rentals;
		GRANT ALL PRIVILEGES ON DATABASE actix_rentals TO admin;

		CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

		CREATE TABLE "user" (
			id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
			email TEXT UNIQUE NOT NULL,
			password TEXT NOT NULL,
			created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
			updated_at TIMESTAMP
		);
EOSQL
