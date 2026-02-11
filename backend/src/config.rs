use serde::Deserialize;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_allow_origin: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub namespace: String,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Surreal<Client>>,
    pub config: Arc<Config>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|e| format!("Invalid PORT value: {}", e))?;
        let cors_allow_origin = std::env::var("CORS_ALLOW_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL environment variable is required".to_string())?;
        let database_namespace = std::env::var("DATABASE_NAMESPACE")
            .unwrap_or_else(|_| "royalpizza".to_string());
        let database_name =
            std::env::var("DATABASE_NAME").unwrap_or_else(|_| "development".to_string());
        let database_username =
            std::env::var("DATABASE_USERNAME").unwrap_or_else(|_| "root".to_string());
        let database_password =
            std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "root".to_string());

        Ok(Config {
            server: ServerConfig {
                host,
                port,
                cors_allow_origin,
            },
            database: DatabaseConfig {
                url: database_url,
                namespace: database_namespace,
                name: database_name,
                username: database_username,
                password: database_password,
            },
        })
    }
}
