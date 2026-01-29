mod error;
mod store;
use crate::model::store::{Db, new_db_pool};

pub use self::error::{Error, Result};

pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        Ok(ModelManager { db })
    }
}
