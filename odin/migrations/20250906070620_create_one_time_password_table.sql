-- Add migration script here
CREATE TABLE IF NOT EXISTS one_time_password
(
    identifier      UUID PRIMARY KEY NOT NULL,
    token           CHAR(6)          NOT NULL,
    user_identifier UUID REFERENCES users (identifier) ON DELETE CASCADE ON UPDATE CASCADE,
    created_date    TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    update_at       TIMESTAMPTZ      NOT NULL
)