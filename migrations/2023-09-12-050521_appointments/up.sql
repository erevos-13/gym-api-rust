-- Your SQL goes here
CREATE TABLE appointments (
    slot_id VARCHAR(255) REFERENCES Slots(id) NOT NULL,
    gym_id VARCHAR(255) REFERENCES Gym(id) NOT NULL,
    user_id VARCHAR(255) REFERENCES Users(id) NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL,
    PRIMARY KEY (slot_id, gym_id, user_id)
);