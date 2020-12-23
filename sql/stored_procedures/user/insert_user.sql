DROP PROCEDURE IF EXISTS insert_user;
CREATE PROCEDURE insert_user(IN username_in VARCHAR(64), IN email_in VARCHAR(128), IN password_hash_in TEXT)
BEGIN
    INSERT INTO user(username, email, password_hash)
        VALUE (username_in, email_in, password_hash_in);

    SELECT user.id,
           user.username,
           user.email,
           user.password_hash,
           user.is_admin
    FROM user
    WHERE user.id = LAST_INSERT_ID();
END;
