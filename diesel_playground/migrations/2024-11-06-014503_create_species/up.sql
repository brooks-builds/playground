-- Your SQL goes here
CREATE TABLE species (
    id SERIAL PRIMARY KEY,
    name VARCHAR(10) NOT NULL UNIQUE
);
