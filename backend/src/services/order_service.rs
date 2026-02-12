use crate::middleware::error::AppError;
use crate::repository::{order_repo, pizza_repo};
use crate::utils::order_number::generate_order_number;
use chrono::Utc;
use shared::dto::{CreateOrderRequest, CreateOrderResponse};
use shared::models::{Order, OrderItem, OrderItemType, OrderStatus};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use uuid::Uuid;

pub async fn create_order(
    db: &Surreal<Client>,
    request: CreateOrderRequest,
) -> Result<CreateOrderResponse, AppError> {
    // Validate the request
    validate_order_request(&request)?;

    // Calculate order items with pricing
    let mut order_items = Vec::new();
    let mut total_amount = 0.0;

    for item_req in &request.items {
        let (item_type, unit_price) = match &item_req.item_type {
            OrderItemType::StandardPizza { pizza_id, size } => {
                // Fetch pizza from database
                let pizza = pizza_repo::query_pizza_by_id(db, pizza_id)
                    .await?
                    .ok_or_else(|| {
                        AppError::ValidationError(vec![format!(
                            "Pizza with id {} not found",
                            pizza_id
                        )])
                    })?;

                // Get price for the specified size
                let price = size.get_price(&pizza.price);

                (
                    OrderItemType::StandardPizza {
                        pizza_id: pizza_id.clone(),
                        size: size.clone(),
                    },
                    price,
                )
            }
            OrderItemType::CustomPizza { custom } => {
                let price = custom.get_price();
                (
                    OrderItemType::CustomPizza {
                        custom: custom.clone(),
                    },
                    price,
                )
            }
        };

        let subtotal = unit_price * item_req.quantity as f64;
        total_amount += subtotal;

        order_items.push(OrderItem {
            id: Uuid::new_v4().to_string(),
            item_type,
            quantity: item_req.quantity,
            unit_price,
            subtotal,
        });
    }

    // Generate order number
    let order_number = generate_order_number(db).await.map_err(|e| {
        AppError::InternalError(format!("Failed to generate order number: {}", e))
    })?;

    // Create order
    let order = Order {
        id: format!("order:{}", Uuid::new_v4()),
        order_number: order_number.clone(),
        customer: request.customer.clone(),
        items: order_items,
        total_amount,
        status: OrderStatus::Pending,
        pickup_time: request.pickup_time,
        created_at: Utc::now(),
    };

    // Save to database
    let created_order = order_repo::create_order_in_db(db, &order).await?;

    Ok(CreateOrderResponse {
        order_id: created_order.id,
        order_number,
        total_amount,
        pickup_time: request.pickup_time,
    })
}

pub async fn get_order_by_id(db: &Surreal<Client>, id: &str) -> Result<Order, AppError> {
    let order = order_repo::query_order_by_id(db, id).await?;

    order.ok_or_else(|| AppError::NotFound(format!("Order with id {} not found", id)))
}

fn validate_order_request(request: &CreateOrderRequest) -> Result<(), AppError> {
    use shared::validation::{
        validate_customer_name, validate_order_items, validate_phone_number, validate_pickup_time,
    };

    let mut errors = Vec::new();

    // Validate customer name
    if let Err(e) = validate_customer_name(&request.customer.name) {
        errors.push(e);
    }

    // Validate phone
    if let Err(e) = validate_phone_number(&request.customer.phone) {
        errors.push(e);
    }

    // Validate pickup time
    if let Err(e) = validate_pickup_time(request.pickup_time) {
        errors.push(e);
    }

    // Validate order items
    if let Err(e) = validate_order_items(&request.items) {
        errors.push(e);
    }

    if !errors.is_empty() {
        return Err(AppError::ValidationError(errors));
    }

    Ok(())
}
