DROP PROCEDURE IF EXISTS insert_ruleset;
CREATE PROCEDURE insert_ruleset(IN user_id_in BIGINT UNSIGNED, IN version_in INT UNSIGNED, IN ruleset_in TEXT)
BEGIN
    INSERT INTO ruleset (user_id, version, ruleset) VALUE (user_id_in, version_in, ruleset_in);

    SELECT ruleset.id,
           ruleset.user_id,
           ruleset.version,
           ruleset.ruleset
    FROM ruleset
    WHERE ruleset.id = LAST_INSERT_ID();
END;
