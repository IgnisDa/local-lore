use sea_orm::DatabaseConnection;

pub struct ProjectContext {
    pub db: DatabaseConnection,
}

impl ProjectContext {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
