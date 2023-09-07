use clap::Parser;
use color_eyre::eyre::Error;
use finance::service::decimal::Decimal;
use finance::service::FinanceService;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub enum CurrencyCommand {
    /// List all currencies
    List,
    /// Add a new currency
    Add {
        /// Currency name
        name: String,
        /// Currency ticker
        ticker: String,
        /// Currency rate to the base currency
        rate_to_base: String,
    },
    /// Remove a currency
    Remove {
        /// Currency ticker
        ticker: String,
    },
    /// Add a new rate
    AddRate {
        /// Currency ticker
        ticker: String,
        /// Currency rate to the base currency
        rate_to_base: String,
    },
}

impl CurrencyCommand {
    pub async fn handle(self, service: &FinanceService) -> Result<(), Error> {
        match self {
            CurrencyCommand::List => {
                let currencies = service.currency_info_list().await?;
                println!(
                    "Currencies: name: ticker rate to the {}",
                    service.base_currency().await?.name
                );
                for currency in currencies {
                    println!("{}", currency);
                }
            }
            CurrencyCommand::Add {
                name,
                ticker,
                rate_to_base,
            } => {
                let currency = service
                    .create_currency(
                        name,
                        ticker.to_ascii_lowercase(),
                        Decimal::from_str(&rate_to_base)?,
                    )
                    .await?;
                println!("Currency {} added", currency.name);
            }
            CurrencyCommand::Remove { ticker } => {
                service
                    .remove_currency(&ticker.to_ascii_lowercase())
                    .await?;
                println!("Currency {} removed", ticker);
            }
            CurrencyCommand::AddRate {
                ticker,
                rate_to_base,
            } => {
                service
                    .add_currency_rate(
                        ticker.to_ascii_lowercase(),
                        Decimal::from_str(&rate_to_base)?,
                    )
                    .await?;
                println!("Currency rate added");
            }
        }
        Ok(())
    }
}
