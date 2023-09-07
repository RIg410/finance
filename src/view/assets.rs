use std::fmt::Display;

pub struct AssetShortInfo {
    pub ticker: String,
}

impl Display for AssetShortInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ticker)
    }
}
