-- Drop old column if it exists
ALTER TABLE wallets
    DROP COLUMN IF EXISTS update_at;

-- Add new updated_at column
ALTER TABLE wallets
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Trigger function to auto-update the column
CREATE
OR REPLACE
FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Attach trigger
CREATE TRIGGER update_wallets_updated_at
    BEFORE UPDATE
    ON wallets
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
