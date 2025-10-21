use std::fs::create_dir_all;

use anyhow::{Result, anyhow};
use fastrace::collector::{Config, ConsoleReporter};
use fastrace::prelude::*;
use log::debug;
use logforth::append;
use surrealdb::{
    Connection, Surreal,
    engine::local::{Db, RocksDb},
};
use surrealdb_migrations::MigrationRunner;
use turbomcp::prelude::*;

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

    logforth::builder()
        .dispatch(|d| d.append(append::Stderr::default()))
        .dispatch(|d| d.append(append::FastraceEvent::default()))
        .apply();

    let root = Span::root("local-lore-server", SpanContext::random());
    let _guard = root.set_local_parent();

    debug!("Starting Local Lore MCP server");

    let db = setup_database().await?;
    run_migrations(&db).await?;

    debug!("Server initialization complete, starting stdio server");

    let result = LocalLoreServer::new(db)
        .run_stdio()
        .await
        .map_err(|e| anyhow!("{}", e));

    fastrace::flush();
    result
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
