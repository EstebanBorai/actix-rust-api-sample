-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "rental" (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	building_type TEXT UNIQUE NOT NULL,
	price DECIMAL NOT NULL,
	year_of_construction SMALLINT NOT NULL,
	address TEXT NOT NULL,
	bedrooms SMALLINT NOT NULL,
	bathrooms SMALLINT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
	updated_at TIMESTAMP
);
