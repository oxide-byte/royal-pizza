use crate::middleware::error::AppError;
use crate::repository::pizza_repo;
use shared::dto::GetPizzasResponse;
use shared::models::Pizza;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn get_all_available_pizzas(
    db: &Surreal<Client>,
) -> Result<GetPizzasResponse, AppError> {
    let pizzas = pizza_repo::query_all_available_pizzas(db).await?;
    Ok(GetPizzasResponse { pizzas })
}

pub async fn get_pizza_by_id(db: &Surreal<Client>, id: &str) -> Result<Pizza, AppError> {
    let pizza = pizza_repo::query_pizza_by_id(db, id).await?;

    pizza.ok_or_else(|| AppError::NotFound(format!("Pizza with id {} not found", id)))
}
