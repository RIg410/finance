use crate::dao::assets::AssetsDao;
use crate::dao::currency::CurrencyDao;
use color_eyre::eyre::Context;
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
}
