use clap::Parser;
use color_eyre::eyre::Error;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub enum Cli {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("DATABASE_URL", "sqlite:todos.db");
    color_eyre::install()?;
    env_logger::init();

    Ok(())
}
