import fs from "fs";
import { v4 as uuidv4 } from "uuid";
import country from "./country.json" assert { type: "json" };

let inserts = [];

country.forEach((c) => {
    const sql = `(
    '${uuidv4()}',
    '${c.name.replace(/'/g, "''")}',
    '${c.code.replace(/'/g, "''")}',
    '${c.name.replace(/'/g, "''")}', 
    '${c.country.replace(/'/g, "''")}',
    '${c.flag}',
    NOW(),
    NOW()
  )`;
    inserts.push(sql);
});

const fullSQL = `
-- Drop table if exists (for reseeding)
DROP TABLE IF EXISTS countries;

-- Create table
CREATE TABLE countries (
  identifier UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  currency_code VARCHAR(10) NOT NULL,
  currency VARCHAR(100) NOT NULL,
  country VARCHAR(100) NOT NULL,
  flag TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Insert seed data
INSERT INTO countries (
  identifier,
  name,
  currency_code,
  currency,
  country,
  flag,
  created_at,
  updated_at
) VALUES
${inserts.join(",\n")}
;
`;

// Write to file
fs.writeFileSync("seed_countries.sql", fullSQL, "utf8");

console.log("✅ Seeder generated: seed_countries.sql");
