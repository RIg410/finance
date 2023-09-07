use crate::service::decimal::Decimal;
use chrono::NaiveDateTime;

pub struct AssetOperation {
    pub id: i64,
    pub asset_id: i64,
    pub operation_type: OperationType,
    pub operation_date: NaiveDateTime,
    pub operation_amount: Decimal,
}

pub enum OperationType {
    UpdatePrice,
    Buy,
    Sell,
    Dividend,
    DividendReinvest,
}

impl From<String> for OperationType {
    fn from(value: String) -> Self {
        From::from(value.as_str())
    }
}

impl From<&str> for OperationType {
    fn from(value: &str) -> Self {
        match value {
            "UpdatePrice" => OperationType::UpdatePrice,
            "Buy" => OperationType::Buy,
            "Sell" => OperationType::Sell,
            "Dividend" => OperationType::Dividend,
            "DividendReinvest" => OperationType::DividendReinvest,
            _ => panic!("Unknown operation type"),
        }
    }
}

impl From<OperationType> for String {
    fn from(val: OperationType) -> Self {
        match val {
            OperationType::UpdatePrice => "UpdatePrice".to_string(),
            OperationType::Buy => "Buy".to_string(),
            OperationType::Sell => "Sell".to_string(),
            OperationType::Dividend => "Dividend".to_string(),
            OperationType::DividendReinvest => "DividendReinvest".to_string(),
        }
    }
}
