use crate::dao::assets::AssetsDao;
use crate::dao::model::assets::{Asset, AssetType};
use crate::dao::model::currency::Currency;
use color_eyre::eyre::Error;

pub struct AssetsService {
    dao: AssetsDao,
}

impl AssetsService {
    pub fn new(dao: AssetsDao) -> Self {
        Self { dao }
    }

    pub async fn find_assets_with_currency(
        &self,
        currency: &Currency,
    ) -> Result<Vec<Asset>, Error> {
        self.dao.find_assets_with_currency(currency).await
    }

    pub async fn find_assets_with_type(&self, asset_type: &AssetType) -> Result<Vec<Asset>, Error> {
        self.dao.find_assets_with_type(asset_type).await
    }

    pub async fn get_types(&self) -> Result<Vec<AssetType>, Error> {
        self.dao.get_types().await
    }

    pub async fn add_type(&self, name: String, description: String) -> Result<AssetType, Error> {
        self.dao.add_type(name, description).await
    }

    pub async fn get_type_by_name(&self, name: String) -> Result<Option<AssetType>, Error> {
        self.dao.get_type_by_name(name).await
    }

    pub async fn get_asset_by_ticker(&self, ticker: String) -> Result<Option<Asset>, Error> {
        self.dao.get_asset_by_ticker(ticker).await
    }

    pub async fn get_type(&self, id: i64) -> Result<Option<AssetType>, Error> {
        self.dao.get_type(id).await
    }

    pub async fn remove_type(&self, tp: AssetType) -> Result<(), Error> {
        if !self.find_assets_with_type(&tp).await?.is_empty() {
            return Err(Error::msg("Type has assets"));
        }
        self.dao.remove_type(&tp).await
    }

    pub async fn get_assets(&self) -> Result<Vec<Asset>, Error> {
        self.dao.get_assets().await
    }

    pub async fn get_asset_types(&self, asset: &Asset) -> Result<Vec<AssetType>, Error> {
        self.dao.get_asset_types(asset).await
    }

    pub async fn add_asset(
        &self,
        ticker: String,
        name: String,
        description: String,
        currency: &Currency,
    ) -> Result<Asset, Error> {
        self.dao
            .add_asset(ticker, name, description, currency)
            .await
    }

    pub async fn add_asset_type(&self, asset: &Asset, tp: &AssetType) -> Result<(), Error> {
        self.dao.add_asset_type(asset, tp).await
    }

    pub async fn remove_asset_type(&self, asset: &Asset, tp: &AssetType) -> Result<(), Error> {
        self.dao.remove_asset_type(asset, tp).await
    }
}
