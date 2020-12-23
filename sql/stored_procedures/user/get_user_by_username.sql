DROP PROCEDURE IF EXISTS get_user_by_username;
CREATE PROCEDURE get_user_by_username(IN username_in VARCHAR(64))
BEGIN
    SELECT user.id,
           user.username,
           user.email,
           user.password_hash,
           user.is_admin
    FROM user
    WHERE user.username = username_in
    LIMIT 1 OFFSET 0;
END;
