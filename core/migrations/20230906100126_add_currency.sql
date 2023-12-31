-- Add migration script here
CREATE TABLE IF NOT EXISTS currency
(
    id     INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    ticker TEXT UNIQUE                       NOT NULL,
    name   TEXT                              NOT NULL
);

CREATE TABLE IF NOT EXISTS currency_rate
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    currency_id INTEGER                           NOT NULL,
    rate        INTEGER                           NOT NULL,
    date        TIMESTAMP                         NOT NULL,
    FOREIGN KEY (currency_id) REFERENCES currency (id) ON DELETE CASCADE ON UPDATE CASCADE
);

INSERT INTO currency (ticker, name)
VALUES ('usd', 'United States Dollar');
INSERT INTO currency_rate (currency_id, rate, date)
VALUES (1, 100, datetime('now'));