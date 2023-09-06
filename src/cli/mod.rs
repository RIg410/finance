use clap::Parser;
use color_eyre::eyre::Error;

pub mod currency;
use currency::CurrencyCommand;
use finance::service::FinanceService;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub enum Cli {
    #[command(subcommand)]
    /// Currency commands
    Currency(CurrencyCommand),
}

impl Cli {
    pub async fn handle(&self, service: &FinanceService) -> Result<(), Error> {
        match self {
            Cli::Currency(currency) => currency.handle(service).await,
        }
    }
}
