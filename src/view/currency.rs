use crate::service::decimal::Decimal;
use std::fmt::Display;

pub struct CurrencyShortInfo {
    pub name: String,
    pub ticker: String,
    pub rate: Decimal,
}

impl Display for CurrencyShortInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name == self.ticker {
            write!(f, "{} = {}", self.name, self.rate)
        } else {
            write!(f, "{}({}) = {}", self.name, self.ticker, self.rate)
        }
    }
}
