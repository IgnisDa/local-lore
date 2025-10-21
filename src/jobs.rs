use apalis::prelude::*;
use apalis_cron::CronContext;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApplicationJob {
    CleanupTasks,
    FileIndexing(String),
    DirectoryScan(String),
}

#[derive(Debug, Default)]
pub struct ScheduledJob;

pub async fn perform_application_job(job: ApplicationJob) -> Result<(), Error> {
    debug!("Processing job: {:?}", job);

    match job {
        ApplicationJob::FileIndexing(path) => {
            debug!("File indexing job for path: {}", path);
            Ok(())
        }
        ApplicationJob::CleanupTasks => {
            debug!("Running cleanup tasks");
            Ok(())
        }
        ApplicationJob::DirectoryScan(path) => {
            debug!("Directory scan job for path: {}", path);
            Ok(())
        }
    }
}

pub async fn perform_scheduled_job(
    _job: ScheduledJob,
    ctx: CronContext<chrono_tz::Tz>,
) -> Result<(), Error> {
    debug!("Running scheduled job at {:#?}", ctx.get_timestamp());
    Ok(())
}
