use clap::Parser;
use color_eyre::eyre::Error;
use sqlx::SqlitePool;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub enum Cli {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("DATABASE_URL", "sqlite:db/finance.db");
    env::set_var("RUST_LOG", "info");
    color_eyre::install()?;
    env_logger::init();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(())
}
