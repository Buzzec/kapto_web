DROP PROCEDURE IF EXISTS delete_ruleset;
CREATE PROCEDURE delete_ruleset(IN id_in BIGINT UNSIGNED)
BEGIN
    DELETE FROM ruleset
    WHERE ruleset.id = id_in;
END;
