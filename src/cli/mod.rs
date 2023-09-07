use clap::Parser;
use color_eyre::eyre::Error;

pub mod assets;
pub mod currency;
pub mod types;

pub mod ops;

use currency::CurrencyCommand;
use finance::service::FinanceService;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub enum Cli {
    #[command(subcommand)]
    /// Currency commands
    Currency(CurrencyCommand),
    #[command(subcommand)]
    /// Types commands
    Types(types::TypesCommand),

    #[command(subcommand)]
    /// Assets commands
    Assets(assets::AssetsCommand),

    #[command(subcommand)]
    /// Financial operations
    Ops(ops::OpsCommand),
}

impl Cli {
    pub async fn handle(self, service: &FinanceService) -> Result<(), Error> {
        match self {
            Cli::Currency(currency) => currency.handle(service).await,
            Cli::Types(types) => types.handle(service).await,
            Cli::Assets(assets) => assets.handle(service).await,
            Cli::Ops(ops) => ops.handle(service).await,
        }
    }
}
