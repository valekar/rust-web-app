CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR NOT NULL,
    hashed_password VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE UNIQUE INDEX users_username on users(username);

CREATE TABLE auth_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users (id),
    token VARCHAR NOT NULL
);

CREATE UNIQUE INDEX auth_tokens_token on auth_tokens(token);