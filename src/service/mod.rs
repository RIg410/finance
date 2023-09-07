use crate::dao::assets::AssetsDao;
use crate::dao::currency::CurrencyDao;
use crate::dao::model::assets::Asset;
use crate::dao::model::currency::Currency;
use crate::service::decimal::Decimal;
use crate::view::assets::AssetShortInfo;
use crate::view::currency::CurrencyShortInfo;
use crate::view::types::TypeView;
use color_eyre::eyre::{Context, Error};
use sqlx::{Pool, Sqlite};

pub mod assets;
pub mod currency;
pub mod decimal;

pub struct FinanceService {
    pub currency: currency::CurrencyService,
    pub assets: assets::AssetsService,
}

impl FinanceService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            currency: currency::CurrencyService::new(CurrencyDao::new(pool.clone())),
            assets: assets::AssetsService::new(AssetsDao::new(pool)),
        }
    }

    pub async fn currency_info_list(&self) -> Result<Vec<CurrencyShortInfo>, Error> {
        self.currency.currency_info_list().await
    }

    pub async fn base_currency(&self) -> Result<Currency, Error> {
        self.currency.base_currency().await
    }

    pub async fn create_currency(
        &self,
        name: String,
        ticker: String,
        rate: Decimal,
    ) -> Result<Currency, Error> {
        let currency = self.currency.create(name.clone(), ticker.clone()).await?;
        self.currency.add_rate(&currency, rate).await?;
        Ok(currency)
    }

    pub async fn remove_currency(&self, ticker: &String) -> Result<(), Error> {
        let currency = self
            .currency
            .currency(ticker)
            .await?
            .ok_or(Error::msg("Currency not found"))?;
        let assets = self.assets.find_assets_with_currency(&currency).await?;
        if assets.len() > 0 {
            return Err(Error::msg("Currency has assets"));
        }
        self.currency.drop(&currency).await?;
        Ok(())
    }

    pub async fn add_currency_rate(&self, ticker: String, rate: Decimal) -> Result<(), Error> {
        let currency = self
            .currency
            .currency(&ticker)
            .await?
            .ok_or(Error::msg("Currency not found"))?;
        self.currency.add_rate(&currency, rate).await?;
        Ok(())
    }

    pub async fn get_types(&self) -> Result<Vec<TypeView>, Error> {
        let types = self.assets.get_types().await?;
        Ok(types
            .into_iter()
            .map(|t| TypeView {
                id: t.id,
                name: t.name,
                description: t.description,
            })
            .collect())
    }

    pub async fn add_type(&self, name: String, description: String) -> Result<(), Error> {
        self.assets.add_type(name, description).await?;
        Ok(())
    }

    pub async fn remove_type(&self, id: String) -> Result<(), Error> {
        let tp = self
            .assets
            .get_type_by_name(id)
            .await?
            .ok_or(Error::msg("Type not found"))?;
        self.assets.remove_type(tp).await?;
        Ok(())
    }

    pub async fn get_assets(&self) -> Result<Vec<AssetShortInfo>, Error> {
        let assets = self.assets.get_assets().await?;
        Ok(assets
            .into_iter()
            .map(|a| AssetShortInfo { ticker: a.ticker })
            .collect())
    }

    pub async fn add_asset(
        &self,
        ticker: String,
        name: Option<String>,
        description: Option<String>,
        currency: String,
    ) -> Result<Asset, Error> {
        let currency = self
            .currency
            .currency(&currency)
            .await?
            .ok_or(Error::msg("Currency not found"))?;
        let name = name.unwrap_or(ticker.clone());
        let description = description.unwrap_or_default();
        let asset = self
            .assets
            .add_asset(ticker.clone(), name, description, &currency)
            .await?;
        Ok(asset)
    }
}
