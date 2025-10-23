use std::{env, fs::create_dir_all, str::FromStr, sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use apalis::{
    layers::WorkerBuilderExt,
    prelude::{MemoryStorage, MessageQueue, Monitor, WorkerBuilder, WorkerFactoryFn},
};
use apalis_cron::{CronStream, Schedule};
use fastrace::{
    collector::{Config, ConsoleReporter},
    prelude::*,
};
use log::debug;
use logforth::append;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tokio::try_join;
use turbomcp::prelude::*;

use crate::{context::LocalLoreContext, migrator::Migrator};

mod collectors;
mod context;
mod entities;
mod jobs;
mod mcp;
mod migrator;
mod models;

#[derive(Clone)]
struct LocalLoreServer {
    ctx: Arc<LocalLoreContext>,
}

#[server(name = "Local Lore", version = "0.1.0", transports = ["stdio"])]
impl LocalLoreServer {
    fn new(ctx: Arc<LocalLoreContext>) -> Self {
        Self { ctx }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    fastrace::set_reporter(ConsoleReporter, Config::default());

    logforth::starter_log::builder()
        .dispatch(|d| d.append(append::Stderr::default()))
        .dispatch(|d| d.append(append::FastraceEvent::default()))
        .apply();

    let root = Span::root("local-lore-server", SpanContext::random());
    let _guard = root.set_local_parent();

    debug!("Starting Local Lore MCP server");

    let db = setup_database().await?;
    run_migrations(&db).await?;

    let project_context = Arc::new(LocalLoreContext::new(db));

    let mut application_job_storage = MemoryStorage::new();

    let current_dir = std::env::current_dir()?;
    let current_dir_str = current_dir
        .to_str()
        .ok_or_else(|| anyhow!("Current directory path contains invalid unicode"))?;

    debug!(
        "Deploying initial directory scan job for: {}",
        current_dir_str
    );
    application_job_storage
        .enqueue(jobs::ApplicationJob::GatherProjectDependencies(
            current_dir_str.to_string(),
        ))
        .await
        .map_err(|_| anyhow!("Failed to enqueue initial scan job"))?;

    let tz: chrono_tz::Tz = env::var("TZ")
        .map(|s| s.parse().unwrap())
        .unwrap_or_else(|_| chrono_tz::Etc::GMT);
    debug!("Timezone: {}", tz);

    let monitor = async {
        Monitor::new()
            .register(
                WorkerBuilder::new("perform_application_job")
                    .enable_tracing()
                    .catch_panic()
                    .rate_limit(10, Duration::new(5, 0))
                    .data(project_context.clone())
                    .backend(application_job_storage)
                    .build_fn(jobs::perform_application_job),
            )
            .register(
                WorkerBuilder::new("perform_scheduled_job")
                    .enable_tracing()
                    .catch_panic()
                    .data(project_context.clone())
                    .backend(CronStream::new_with_timezone(
                        Schedule::from_str("0 0 * * * *").unwrap(),
                        tz,
                    ))
                    .build_fn(jobs::perform_scheduled_job),
            )
            .run()
            .await
            .map_err(|e| anyhow!("{}", e))
    };

    debug!("Server initialization complete, starting stdio server and job monitor");

    let mcp_server = async {
        LocalLoreServer::new(project_context.clone())
            .run_stdio()
            .await
            .map_err(|e| anyhow!("{}", e))
    };

    let result = try_join!(monitor, mcp_server);

    fastrace::flush();
    result.map(|_| ())
}

async fn setup_database() -> Result<DatabaseConnection> {
    debug!("Setting up database");
    let data_dir = dirs::data_dir()
        .ok_or_else(|| anyhow!("Failed to determine data directory"))?
        .join("local-lore");
    create_dir_all(&data_dir)?;
    let db_path = data_dir.join("local-lore.db");
    let db_path_str = db_path
        .to_str()
        .ok_or_else(|| anyhow!("Database path includes invalid unicode characters"))?;
    let connection_string = format!("sqlite://{}?mode=rwc", db_path_str);
    debug!("Connecting to database at: {}", connection_string);
    let db = Database::connect(&connection_string).await?;
    debug!("Database connection established");
    Ok(db)
}

async fn run_migrations(db: &DatabaseConnection) -> Result<()> {
    debug!("Running database migrations");
    Migrator::up(db, None).await?;
    debug!("Database migrations completed");
    Ok(())
}
