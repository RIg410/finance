use std::fmt::Display;

pub struct AssetShortInfo {
    pub ticker: String,
    pub tags: Vec<String>,
}

impl Display for AssetShortInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:?}", self.ticker, self.tags)
    }
}
