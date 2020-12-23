DROP PROCEDURE IF EXISTS get_user_by_email;
CREATE PROCEDURE get_user_by_email(IN email_in VARCHAR(128))
BEGIN
    SELECT user.id,
           user.username,
           user.email,
           user.password_hash,
           user.is_admin
    FROM user
    WHERE user.email = email_in
    LIMIT 1 OFFSET 0;
END;
