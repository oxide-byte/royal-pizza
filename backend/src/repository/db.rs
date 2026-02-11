use crate::config::DatabaseConfig;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn create_db_client(config: &DatabaseConfig) -> Result<Surreal<Client>, surrealdb::Error> {
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
