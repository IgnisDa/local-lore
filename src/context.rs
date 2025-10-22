use sea_orm::DatabaseConnection;

pub struct LocalLoreContext {
    pub db: DatabaseConnection,
}

impl LocalLoreContext {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
