use color_eyre::eyre::Error;

pub enum OperationType {
    UpdatePrice,
    Buy,
    Sell,
    Dividend,
    DividendReinvest,
}

impl TryFrom<u32> for OperationType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OperationType::UpdatePrice),
            1 => Ok(OperationType::Buy),
            2 => Ok(OperationType::Sell),
            3 => Ok(OperationType::Dividend),
            4 => Ok(OperationType::DividendReinvest),
            _ => Err(Error::msg("Invalid operation type")),
        }
    }
}

impl From<OperationType> for u32 {
    fn from(value: OperationType) -> Self {
        match value {
            OperationType::UpdatePrice => 0,
            OperationType::Buy => 1,
            OperationType::Sell => 2,
            OperationType::Dividend => 3,
            OperationType::DividendReinvest => 4,
        }
    }
}
