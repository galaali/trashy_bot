-- Your SQL goes here
CREATE TABLE banks (
    id SERIAL8 PRIMARY KEY,
    user_id INT8 NOT NULL,
    amount INT8 NOT NULL
)