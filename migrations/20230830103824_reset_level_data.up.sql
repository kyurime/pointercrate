-- warning! this is a destructive migration.
-- there wasn't really a better option. unless i wanted to write parsing in sql
-- (i didn't)

DROP TABLE gj_level_data;
CREATE TABLE gj_level_data (
    level_id bigint PRIMARY KEY REFERENCES gj_level(level_id) NOT NULL,
    level_password integer,
		level_length integer NOT NULL,
		level_objects_count integer NOT NULL
);

-- force a refresh of all level data
DELETE FROM gj_level_data_meta;
