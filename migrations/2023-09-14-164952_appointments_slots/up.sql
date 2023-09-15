-- Your SQL goes here
CREATE TABLE appointments_slots (
    slot_id VARCHAR(255) REFERENCES slots(id) UNIQUE not null ,
    appointment_id VARCHAR(255) REFERENCES appointments(id) UNIQUE not null ,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL,
    PRIMARY KEY (appointment_id, slot_id)
)
