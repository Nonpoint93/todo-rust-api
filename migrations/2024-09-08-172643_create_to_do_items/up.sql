-- Your SQL goes here
CREATE TABLE to_do_table (
id SERIAL PRIMARY KEY,
txt_title VARCHAR NOT NULL,
txt_status VARCHAR NOT NULL,
dat_date timestamp NOT NULL DEFAULT NOW()
)