use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tracing::{info, warn};

/// Seeds the database with schema and initial data
///
/// This function reads the schema.surql and init.surql files and executes them
/// against the database. It's designed to be idempotent and safe to run multiple times.
///
/// # Arguments
/// * `db` - SurrealDB client connection
/// * `force_reseed` - If true, will re-run initialization even if data exists
///
/// # Returns
/// Result indicating success or failure of seeding operation
pub async fn seed_database(
    db: &Surreal<Client>,
    force_reseed: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting database seeding...");

    // Check if pizzas already exist
    let check_query = "SELECT count() FROM pizza GROUP ALL";
    let mut result = db.query(check_query).await?;
    let counts: Vec<serde_json::Value> = result.take(0)?;

    let existing_count = counts
        .first()
        .and_then(|v| v.get("count"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    if existing_count > 0 && !force_reseed {
        info!("Database already seeded with {} pizzas, skipping initialization", existing_count);
        return Ok(());
    }

    if force_reseed && existing_count > 0 {
        warn!("Force reseed enabled, existing data will be replaced");
    }

    // Execute schema definition
    info!("Executing schema definition...");
    let schema_sql = include_str!("../../../database/schema.surql");
    execute_sql_script(db, schema_sql, "schema").await?;

    // Execute initialization data
    info!("Executing initialization data...");
    let init_sql = include_str!("../../../database/init.surql");
    execute_sql_script(db, init_sql, "initialization").await?;

    // Verify seeding
    let mut verify_result = db.query("SELECT count() FROM pizza GROUP ALL").await?;
    let verify_counts: Vec<serde_json::Value> = verify_result.take(0)?;
    let final_count = verify_counts
        .first()
        .and_then(|v| v.get("count"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    info!("Database seeding completed successfully! {} pizzas available", final_count);

    Ok(())
}

/// Executes a SQL script and handles errors
async fn execute_sql_script(
    db: &Surreal<Client>,
    sql: &str,
    script_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match db.query(sql).await {
        Ok(_) => {
            info!("Successfully executed {} script", script_name);
            Ok(())
        }
        Err(e) => {
            warn!("Error executing {} script: {:?}", script_name, e);
            Err(Box::new(e))
        }
    }
}

/// Initialize database with schema only (no seed data)
/// Useful for production environments where data should be loaded separately
#[allow(dead_code)]
pub async fn init_schema_only(
    db: &Surreal<Client>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing database schema...");
    let schema_sql = include_str!("../../../database/schema.surql");
    execute_sql_script(db, schema_sql, "schema").await?;
    info!("Schema initialization completed");
    Ok(())
}
