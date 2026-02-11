use shared::models::Order;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn create_order_in_db(
    db: &Surreal<Client>,
    order: &Order,
) -> Result<Order, surrealdb::Error> {
    let order_clone = order.clone();
    let created: Option<Order> = db.create("order").content(order_clone).await?;

    created.ok_or_else(|| {
        surrealdb::Error::Api(surrealdb::error::Api::Query(
            "Failed to create order".to_string(),
        ))
    })
}

pub async fn query_order_by_id(
    db: &Surreal<Client>,
    id: &str,
) -> Result<Option<Order>, surrealdb::Error> {
    let mut result = db
        .query("SELECT * FROM order WHERE id = $id")
        .bind(("id", id.to_string()))
        .await?;

    let orders: Vec<Order> = result.take(0)?;
    Ok(orders.into_iter().next())
}

pub async fn count_orders_for_date(
    db: &Surreal<Client>,
    date_prefix: &str,
) -> Result<usize, surrealdb::Error> {
    let mut result = db
        .query("SELECT count() FROM order WHERE order_number LIKE $pattern GROUP ALL")
        .bind(("pattern", format!("RP-{}-___", date_prefix)))
        .await?;

    let counts: Vec<serde_json::Value> = result.take(0)?;

    if let Some(count_obj) = counts.first() {
        if let Some(count) = count_obj.get("count").and_then(|v| v.as_u64()) {
            return Ok(count as usize);
        }
    }

    Ok(0)
}
