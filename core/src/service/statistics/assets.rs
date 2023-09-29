use crate::dao::model::assets::Asset;
use crate::dao::model::operations::{AssetOperation, OperationType};
use crate::service::decimal::Decimal;
use chrono::NaiveDateTime;
use std::fmt::Display;

pub struct AssetStatistic {
    pub asset: Asset,
    snapshots: Vec<AssetSnapshot>,
}

impl AssetStatistic {
    pub fn new(asset: Asset, operation: Vec<AssetOperation>) -> Self {
        let mut snapshots = Vec::<AssetSnapshot>::new();
        for op in operation {
            if let Some(snapshot) = snapshots.last() {
                snapshots.push(snapshot.make_next(op));
            } else {
                snapshots.push(AssetSnapshot::default().make_next(op));
            }
        }

        Self { asset, snapshots }
    }

    pub fn get_last_snapshot(&self) -> Option<&AssetSnapshot> {
        self.snapshots.last()
    }

    pub fn get_latest_snapshot(&self, date: NaiveDateTime) -> Option<&AssetSnapshot> {
        self.snapshots.iter().rev().find(|s| s.date <= date)
    }
}

#[derive(Debug, Default, Clone)]
pub struct AssetSnapshot {
    /// Snapshot date
    pub date: NaiveDateTime,
    /// Asset price
    pub price: Decimal,
    /// quantity of asset
    pub quantity: Decimal,
    /// Total received from asset
    pub paid: Decimal,
    /// Total invested in asset
    pub invested: Decimal,
    /// Total withdrawn from asset
    pub withdrawn: Decimal,
}

impl AssetSnapshot {
    fn make_next(&self, operation: AssetOperation) -> Self {
        let mut next = AssetSnapshot::default();
        next.date = operation.operation_date;
        match operation.operation_type {
            OperationType::UpdatePrice => {
                next.price = operation.operation_amount;
                next.quantity = self.quantity;
                next.paid = self.paid;
                next.invested = self.invested;
                next.withdrawn = self.withdrawn;
            }
            OperationType::Buy => {
                next.price = self.price;
                next.quantity = self.quantity + operation.operation_amount;
                next.paid = self.paid;
                next.invested = self.invested + (operation.operation_amount * self.price);
                next.withdrawn = self.withdrawn;
            }
            OperationType::Sell => {
                next.price = self.price;
                next.quantity = self.quantity - operation.operation_amount;
                next.paid = self.paid;
                next.invested = self.invested - (operation.operation_amount * self.price);
                next.withdrawn = self.withdrawn + (operation.operation_amount * self.price);
            }
            OperationType::Dividend => {
                next.price = self.price;
                next.quantity = self.quantity;
                next.paid = self.paid + operation.operation_amount;
                next.invested = self.invested;
                next.withdrawn = self.withdrawn;
            }
        }
        next
    }
}

impl Display for AssetSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "date: {}", self.date)?;
        writeln!(f, "price: {}", self.price)?;
        writeln!(f, "quantity: {}", self.quantity)?;
        writeln!(f, "paid: {}", self.paid)?;
        writeln!(f, "invested: {}", self.invested)?;
        writeln!(f, "withdrawn: {}", self.withdrawn)?;
        Ok(())
    }
}
