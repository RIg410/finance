use crate::dao::model::assets::{Asset, AssetType};
use crate::dao::model::currency::{Currency, CurrencyRate};
use crate::dao::model::operations::{AssetOperation, OperationType};
use crate::service::decimal::Decimal;
use color_eyre::eyre::Error;
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
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

    pub async fn get_asset_by_ticker(&self, ticker: String) -> Result<Option<Asset>, Error> {
        let assets = sqlx::query_as!(Asset, "SELECT * FROM asset WHERE ticker = ?", ticker)
            .fetch_optional(&self.pool)
            .await?;
        Ok(assets)
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

    pub async fn get_asset_types(&self, asset: &Asset) -> Result<Vec<AssetType>, Error> {
        let id = asset.id;
        let types = sqlx::query_as!(
            AssetType,
            "\
        SELECT id, name, description FROM asset_type \
        INNER JOIN asset_to_type ON asset_type.id = asset_to_type.type_id \
        WHERE asset_to_type.asset_id = ?",
            id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(types)
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

    pub async fn add_asset_type(&self, asset: &Asset, tp: &AssetType) -> Result<(), Error> {
        let asset_id = asset.id;
        let type_id = tp.id;
        sqlx::query!(
            "INSERT INTO asset_to_type (asset_id, type_id) VALUES (?, ?)",
            asset_id,
            type_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn remove_asset_type(&self, asset: &Asset, tp: &AssetType) -> Result<(), Error> {
        let asset_id = asset.id;
        let type_id = tp.id;
        sqlx::query!(
            "DELETE FROM asset_to_type WHERE asset_id = ? AND type_id = ?",
            asset_id,
            type_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn remove_asset(&self, asset: &Asset) -> Result<(), Error> {
        let id = asset.id;
        sqlx::query!("DELETE FROM asset WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_operation(
        &self,
        asset: &Asset,
        asset_rate: &CurrencyRate,
        operation_type: OperationType,
        operation_amount: Decimal,
    ) -> Result<(), Error> {
        let asset_id = asset.id;
        let operation_type: String = operation_type.into();
        let currency_rate = asset_rate.id;
        let operation_amount: i64 = operation_amount.into();
        sqlx::query!(
            "INSERT INTO asset_operations (asset_id, operation_type, operation_date, operation_amount, currency_rate) VALUES (?, ?, datetime('now'), ?, ?)",
            asset_id,
            operation_type,
            operation_amount,
            currency_rate
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_operations(&self, asset: &Asset) -> Result<Vec<AssetOperation>, Error> {
        let asset_id = asset.id;
        let operations = sqlx::query_as!(
            AssetOperation,
            "SELECT * FROM asset_operations WHERE asset_id = ?",
            asset_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(operations)
    }
}
