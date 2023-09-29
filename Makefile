sqlx_prepare:
	cargo sqlx prepare --database-url "sqlite:db/finance.db"

sqlx_make_db:
	cd db && rm -rf finance.db
	cargo sqlx db create --database-url "sqlite:db/finance.db"
	cargo sqlx migrate run --database-url "sqlite:db/finance.db"

 run:
	cargo run --bin service_runner