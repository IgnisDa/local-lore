use std::{collections::HashMap, fs::read_to_string};

use anyhow::Result;
use log::debug;
use package_lock_json_parser::parse_dependencies;

use crate::models::{InsertDependency, ProjectLanguage};

pub async fn collect_dependencies(path: &str) -> Result<Vec<InsertDependency>> {
    debug!("Scanning directory for package-lock.json: {}", path);

    let package_lock_path = format!("{}/package-lock.json", path);

    let dependencies_result = match parse_dependencies(&read_to_string(package_lock_path)?) {
        Ok(deps) => deps,
        Err(e) => {
            debug!("Failed to parse package-lock.json for {}: {}", path, e);
            return Ok(Vec::new());
        }
    };

    debug!(
        "Found {} packages in package-lock.json",
        dependencies_result.len()
    );

    let mut dependencies_map: HashMap<(String, String), InsertDependency> = HashMap::new();

    for dep in dependencies_result {
        let name = dep.name;
        let version = dep.version;
        let key = (name.clone(), version.clone());
        dependencies_map
            .entry(key)
            .or_insert_with(|| InsertDependency::new(name, version, ProjectLanguage::JavaScript));
    }

    let dependencies: Vec<InsertDependency> = dependencies_map.into_values().collect();

    debug!(
        "Found {} unique JavaScript dependencies",
        dependencies.len()
    );

    Ok(dependencies)
}
