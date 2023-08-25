-- Your SQL goes here
CREATE TABLE password_users (
    id VARCHAR(255) PRIMARY KEY, 
    user_id VARCHAR(255) REFERENCES Users(id) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL
)