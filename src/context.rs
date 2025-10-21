use std::sync::Arc;

use surrealdb::{Surreal, engine::local::Db};

pub struct ProjectContext {
    pub db: Arc<Surreal<Db>>,
}

impl ProjectContext {
    pub fn new(db: Arc<Surreal<Db>>) -> Self {
        Self { db }
    }
}
