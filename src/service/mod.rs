use crate::dao::assets::AssetsDao;
use crate::dao::currency::CurrencyDao;
use crate::dao::model::currency::Currency;
use crate::service::decimal::Decimal;
use crate::view::currency::CurrencyShortInfo;
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
        let currency = self.currency.currency(ticker).await?;
        let assets = self.assets.find_assets_with_currency(&currency).await?;
        if assets.len() > 0 {
            return Err(Error::msg("Currency has assets"));
        }
        self.currency.drop(&currency).await?;
        Ok(())
    }

    pub async fn add_currency_rate(&self, ticker: String, rate: Decimal) -> Result<(), Error> {
        let currency = self.currency.currency(&ticker).await?;
        self.currency.add_rate(&currency, rate).await?;
        Ok(())
    }
}
