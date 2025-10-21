use std::sync::Arc;

use apalis::prelude::*;
use apalis_cron::CronContext;
use log::debug;
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::local::Db};

use crate::scan_directory::scan_directory;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApplicationJob {
    CleanupTasks,
    FileIndexing(String),
    DirectoryScan(String),
}

#[derive(Debug, Default)]
pub struct ScheduledJob;

pub async fn perform_application_job(
    job: ApplicationJob,
    db: Data<Arc<Surreal<Db>>>,
) -> Result<(), Error> {
    debug!("Processing job: {:?}", job);

    match job {
        ApplicationJob::FileIndexing(path) => {
            debug!("File indexing job for path: {} (db available)", path);
            Ok(())
        }
        ApplicationJob::CleanupTasks => {
            debug!("Running cleanup tasks (db available)");
            Ok(())
        }
        ApplicationJob::DirectoryScan(path) => {
            scan_directory(&path, &db)
                .await
                .map_err(|e| Error::Failed(Arc::new(e.into())))?;
            Ok(())
        }
    }
}

pub async fn perform_scheduled_job(
    _job: ScheduledJob,
    ctx: CronContext<chrono_tz::Tz>,
    _db: Data<Arc<Surreal<Db>>>,
) -> Result<(), Error> {
    debug!(
        "Running scheduled job at {:#?} (db available)",
        ctx.get_timestamp()
    );
    Ok(())
}
