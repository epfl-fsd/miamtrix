-- Your SQL goes here
CREATE TABLE crons (
    id SERIAL PRIMARY KEY,
    room VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    cron_expression VARCHAR NOT NULL,
    command VARCHAR NOT NULL,
    job_id VARCHAR NOT NULL
);

ALTER TABLE crons ADD CONSTRAINT unique_room_name UNIQUE (room, name);
