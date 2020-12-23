DROP PROCEDURE IF EXISTS get_user_by_id;
CREATE PROCEDURE get_user_by_id(IN id_in BIGINT UNSIGNED)
BEGIN
    SELECT user.id,
           user.username,
           user.email,
           user.password_hash,
           user.is_admin
    FROM user
    WHERE user.id = id_in
    LIMIT 1 OFFSET 0;
END;
