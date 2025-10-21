use surrealdb::{Surreal, engine::local::Db};

pub struct ProjectContext {
    pub db: Surreal<Db>,
}

impl ProjectContext {
    pub fn new(db: Surreal<Db>) -> Self {
        Self { db }
    }
}
