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

#[server(name = "local-lore", version = "0.1.0")]
impl LocalLoreServer {
    fn new(db: Surreal<Db>) -> Self {
        Self { db }
    }

    #[tool("Say hello to someone")]
    #[trace]
    async fn hello(&self, name: String) -> McpResult<String> {
        debug!("Hello called with name: {}", name);
        Ok(format!("Hello, {name}! Welcome to Local Lore."))
    }

    #[tool("Get information about the current directory")]
    #[trace]
    async fn get_current_directory(&self) -> McpResult<String> {
        debug!("Getting current directory");
        let current_dir = std::env::current_dir()
            .map_err(|e| McpError::internal(format!("Failed to get current directory: {}", e)))?;

        Ok(format!("Current directory: {}", current_dir.display()))
    }

    #[tool("List files and directories in the specified path")]
    #[trace]
    async fn list_directory(&self, path: String) -> McpResult<Vec<String>> {
        debug!("Listing directory: {}", path);
        let dir_path = std::path::Path::new(&path);

        if !dir_path.exists() {
            return Err(McpError::invalid_request(format!(
                "Path does not exist: {}",
                path
            )));
        }

        if !dir_path.is_dir() {
            return Err(McpError::invalid_request(format!(
                "Path is not a directory: {}",
                path
            )));
        }

        let entries = std::fs::read_dir(dir_path)
            .map_err(|e| McpError::internal(format!("Failed to read directory: {}", e)))?;

        let mut items = Vec::new();
        for entry in entries {
            let entry =
                entry.map_err(|e| McpError::internal(format!("Failed to read entry: {}", e)))?;
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy().to_string();

            let prefix = if entry.path().is_dir() {
                "[DIR] "
            } else {
                "[FILE] "
            };
            items.push(format!("{}{}", prefix, name));
        }

        items.sort();
        debug!("Found {} items in directory", items.len());
        Ok(items)
    }

    #[tool("Read the contents of a text file")]
    #[trace]
    async fn read_file(&self, path: String) -> McpResult<String> {
        debug!("Reading file: {}", path);
        let file_path = std::path::Path::new(&path);

        if !file_path.exists() {
            return Err(McpError::invalid_request(format!(
                "File does not exist: {}",
                path
            )));
        }

        if !file_path.is_file() {
            return Err(McpError::invalid_request(format!(
                "Path is not a file: {}",
                path
            )));
        }

        let contents = std::fs::read_to_string(file_path)
            .map_err(|e| McpError::internal(format!("Failed to read file: {}", e)))?;

        debug!("Successfully read file ({} bytes)", contents.len());
        Ok(contents)
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

#[trace]
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

#[trace]
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
