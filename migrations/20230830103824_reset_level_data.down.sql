-- Add down migration script here

DROP TABLE gj_level_data;
CREATE TABLE gj_level_data (
    level_id bigint PRIMARY KEY REFERENCES gj_level(level_id) NOT NULL,
    level_data bytea NOT NULL,
    level_password integer,
    time_since_upload text NOT NULL,
    time_since_update text NOT NULL,
    index_36 text
);

DELETE FROM gj_level_data_meta;
