use crate::dao::model::currency::{Currency, CurrencyRate};
use crate::service::decimal::Decimal;
use color_eyre::eyre::Error;
use log::debug;
use sqlx::{Pool, Sqlite};

pub struct CurrencyDao {
    pool: Pool<Sqlite>,
}

impl CurrencyDao {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Currency>, Error> {
        let currency = sqlx::query_as!(Currency, "SELECT * FROM currency")
            .fetch_all(&self.pool)
            .await?;
        Ok(currency)
    }

    pub async fn get(&self, id: i64) -> Result<Option<Currency>, Error> {
        let currency = sqlx::query_as!(Currency, "SELECT * FROM currency WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(currency)
    }

    pub async fn find_by_ticker(&self, ticker: &String) -> Result<Option<Currency>, Error> {
        let currency = sqlx::query_as!(Currency, "SELECT * FROM currency WHERE ticker = ?", ticker)
            .fetch_optional(&self.pool)
            .await?;
        Ok(currency)
    }

    pub async fn create(&self, name: String, ticker: String) -> Result<Currency, Error> {
        let mut conn = self.pool.acquire().await?;
        let id = sqlx::query!(
            "INSERT INTO currency (name, ticker) VALUES (?, ?)",
            name,
            ticker
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();
        debug!("currency created: {}", id);
        Ok(Currency { id, name, ticker })
    }

    pub async fn drop(&self, currency: &Currency) -> Result<(), Error> {
        debug!("drop currency: {:?}", currency.id);
        let id = currency.id;
        sqlx::query!("DELETE FROM currency WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_rate(&self, currency_id: i64, rate: Decimal) -> Result<i64, Error> {
        let rate: i64 = rate.into();
        let id = sqlx::query!(
            "INSERT INTO currency_rate (currency_id, rate, date) VALUES (?, ?, datetime('now'))",
            currency_id,
            rate
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn get_rates(&self, currency_id: i64) -> Result<Vec<CurrencyRate>, Error> {
        let rate = sqlx::query_as!(
            CurrencyRate,
            "SELECT * FROM currency_rate WHERE currency_id = ?",
            currency_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rate)
    }

    pub async fn last_rate(&self, currency_id: i64) -> Result<CurrencyRate, Error> {
        let rate = sqlx::query_as!(
            CurrencyRate,
            "SELECT * FROM currency_rate WHERE currency_id = ? ORDER BY date DESC LIMIT 1",
            currency_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rate)
    }

    pub async fn get_rates_paging(
        &self,
        currency_id: i64,
        page: i64,
        size: i64,
    ) -> Result<Vec<CurrencyRate>, Error> {
        let offset = page * size;
        let rate = sqlx::query_as!(
            CurrencyRate,
            "SELECT * FROM currency_rate WHERE currency_id = ? LIMIT ? OFFSET ?",
            currency_id,
            size,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rate)
    }

    pub async fn drop_rate(&self, id: i64) -> Result<(), Error> {
        sqlx::query!("DELETE FROM currency_rate WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
