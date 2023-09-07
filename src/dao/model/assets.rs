pub struct AssetType {
    pub id: i64,
    pub name: String,
    pub description: String,
}

pub struct AssetToType {
    pub asset_id: i64,
    pub type_id: String,
}

pub struct Asset {
    pub id: i64,
    pub name: String,
    pub ticker: String,
    pub description: String,
    pub currency: i64,
}
