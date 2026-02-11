use crate::repository::order_repo;
use chrono::Utc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

/// Generate order number in format: RP-YYYYMMDD-NNN
/// Example: RP-20260211-001
pub async fn generate_order_number(db: &Surreal<Client>) -> Result<String, surrealdb::Error> {
    let now = Utc::now();
    let date_prefix = now.format("%Y%m%d").to_string();

    // Query database for today's order count
    let count = order_repo::count_orders_for_date(db, &date_prefix).await?;

    // Increment sequence number and pad to 3 digits
    let sequence = count + 1;
    let order_number = format!("RP-{}-{:03}", date_prefix, sequence);

    Ok(order_number)
}
