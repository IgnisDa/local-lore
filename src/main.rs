use std::{env, fs::create_dir_all, str::FromStr, time::Duration};

use anyhow::{Result, anyhow};
use apalis::{
    layers::WorkerBuilderExt,
    prelude::{MemoryStorage, Monitor, WorkerBuilder, WorkerFactoryFn},
};
use apalis_cron::{CronStream, Schedule};
use fastrace::collector::{Config, ConsoleReporter};
use fastrace::prelude::*;
use log::debug;
use logforth::append;
use surrealdb::{
    Connection, Surreal,
    engine::local::{Db, RocksDb},
};
use surrealdb_migrations::MigrationRunner;
use tokio::try_join;
use turbomcp::prelude::*;

mod jobs;

#[derive(Clone)]
struct LocalLoreServer {
    db: Surreal<Db>,
}

#[server(name = "Local Lore", version = "0.1.0")]
impl LocalLoreServer {
    fn new(db: Surreal<Db>) -> Self {
        Self { db }
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

    let application_job_storage = MemoryStorage::new();

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
                    .backend(application_job_storage)
                    .build_fn(jobs::perform_application_job),
            )
            .register(
                WorkerBuilder::new("perform_scheduled_job")
                    .enable_tracing()
                    .catch_panic()
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
        LocalLoreServer::new(db)
            .run_stdio()
            .await
            .map_err(|e| anyhow!("{}", e))
    };

    let result = try_join!(monitor, mcp_server);

    fastrace::flush();
    result.map(|_| ())
}

async fn setup_database() -> Result<Surreal<Db>> {
    debug!("Setting up database");
    let data_dir = dirs::data_dir()
        .ok_or_else(|| anyhow!("Failed to determine data directory"))?
        .join("local-lore");
    create_dir_all(&data_dir)?;
    let storage_path = data_dir.join("storage");
    create_dir_all(&storage_path)?;
    let storage_path = storage_path
        .to_str()
        .ok_or_else(|| anyhow!("Storage path includes invalid unicode characters"))?;
    debug!("Connecting to database at: {}", storage_path);
    let db = Surreal::new::<RocksDb>(storage_path).await?;
    db.use_ns("main").use_db("main").await?;
    debug!("Database connection established");
    Ok(db)
}

async fn run_migrations<C>(db: &Surreal<C>) -> Result<()>
where
    C: Connection,
{
    debug!("Running database migrations");
    MigrationRunner::new(db)
        .up()
        .await
        .map_err(|e| anyhow!("{}", e))?;
    debug!("Database migrations completed");
    Ok(())
}
