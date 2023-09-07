use crate::dao::assets::AssetsDao;
use crate::dao::model::assets::Asset;
use color_eyre::eyre::Error;
use crate::dao::model::currency::Currency;

pub struct AssetsService {
    dao: AssetsDao,
}

impl AssetsService {
    pub fn new(dao: AssetsDao) -> Self {
        Self { dao }
    }

    pub async fn find_assets_with_currency(&self, currency: &Currency) -> Result<Vec<Asset>, Error> {
        self.dao.find_assets_with_currency(currency).await
    }
}
