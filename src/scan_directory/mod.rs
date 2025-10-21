use std::sync::Arc;

use anyhow::Result;
use futures::future::try_join_all;
use log::debug;

use crate::{
    context::ProjectContext,
    models::{DEPENDENCY_TABLE, Dependency},
};

pub mod rust;

static CHUNK_SIZE: usize = 10;

pub async fn scan_directory(path: &str, ctx: &Arc<ProjectContext>) -> Result<()> {
    let dependencies = rust::scan_directory(path).await?;

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
