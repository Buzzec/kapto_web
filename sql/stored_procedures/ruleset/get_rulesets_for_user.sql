DROP PROCEDURE IF EXISTS get_rulesets_for_user;
CREATE PROCEDURE get_rulesets_for_user(IN user_id_in BIGINT UNSIGNED,
                                       IN limit_in BIGINT UNSIGNED,
                                       IN offset_in BIGINT UNSIGNED)
BEGIN
    SELECT ruleset.id,
           ruleset.user_id,
           ruleset.version,
           ruleset.ruleset
    FROM ruleset
    WHERE ruleset.user_id = user_id_in
    LIMIT limit_in OFFSET offset_in;
END;
