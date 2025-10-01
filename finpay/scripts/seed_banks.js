import countriesIdentifier from "./seed/country_identifier.json" with {type: "json"}
import {faker} from '@faker-js/faker';
import {v4 as uuidv4} from "uuid";
import fs from "node:fs"

let inserts = [];

countriesIdentifier.identifiers.forEach((b) => {
    const sql = `(
    '${uuidv4()}',
    '${faker.company.name().replace(/'/g, "''")} Bank',
    '${b.replace(/'/g, "''")}'
    )`

    inserts.push(sql)
})


const fullSQL = `
-- Create table
CREATE TABLE IF NOT EXISTS banks (
  identifier UUID PRIMARY KEY,
  bank_name VARCHAR NOT NULL,
  country_identifier UUID NOT NULL REFERENCES countries(identifier) ON UPDATE CASCADE ON DELETE CASCADE,
  created_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

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
CREATE TRIGGER update_banks_updated_at
    BEFORE UPDATE
    ON banks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
    
    
-- Insert seed data 
INSERT INTO banks(
identifier, 
bank_name,
country_identifier ) VALUES 
${inserts.join(",\n")}
;
`

// Write to file
fs.writeFileSync("sql/seed_banks.sql", fullSQL, "utf8");

console.log("✅ Seeder generated: seed_banks.sql");
