use crate::dao::model::assets::{Asset, AssetType};
use crate::dao::model::currency::Currency;
use color_eyre::eyre::Error;
use sqlx::{Pool, Sqlite};

pub struct AssetsDao {
    pool: Pool<Sqlite>,
}

impl AssetsDao {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn find_assets_with_currency(
        &self,
        currency: &Currency,
    ) -> Result<Vec<Asset>, Error> {
        let assets = sqlx::query_as!(Asset, "SELECT * FROM asset WHERE currency = ?", currency.id)
            .fetch_all(&self.pool)
            .await?;
        Ok(assets)
    }

    pub async fn find_assets_with_type(&self, asset_type: &AssetType) -> Result<Vec<Asset>, Error> {
        let id = asset_type.id;
        let assets = sqlx::query_as!(
            Asset,
            "\
        SELECT id, name, ticker, description, currency FROM asset \
        INNER JOIN asset_to_type ON asset.id = asset_to_type.asset_id \
        WHERE asset_to_type.type_id = ?",
            id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(assets)
    }

    pub async fn get_types(&self) -> Result<Vec<AssetType>, Error> {
        let types = sqlx::query_as!(AssetType, "SELECT * FROM asset_type")
            .fetch_all(&self.pool)
            .await?;
        Ok(types)
    }

    pub async fn add_type(&self, name: String, description: String) -> Result<AssetType, Error> {
        let id = sqlx::query!(
            "INSERT INTO asset_type (name, description) VALUES (?, ?)",
            name,
            description
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(AssetType {
            id,
            name,
            description,
        })
    }

    pub async fn get_type(&self, id: i64) -> Result<Option<AssetType>, Error> {
        let types = sqlx::query_as!(AssetType, "SELECT * FROM asset_type WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(types)
    }

    pub async fn get_type_by_name(&self, name: String) -> Result<Option<AssetType>, Error> {
        let types = sqlx::query_as!(AssetType, "SELECT * FROM asset_type WHERE name = ?", name)
            .fetch_optional(&self.pool)
            .await?;
        Ok(types)
    }

    pub async fn remove_type(&self, tp: &AssetType) -> Result<(), Error> {
        let id = tp.id;
        sqlx::query!("DELETE FROM asset_type WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_assets(&self) -> Result<Vec<Asset>, Error> {
        let assets = sqlx::query_as!(Asset, "SELECT * FROM asset")
            .fetch_all(&self.pool)
            .await?;
        Ok(assets)
    }

    pub async fn add_asset(
        &self,
        ticker: String,
        name: String,
        description: String,
        currency: &Currency,
    ) -> Result<Asset, Error> {
        let currency = currency.id;
        let id = sqlx::query!(
            "INSERT INTO asset (ticker, name, description, currency) VALUES (?, ?, ?, ?)",
            ticker,
            name,
            description,
            currency
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(Asset {
            id,
            ticker,
            name,
            description,
            currency,
        })
    }
}
