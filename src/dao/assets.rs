use crate::dao::model::assets::Asset;
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
}
