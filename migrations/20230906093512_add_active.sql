-- Add migration script here
CREATE TABLE IF NOT EXISTS asset_type
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name        TEXT                UNIQUE NOT NULL,
    description TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS asset
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name        TEXT                UNIQUE NOT NULL,
    ticker      TEXT                UNIQUE NOT NULL,
    description TEXT                NOT NULL,
    type_id     INTEGER             NOT NULL,
    FOREIGN KEY (type_id) REFERENCES asset_type (id)
);

CREATE TABLE IF NOT EXISTS asset_operations (
    id               INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    asset_id         INTEGER         NOT NULL,
    operation_type   TEXT            NOT NULL,
    operation_date   TIMESTAMP       NOT NULL,
    operation_amount INTEGER         NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES asset (id)
);
