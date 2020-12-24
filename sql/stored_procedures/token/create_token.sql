DROP PROCEDURE IF EXISTS create_token;
CREATE PROCEDURE create_token(IN user_id_in BIGINT UNSIGNED, IN token_in BINARY(128))
BEGIN
    SELECT LAST_INSERT_ID();

    INSERT INTO token(user_id, token, expires)
        VALUE (user_id_in, token_in, TIMESTAMPADD(DAY, 7, NOW()));

    SELECT id,
           user_id,
           token,
           expires
    FROM token
    WHERE token.id = LAST_INSERT_ID();
END;
