use clap::Parser;
use color_eyre::eyre::Error;
use finance::dao::model::operations::OperationType;
use finance::service::decimal::Decimal;
use finance::service::FinanceService;

#[derive(Parser, Debug)]
pub enum OpsCommand {
    Price {
        asset: String,
        price: String,
    },
    Buy {
        asset: String,
        amount: String,
        price: Option<String>,
    },
    Sell {
        asset: String,
        amount: String,
        price: Option<String>,
    },
    Dividend {
        asset: String,
        amount: String,
        reinvest: bool,
    },
}

impl OpsCommand {
    fn asset(&self) -> &String {
        match self {
            OpsCommand::Price { asset, price: _ } => asset,
            OpsCommand::Buy {
                asset,
                amount: _,
                price: _,
            } => asset,
            OpsCommand::Sell {
                asset,
                amount: _,
                price: _,
            } => asset,
            OpsCommand::Dividend {
                asset,
                amount: _,
                reinvest: _,
            } => asset,
        }
    }
    fn new_price(&self) -> Result<Option<Decimal>, Error> {
        Ok(match self {
            OpsCommand::Price { asset: _, price } => Some(price.parse::<Decimal>()?),
            OpsCommand::Buy {
                asset: _,
                amount: _,
                price,
            } => {
                if let Some(price) = price {
                    Some(price.parse::<Decimal>()?)
                } else {
                    None
                }
            }
            OpsCommand::Sell {
                asset: _,
                amount: _,
                price,
            } => {
                if let Some(price) = price {
                    Some(price.parse::<Decimal>()?)
                } else {
                    None
                }
            }
            OpsCommand::Dividend {
                asset: _,
                amount: _,
                reinvest: _,
            } => None,
        })
    }

    pub async fn handle(self, service: &FinanceService) -> Result<(), Error> {
        if let Some(price) = self.new_price()? {
            service
                .add_operation(
                    self.asset().to_ascii_lowercase(),
                    price,
                    OperationType::UpdatePrice,
                )
                .await?;
            println!("Price updated");
        }

        match self {
            OpsCommand::Price { .. } => {
                //no-op
                Ok(())
            }
            OpsCommand::Buy {
                asset,
                amount,
                price: _,
            } => {
                service
                    .add_operation(
                        asset.to_ascii_lowercase(),
                        amount.parse::<Decimal>()?,
                        OperationType::Buy,
                    )
                    .await?;
                println!("Bought: {}", amount);
                Ok(())
            }
            OpsCommand::Sell {
                asset,
                amount,
                price: _,
            } => {
                service
                    .add_operation(
                        asset.to_ascii_lowercase(),
                        amount.parse::<Decimal>()?,
                        OperationType::Sell,
                    )
                    .await?;
                println!("Sold: {}", amount);
                Ok(())
            }
            OpsCommand::Dividend {
                asset,
                amount,
                reinvest,
            } => {
                service
                    .add_operation(
                        asset.to_ascii_lowercase(),
                        amount.parse::<Decimal>()?,
                        if reinvest {
                            OperationType::DividendReinvest
                        } else {
                            OperationType::Dividend
                        },
                    )
                    .await?;
                println!("Dividend: {}", amount);
                Ok(())
            }
        }
    }
}
