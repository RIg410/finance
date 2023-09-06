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
    pub async fn handle(&self, service: &FinanceService) -> Result<(), Error> {
        match self {
            CurrencyCommand::List => {
                let currencies = service.currency.currency_info_list().await?;
                println!(
                    "Currencies: name: ticker rate to the {}",
                    service.currency.base_currency().await?.name
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
                let rate = Decimal::from_str(rate_to_base)?;
                let currency = service
                    .currency
                    .create(name.clone(), ticker.clone())
                    .await?;
                service.currency.add_rate(&currency, rate).await?;
            }
            CurrencyCommand::Remove { ticker } => {
                
            }
            CurrencyCommand::AddRate { ticker, rate_to_base } => {
                let rate = Decimal::from_str(rate_to_base)?;

                
            }
        }
        Ok(())
    }
}
