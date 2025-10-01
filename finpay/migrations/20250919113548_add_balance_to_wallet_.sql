-- Add migration script here
-- up to 10^28 with 2 decimal places
ALTER TABLE wallets
    ADD COLUMN balance NUMERIC(20, 6) NOT NULL DEFAULT 0;
