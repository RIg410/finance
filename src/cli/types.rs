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
                service.add_type(name, description).await?;
                println!("Type added");
                Ok(())
            }
            TypesCommand::Remove { id } => {
                service.remove_type(id).await?;
                println!("Type removed");
                Ok(())
            }
        }
    }
}
