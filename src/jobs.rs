use std::sync::Arc;

use apalis::prelude::*;
use apalis_cron::CronContext;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{collectors::gather_project_dependencies, context::LocalLoreContext};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApplicationJob {
    DirectoryScan(String),
}

#[derive(Debug, Default)]
pub struct ScheduledJob;

pub async fn perform_application_job(
    job: ApplicationJob,
    ctx: Data<Arc<LocalLoreContext>>,
) -> Result<(), Error> {
    debug!("Processing job: {:?}", job);

    match job {
        ApplicationJob::DirectoryScan(path) => {
            gather_project_dependencies(&path, &ctx)
                .await
                .map_err(|e| Error::Failed(Arc::new(e.into())))?;
            Ok(())
        }
    }
}

pub async fn perform_scheduled_job(
    _job: ScheduledJob,
    cron_ctx: CronContext<chrono_tz::Tz>,
    _ctx: Data<Arc<LocalLoreContext>>,
) -> Result<(), Error> {
    debug!(
        "Running scheduled job at {:#?} (context available)",
        cron_ctx.get_timestamp()
    );
    Ok(())
}
