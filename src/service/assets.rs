use crate::dao::assets::AssetsDao;

pub struct AssetsService {
    dao: AssetsDao,
}

impl AssetsService {
    pub fn new(dao: AssetsDao) -> Self {
        Self { dao }
    }
}
