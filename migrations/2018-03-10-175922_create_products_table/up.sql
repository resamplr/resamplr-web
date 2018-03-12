-- up.sql
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    price_cents INT NOT NULL DEFAULT 0,
    published BOOLEAN NOT NULL DEFAULT 'f'
);