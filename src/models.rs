use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

pub static DEPENDENCY_TABLE: &str = "dependency";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub id: RecordId,
    pub name: String,
    pub version: String,
    pub indexed_at: DateTime<Utc>,
    pub first_seen_at: DateTime<Utc>,
}

impl Dependency {
    pub fn new(name: String, version: String) -> Self {
        let now = Utc::now();
        Self {
            id: (DEPENDENCY_TABLE, format!("{}:{}", name, version)).into(),
            name,
            version,
            indexed_at: now,
            first_seen_at: now,
        }
    }
}
