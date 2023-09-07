use crate::dao::currency::CurrencyDao;
use crate::dao::model::currency::Currency;
use crate::service::decimal::Decimal;
use crate::view::currency::CurrencyShortInfo;
use color_eyre::eyre::Error;

pub struct CurrencyService {
    dao: CurrencyDao,
}

impl CurrencyService {
    pub fn new(dao: CurrencyDao) -> Self {
        Self { dao }
    }

    pub async fn base_currency(&self) -> Result<Currency, Error> {
        Ok(self.dao.get(1).await?)
    }

    pub async fn currency_info(&self, ticker: &String) -> Result<CurrencyShortInfo, Error> {
        let currency = self.dao.find_by_ticker(ticker).await?;
        let rate = self.dao.last_rate(currency.id).await?;
        Ok(CurrencyShortInfo {
            name: currency.name,
            ticker: currency.ticker,
            rate: rate.rate,
        })
    }

    pub async fn currency(&self, ticker: &String) -> Result<Currency, Error> {
        Ok(self.dao.find_by_ticker(ticker).await?)
    }

    pub async fn drop(&self, currency: &Currency) -> Result<(), Error> {
        self.dao.drop(currency).await?;
        Ok(())
    }

    pub async fn currency_info_list(&self) -> Result<Vec<CurrencyShortInfo>, Error> {
        let currencies = self.dao.list().await?;
        let mut result = Vec::new();
        for currency in currencies {
            let rate = self.dao.last_rate(currency.id).await?;
            result.push(CurrencyShortInfo {
                name: currency.name,
                ticker: currency.ticker,
                rate: rate.rate,
            });
        }
        Ok(result)
    }

    pub async fn create(&self, name: String, ticker: String) -> Result<Currency, Error> {
        Ok(self.dao.create(name, ticker).await?)
    }

    pub async fn add_rate(&self, currency: &Currency, rate: Decimal) -> Result<(), Error> {
        self.dao.add_rate(currency.id, rate).await?;
        Ok(())
    }
}
