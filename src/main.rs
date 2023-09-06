use clap::Parser;
use color_eyre::eyre::Error;
use finance::service::FinanceService;
use sqlx::SqlitePool;
use std::env;

pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = cli::Cli::parse();
    env::set_var("DATABASE_URL", "sqlite:db/finance.db");
    env::set_var("RUST_LOG", "info");
    color_eyre::install()?;
    env_logger::init();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&pool).await?;
    let service = FinanceService::new(pool);
    cli.handle(&service).await?;
    Ok(())
}
