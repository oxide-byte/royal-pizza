use crate::config::DatabaseConfig;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::{info, warn};

pub async fn create_db_client(config: &DatabaseConfig) -> Result<Surreal<Client>, surrealdb::Error> {
    const MAX_RETRIES: u32 = 10;
    const RETRY_DELAY_MS: u64 = 500;

    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        let url = config.url.clone();
        info!("Attempting to connect to SurrealDB [{}] (attempt {}/{})",url , attempt, MAX_RETRIES);

        match try_connect(config).await {
            Ok(db) => {
                info!("Successfully connected to SurrealDB");
                return Ok(db);
            }
            Err(e) => {
                warn!("Connection attempt {} failed: {}", attempt, e);
                last_error = Some(e);

                if attempt < MAX_RETRIES {
                    info!("Retrying in {}ms...", RETRY_DELAY_MS);
                    tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;
                }
            }
        }
    }

    Err(last_error.unwrap())
}

async fn try_connect(config: &DatabaseConfig) -> Result<Surreal<Client>, surrealdb::Error> {
    // Connect to SurrealDB via WebSocket
    let db = Surreal::new::<Ws>(&config.url).await?;

    // Sign in with Root credentials
    db.signin(Root {
        username: &config.username,
        password: &config.password,
    })
    .await?;

    // Use namespace and database
    db.use_ns(&config.namespace).use_db(&config.name).await?;

    Ok(db)
}