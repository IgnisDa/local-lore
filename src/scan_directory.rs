use std::sync::Arc;

use anyhow::Result;
use log::debug;

use crate::context::ProjectContext;

pub async fn scan_directory(path: &str, _ctx: &Arc<ProjectContext>) -> Result<()> {
    debug!("Scanning directory: {}", path);
    Ok(())
}
