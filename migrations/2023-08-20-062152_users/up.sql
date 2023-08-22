-- Your SQL goes 
CREATE TABLE Users (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    username VARCHAR(255) NOT NULL,
    age INT NOT NULL,
    email VARCHAR(255) NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL

);
