use clap::Parser;
use color_eyre::eyre::Error;
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
    pub async fn handle(self, service: &FinanceService) -> Result<(), Error> {
        match self {
            OpsCommand::Price { asset, price } => {
                let price = service.add_price(asset.to_ascii_lowercase(), price).await?;
                println!("Price added: {}", price);
                Ok(())
            }
            OpsCommand::Buy {
                asset,
                amount,
                price,
            } => {
                let price = if let Some(price) = price {
                    Some(service.add_price(asset.to_ascii_lowercase(), price).await?)
                } else {
                    None
                };
                let amount = service
                    .buy(asset.to_ascii_lowercase(), amount, price)
                    .await?;
                println!("Bought: {}", amount);
                Ok(())
            }
            OpsCommand::Sell {
                asset,
                amount,
                price,
            } => {
                let price = if let Some(price) = price {
                    Some(service.add_price(asset.to_ascii_lowercase(), price).await?)
                } else {
                    None
                };
                let amount = service
                    .sell(asset.to_ascii_lowercase(), amount, price)
                    .await?;
                println!("Sold: {}", amount);
                Ok(())
            }
            OpsCommand::Dividend {
                asset,
                amount,
                reinvest,
            } => {
                let amount = service
                    .dividend(asset.to_ascii_lowercase(), amount, reinvest)
                    .await?;
                println!("Dividend: {}", amount);
                Ok(())
            }
        }
    }
}
