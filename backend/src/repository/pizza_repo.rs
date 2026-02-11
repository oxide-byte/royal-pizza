use shared::models::Pizza;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn query_all_available_pizzas(
    db: &Surreal<Client>,
) -> Result<Vec<Pizza>, surrealdb::Error> {
    let mut result = db
        .query("SELECT * FROM pizza WHERE is_available = true")
        .await?;

    let pizzas: Vec<Pizza> = result.take(0)?;
    Ok(pizzas)
}

pub async fn query_pizza_by_id(
    db: &Surreal<Client>,
    id: &str,
) -> Result<Option<Pizza>, surrealdb::Error> {
    let mut result = db
        .query("SELECT * FROM pizza WHERE id = $id")
        .bind(("id", id.to_string()))
        .await?;

    let pizzas: Vec<Pizza> = result.take(0)?;
    Ok(pizzas.into_iter().next())
}
