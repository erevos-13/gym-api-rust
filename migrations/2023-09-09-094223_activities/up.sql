-- Your SQL goes here
CREATE TABLE activities (
    id VARCHAR(255) PRIMARY KEY,
    name TEXT NOT NULL,
    gym_id VARCHAR(255) UNIQUE NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL
)