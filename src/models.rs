use serde::{Deserialize, Serialize};
use strum::Display;

pub static DEPENDENCY_TABLE: &str = "dependency";

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "lowercase")]
pub enum ProjectLanguage {
    Rust,
    JavaScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDependency {
    pub name: String,
    pub version: String,
    pub language: ProjectLanguage,
}

impl InsertDependency {
    pub fn new(name: String, version: String, language: ProjectLanguage) -> Self {
        Self {
            name,
            version,
            language,
        }
    }
}
