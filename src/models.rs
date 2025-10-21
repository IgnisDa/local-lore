use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

pub static DEPENDENCY_TABLE: &str = "dependency";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectLanguage {
    Rust,
    JavaScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDependency {
    pub id: RecordId,
    pub name: String,
    pub version: String,
    pub language: ProjectLanguage,
}

impl InsertDependency {
    pub fn new(name: String, version: String, language: ProjectLanguage) -> Self {
        let language_str = match language {
            ProjectLanguage::Rust => "rust",
            ProjectLanguage::JavaScript => "javascript",
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
        }
    }
}
