use std::sync::Arc;

use anyhow::Result;
use futures::future::try_join_all;
use log::debug;

use crate::{
    context::ProjectContext,
    models::{DEPENDENCY_TABLE, InsertDependency},
};

pub mod cargo_lock;
pub mod package_lock;

static CHUNK_SIZE: usize = 20;

pub async fn scan_directory(path: &str, ctx: &Arc<ProjectContext>) -> Result<()> {
    let mut all_dependencies = Vec::new();

    let rust_deps = cargo_lock::collect_dependencies(path).await?;
    all_dependencies.extend(rust_deps);

    let js_deps = package_lock::collect_dependencies(path).await?;
    all_dependencies.extend(js_deps);

    let chunks: Vec<Vec<InsertDependency>> = all_dependencies
        .chunks(CHUNK_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect();

    debug!("Processing {} chunks of dependencies", chunks.len());

    let mut total_upsert = 0;
    for chunk in chunks {
        let mut upsert_tasks = Vec::new();
        for dependency in chunk {
            let ctx = ctx.clone();
            let task = async move {
                ctx.db
                    .upsert(DEPENDENCY_TABLE)
                    .content(dependency)
                    .await
                    .map(|e: Vec<InsertDependency>| e.len())
            };
            upsert_tasks.push(task);
        }
        let results = try_join_all(upsert_tasks).await?;
        total_upsert += results.iter().sum::<usize>();
    }

    debug!(
        "Scan completed for: {} and upsert {} dependencies",
        path, total_upsert
    );

    let mut dependencies_to_index_response = ctx
        .db
        .query(format!(
            "SELECT * FROM {DEPENDENCY_TABLE} WHERE last_indexed_at = NONE"
        ))
        .await?;

    let dependencies_to_index: Vec<InsertDependency> = dependencies_to_index_response.take(0)?;

    dbg!(dependencies_to_index);

    Ok(())
}
