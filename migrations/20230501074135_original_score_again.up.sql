-- Add up migration script here

CREATE OR REPLACE FUNCTION record_score(progress FLOAT, demon FLOAT, list_size FLOAT, requirement FLOAT) RETURNS FLOAT AS $record_score$
    SELECT CASE
        WHEN progress = 100 THEN
            list_size * EXP((1.0 - demon) * LN(1.0 / 30.0) / (-list_size + 1.0))  -- i wanted to do one of those bitwise things but it doesn't like floats
        WHEN progress < requirement THEN
            0.0
				WHEN list_size < demon THEN -- if sql messes up then this
						0.0
        ELSE
            list_size * EXP((1.0 - demon) * LN(1.0 / 30.0) / (-list_size + 1.0)) * (0.25 * (progress - requirement) / (100 - requirement) + 0.25)
    END;
$record_score$ LANGUAGE SQL IMMUTABLE;
