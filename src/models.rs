use serde::{Deserialize, Serialize};
use strum::Display;
use surrealdb::RecordId;

pub static DEPENDENCY_TABLE: &str = "dependency";

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "lowercase")]
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
        let id = (
            DEPENDENCY_TABLE,
            format!("{}:{}:{}", language.to_string(), name, version),
        );
        Self {
            name,
            version,
            language,
            id: id.into(),
        }
    }
}
