use crate::service::decimal::Decimal;
use chrono::NaiveDateTime;

pub struct Currency {
    pub id: i64,
    pub name: String,
    pub ticker: String,
}

#[derive(sqlx::FromRow)]
pub struct CurrencyRate {
    pub id: i64,
    pub currency_id: i64,
    pub rate: Decimal,
    pub date: NaiveDateTime,
}
