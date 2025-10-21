use std::sync::Arc;

use anyhow::Result;
use log::debug;
use surrealdb::{Surreal, engine::local::Db};

pub async fn scan_directory(path: &str, _db: &Arc<Surreal<Db>>) -> Result<()> {
    debug!("Scanning directory: {}", path);
    Ok(())
}
