-- migrate:up
CREATE TABLE users (
   id SERIAL PRIMARY KEY, 
   email VARCHAR NOT NULL UNIQUE, 
   openid_sub VARCHAR UNIQUE,
   created_at TIMESTAMP NOT NULL DEFAULT NOW(),
   updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO users (email) VALUES ('test@mail.com');

-- migrate:down
DROP TABLE users;

