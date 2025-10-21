use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

pub static DEPENDENCY_TABLE: &str = "dependency";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectLanguage {
    Rust,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub id: RecordId,
    pub language: ProjectLanguage,
    pub name: String,
    pub version: String,
    pub indexed_at: DateTime<Utc>,
    pub first_seen_at: DateTime<Utc>,
}

impl Dependency {
    pub fn new(name: String, version: String, language: ProjectLanguage) -> Self {
        let now = Utc::now();
        let language_str = match language {
            ProjectLanguage::Rust => "rust",
        };
        Self {
            id: (
                DEPENDENCY_TABLE,
                format!("{}:{}:{}", language_str, name, version),
            )
                .into(),
            language,
            name,
            version,
            indexed_at: now,
            first_seen_at: now,
        }
    }
}
