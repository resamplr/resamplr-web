-- up.sql
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    published BOOLEAN NOT NULL DEFAULT 'f'
);