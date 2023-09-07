use clap::Parser;
use color_eyre::eyre::Error;
use finance::service::FinanceService;

#[derive(Parser, Debug)]
pub enum TypesCommand {
    /// List all asset types
    List,
    /// Add new asset type
    Add { name: String, description: String },
    /// Remove asset type
    Remove { id: String },
    /// Apply asset type to asset
    Apply { asset: String, asset_type: String },
    /// Remove asset type from asset
    RemoveFrom { asset: String, asset_type: String },
}

impl TypesCommand {
    pub async fn handle(self, service: &FinanceService) -> Result<(), Error> {
        match self {
            TypesCommand::List => {
                let types = service.get_types().await?;
                println!("Types: ");
                for t in types {
                    println!("{}", t);
                }
                Ok(())
            }
            TypesCommand::Add { name, description } => {
                service
                    .add_type(name.to_ascii_lowercase(), description)
                    .await?;
                println!("Type added");
                Ok(())
            }
            TypesCommand::Remove { id } => {
                service.remove_type(id.to_ascii_lowercase()).await?;
                println!("Type removed");
                Ok(())
            }
            TypesCommand::Apply { asset, asset_type } => {
                service
                    .add_asset_type(asset.to_ascii_lowercase(), asset_type)
                    .await?;
                println!("Type applied");
                Ok(())
            }
            TypesCommand::RemoveFrom { asset, asset_type } => {
                service
                    .remove_asset_type(asset.to_ascii_lowercase(), asset_type.to_ascii_lowercase())
                    .await?;
                println!("Type removed");
                Ok(())
            }
        }
    }
}
