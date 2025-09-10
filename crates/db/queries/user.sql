--! user 
SELECT 
    id, email, username, created_at
FROM 
    users
WHERE
    id = :id;
    
--! insert_user
INSERT INTO 
    users (email, username, password_hash)
VALUES(:email, :username, :password_hash) 
RETURNING id, email, username, created_at;

--! user_by_openid_sub 
SELECT 
    id, email, username, created_at
FROM 
    users
WHERE
    openid_sub = :openid_sub;

--! get_by_email 
SELECT 
    id, email, username, password_hash, created_at
FROM 
    users
WHERE
    email = :email;

--! get_by_email_auth
SELECT 
    id, email, username, password_hash, created_at
FROM 
    users
WHERE
    email = :email;

--! count_users
SELECT
    count(id)
FROM
    users;

