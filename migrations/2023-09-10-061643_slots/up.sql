-- Your SQL goes here
CREATE TABLE slots (
    id VARCHAR(255) PRIMARY KEY,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    attendants INT NOT NULL,
    gym_id VARCHAR(255) REFERENCES Gym(id) NOT NULL,
    activity_id VARCHAR(255) REFERENCES Activities(id) NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL
);