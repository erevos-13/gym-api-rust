-- Your SQL goes here
CREATE TABLE gym (
    id VARCHAR(255) PRIMARY KEY,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    postal_code INT NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL
)
