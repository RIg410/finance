sqlx_prepare:
	cargo sqlx prepare --database-url "sqlite:db/finance.db"

sqlx_make_db:
	cd db && rm -rf finance.db
	cargo sqlx db create --database-url "sqlite:db/finance.db"
	cargo sqlx migrate run --database-url "sqlite:db/finance.db"

 install:
	cargo install --path .

init_types:
	cargo run types add 'crowd' "business loans"
	cargo run types add 'real_estate' "real estate"
	cargo run types add 'crypto' "crypto currency"
	cargo run types add 'stocks' "stocks"
	cargo run types add 'bonds' "bonds"
	cargo run types add 'cash' "cash"
	cargo run types add 'etf' "etf"


