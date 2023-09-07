use clap::Parser;
use color_eyre::eyre::Error;
use finance::service::FinanceService;

#[derive(Parser, Debug)]
pub enum AssetsCommand {
    /// List all assets
    List,
    /// Add new asset
    Add {
        ticker: String,
        #[clap(short, long)]
        name: Option<String>,
        #[clap(short, long)]
        description: Option<String>,
        currency: String,
    },
}

impl AssetsCommand {
    pub async fn handle(self, service: &FinanceService) -> Result<(), Error> {
        match self {
            AssetsCommand::List => {
                let assets = service.get_assets().await?;
                println!("Assets: ");
                for a in assets {
                    println!("{}", a);
                }
                Ok(())
            }
            AssetsCommand::Add {
                ticker,
                name,
                description,
                currency,
            } => {
                service
                    .add_asset(ticker, name, description, currency)
                    .await?;
                println!("Asset added");
                Ok(())
            }
        }
    }
}
