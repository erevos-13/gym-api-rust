-- Your SQL goes 
CREATE TABLE Users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    -- age serial NOT NULL,
    email TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL

);
