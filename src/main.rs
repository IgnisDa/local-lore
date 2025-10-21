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
    async fn hello(&self, name: String) -> McpResult<String> {
        Ok(format!("Hello, {name}! Welcome to Local Lore."))
    }

    #[tool("Get information about the current directory")]
    async fn get_current_directory(&self) -> McpResult<String> {
        let current_dir = std::env::current_dir()
            .map_err(|e| McpError::internal(format!("Failed to get current directory: {}", e)))?;

        Ok(format!("Current directory: {}", current_dir.display()))
    }

    #[tool("List files and directories in the specified path")]
    async fn list_directory(&self, path: String) -> McpResult<Vec<String>> {
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
        Ok(items)
    }

    #[tool("Read the contents of a text file")]
    async fn read_file(&self, path: String) -> McpResult<String> {
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

        Ok(contents)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Surreal::new::<RocksDb>("./tmp/storage").await?;
    db.use_ns("main").use_db("main").await?;

    run_migrations(&db).await?;

    LocalLoreServer::new(db).run_stdio().await?;
    Ok(())
}

async fn run_migrations<C>(db: &Surreal<C>) -> Result<(), Box<dyn std::error::Error>>
where
    C: Connection,
{
    MigrationRunner::new(db).up().await?;
    Ok(())
}
