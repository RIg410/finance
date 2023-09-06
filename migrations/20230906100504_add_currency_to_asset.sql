-- Add migration script here
ALTER TABLE asset ADD currency INTEGER NOT NULL REFERENCES currency(id);
ALTER TABLE asset_operations ADD currency_rate INTEGER NOT NULL REFERENCES currency_rate(id);
