DROP PROCEDURE IF EXISTS get_ruleset_by_id;
CREATE PROCEDURE get_ruleset_by_id(IN id_in BIGINT UNSIGNED)
BEGIN
    SELECT ruleset.id,
           ruleset.user_id,
           ruleset.version,
           ruleset.ruleset
    FROM ruleset
    WHERE ruleset.id = id_in
    LIMIT 1 OFFSET 0;
END;
