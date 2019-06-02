CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    login VARCHAR(15) NOT NULL,
    password TEXT NOT NULL,
    nickname VARCHAR(15) NOT NULL,
    role SMALLINT NOT NULL DEFAULT 0,
    ticket VARCHAR(32) DEFAULT NULL,
    last_server SMALLINT DEFAULT NULL
);

CREATE TABLE game_servers (
    id SMALLINT PRIMARY KEY,
    host TEXT NOT NULL,
    port SMALLINT NOT NULL
);

CREATE TABLE characters (
    id SERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES accounts,
    server_id SMALLINT NOT NULL REFERENCES game_servers
);
