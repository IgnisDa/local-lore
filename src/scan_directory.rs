use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use cargo_metadata::MetadataCommand;
use log::debug;

use crate::{
    context::ProjectContext,
    models::{DEPENDENCY_TABLE, Dependency},
};

pub async fn scan_directory(path: &str, ctx: &Arc<ProjectContext>) -> Result<()> {
    debug!("Scanning directory: {}", path);

    let metadata = match MetadataCommand::new()
        .manifest_path(format!("{}/Cargo.toml", path))
        .exec()
    {
        Ok(metadata) => metadata,
        Err(e) => {
            debug!("Failed to execute cargo metadata for {}: {}", path, e);
            return Ok(());
        }
    };

    debug!("Found {} packages in workspace", metadata.packages.len());

    let mut dependencies_map: HashMap<(String, String), Dependency> = HashMap::new();

    if let Some(resolve) = metadata.resolve {
        for node in resolve.nodes {
            if let Some(package) = metadata.packages.iter().find(|p| p.id == node.id) {
                let name = package.name.to_string();
                let version = package.version.to_string();
                let key = (name.clone(), version.clone());
                dependencies_map
                    .entry(key)
                    .or_insert_with(|| Dependency::new(name, version));
            }
        }
    }

    let dependencies: Vec<Dependency> = dependencies_map.into_values().collect();

    debug!("Found {} unique dependencies", dependencies.len());

    let inserted: Vec<Dependency> = ctx
        .db
        .insert(DEPENDENCY_TABLE)
        .content(dependencies)
        .await?;

    debug!(
        "Scan completed for: {}, inserted {} dependencies",
        path,
        inserted.len()
    );
    Ok(())
}
