import fs from "fs";
import {v4 as uuidv4} from "uuid";
import country from "./seed/country.json" with {type: "json"};

let inserts = [];

country.forEach((c) => {
    const sql = `(
    '${uuidv4()}',
    '${c.code.replace(/'/g, "''")}',
    '${c.name.replace(/'/g, "''")}', 
    '${c.country.replace(/'/g, "''")}',
    '${c.flag}'
  )`;
    inserts.push(sql);
});

const fullSQL = `
-- Create table
CREATE TABLE IF NOT EXISTS countries (
  identifier UUID PRIMARY KEY,
  currency_code VARCHAR(10) NOT NULL,
  currency VARCHAR(100) NOT NULL,
  country VARCHAR(100) NOT NULL,
  flag TEXT
);

-- Insert seed data
INSERT INTO countries (
  identifier,
  currency_code,
  currency,
  country,
  flag
) VALUES
${inserts.join(",\n")}
;
`;

// Write to file
fs.writeFileSync("sql/seed_countries.sql", fullSQL, "utf8");

console.log("✅ Seeder generated: seed_countries.sql");
