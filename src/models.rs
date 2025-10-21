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
    pub name: String,
    pub version: String,
    pub language: ProjectLanguage,
    pub first_seen_at: DateTime<Utc>,
    pub last_indexed_at: Option<DateTime<Utc>>,
}

impl Dependency {
    pub fn new(name: String, version: String, language: ProjectLanguage) -> Self {
        let now = Utc::now();
        let language_str = match language {
            ProjectLanguage::Rust => "rust",
        };
        let id = (
            DEPENDENCY_TABLE,
            format!("{}:{}:{}", language_str, name, version),
        );
        Self {
            name,
            version,
            language,
            id: id.into(),
            first_seen_at: now,
            last_indexed_at: None,
        }
    }
}
