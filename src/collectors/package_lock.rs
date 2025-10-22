use std::{collections::HashMap, fs::read_to_string};

use anyhow::Result;
use log::debug;
use package_lock_json_parser::parse_dependencies;

use crate::{collectors::CollectorDependency, models::ProjectLanguage};

pub async fn collect_dependencies(path: &str) -> Result<Vec<CollectorDependency>> {
    debug!("Scanning directory for package-lock.json: {}", path);

    let package_lock_path = format!("{}/package-lock.json", path);
    let contents = match read_to_string(&package_lock_path) {
        Ok(c) => c,
        Err(_e) => {
            debug!("Could not read package-lock.json at {}", package_lock_path);
            return Ok(Vec::new());
        }
    };

    let dependencies_result = match parse_dependencies(&contents) {
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

    let mut dependencies_map: HashMap<(String, String), CollectorDependency> = HashMap::new();

    for dep in dependencies_result {
        let name = dep.name;
        let version = dep.version;
        let key = (name.clone(), version.clone());
        dependencies_map.entry(key).or_insert_with(|| {
            CollectorDependency::new(name, version, ProjectLanguage::JavaScript)
        });
    }

    let dependencies: Vec<CollectorDependency> = dependencies_map.into_values().collect();

    debug!(
        "Found {} unique JavaScript dependencies",
        dependencies.len()
    );

    Ok(dependencies)
}
