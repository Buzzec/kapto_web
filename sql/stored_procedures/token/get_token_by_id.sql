DROP PROCEDURE IF EXISTS get_token_by_id;
CREATE PROCEDURE get_token_by_id(IN id_in BIGINT UNSIGNED)
BEGIN
    DECLARE retrieved_token DATETIME;

    SELECT expires
    INTO retrieved_token
    FROM token
    WHERE token.id = id_in
    LIMIT 1 OFFSET 0;

    IF NOT ISNULL(retrieved_token) THEN
        IF retrieved_token < NOW() THEN
            DELETE FROM token WHERE token.id = id_in;
        ELSE
            SELECT id,
                   user_id,
                   token,
                   expires
            FROM token
            WHERE token.id = id_in
            LIMIT 1 OFFSET 0;
        END IF;
    END IF;
END;
