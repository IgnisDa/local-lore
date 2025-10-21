use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use cargo_metadata::{DependencyKind, MetadataCommand};
use futures::future::try_join_all;
use log::debug;

use crate::{
    context::ProjectContext,
    models::{DEPENDENCY_TABLE, Dependency, ProjectLanguage},
};

static CHUNK_SIZE: usize = 10;

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

    let workspace_members: Vec<_> = metadata
        .workspace_members
        .iter()
        .filter_map(|member_id| metadata.packages.iter().find(|p| &p.id == member_id))
        .collect();

    debug!("Found {} workspace members", workspace_members.len());

    for package in workspace_members {
        debug!(
            "Processing dependencies for package: {} v{}",
            package.name, package.version
        );

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
                    .or_insert_with(|| Dependency::new(name, version, ProjectLanguage::Rust));
            }
        }
    }

    let dependencies: Vec<Dependency> = dependencies_map.into_values().collect();

    debug!("Found {} unique dependencies", dependencies.len());

    let chunks: Vec<Vec<Dependency>> = dependencies
        .chunks(CHUNK_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect();

    debug!("Processing {} chunks of dependencies", chunks.len());

    let mut insert_tasks = Vec::new();
    for chunk in chunks {
        let ctx = ctx.clone();
        let task = async move {
            ctx.db
                .insert(DEPENDENCY_TABLE)
                .content(chunk)
                .await
                .map(|deps: Vec<Dependency>| deps.len())
        };
        insert_tasks.push(task);
    }

    let results = try_join_all(insert_tasks).await?;
    let total_inserted: usize = results.iter().sum();

    debug!(
        "Scan completed for: {}, inserted {} dependencies",
        path, total_inserted
    );
    Ok(())
}
