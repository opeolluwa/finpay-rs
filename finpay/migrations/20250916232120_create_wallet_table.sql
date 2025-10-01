-- Add migration script here

CREATE TABLE wallets
(
    identifier          UUID        NOT NULL PRIMARY KEY,
    name                VARCHAR,
    user_identifier     UUID        NOT NULL REFERENCES users (identifier) ON DELETE CASCADE ON UPDATE CASCADE,
    currency_identifier UUID        NOT NULL REFERENCES countries (identifier) ON DELETE CASCADE ON UPDATE CASCADE,
    created_date        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    update_at           TIMESTAMPTZ NOT NULL

)