-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    emailAddress VARCHAR(128) NOT NULL,
    passwordHash VARCHAR NOT NULL,
    passwordSalt VARCHAR NOT NULL,
    fullName VARCHAR(64) NOT NULL,
    phonenNumber VARCHAR(16) NOT NULL
);

CREATE TABLE user_tokens (
    id SERIAL PRIMARY Key NOT NULL,
    userid SERIAL NOT NULL,
    token VARCHAR(128) NOT NULL,
    createdAt TIMESTAMP NOT NULL,
    expiresAt TIMESTAMP NOT NULL
);