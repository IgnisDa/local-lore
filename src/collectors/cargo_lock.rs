use std::collections::HashMap;

use anyhow::Result;
use cargo_metadata::{DependencyKind, MetadataCommand};
use log::debug;

use crate::models::{InsertDependency, ProjectLanguage};

pub async fn collect_dependencies(path: &str) -> Result<Vec<InsertDependency>> {
    debug!("Scanning directory: {}", path);

    let metadata = match MetadataCommand::new()
        .manifest_path(format!("{}/Cargo.toml", path))
        .exec()
    {
        Ok(metadata) => metadata,
        Err(e) => {
            debug!("Failed to execute cargo metadata for {}: {}", path, e);
            return Ok(Vec::new());
        }
    };

    debug!("Found {} packages in workspace", metadata.packages.len());

    let mut dependencies_map: HashMap<(String, String), InsertDependency> = HashMap::new();

    let workspace_members: Vec<_> = metadata
        .workspace_members
        .iter()
        .filter_map(|member_id| metadata.packages.iter().find(|p| &p.id == member_id))
        .collect();

    debug!("Found {} workspace members", workspace_members.len());

    for package in workspace_members {
        for dep in &package.dependencies {
            if matches!(dep.kind, DependencyKind::Build) {
                continue;
            }

            if let Some(resolved_package) = metadata.packages.iter().find(|p| p.name == dep.name) {
                let name = resolved_package.name.to_string();
                let version = resolved_package.version.to_string();
                let key = (name.clone(), version.clone());
                dependencies_map
                    .entry(key)
                    .or_insert_with(|| InsertDependency::new(name, version, ProjectLanguage::Rust));
            }
        }
    }

    let dependencies: Vec<InsertDependency> = dependencies_map.into_values().collect();

    debug!("Found {} unique dependencies", dependencies.len());

    Ok(dependencies)
}
