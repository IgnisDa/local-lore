use std::sync::Arc;

use anyhow::Result;
use futures::future::try_join_all;
use log::debug;

use crate::{
    context::ProjectContext,
    models::{DEPENDENCY_TABLE, Dependency},
};

pub mod cargo_lock;
pub mod package_lock;

static CHUNK_SIZE: usize = 10;

pub async fn scan_directory(path: &str, ctx: &Arc<ProjectContext>) -> Result<()> {
    let mut all_dependencies = Vec::new();

    let rust_deps = cargo_lock::collect_dependencies(path).await?;
    all_dependencies.extend(rust_deps);

    let js_deps = package_lock::collect_dependencies(path).await?;
    all_dependencies.extend(js_deps);

    let chunks: Vec<Vec<Dependency>> = all_dependencies
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
