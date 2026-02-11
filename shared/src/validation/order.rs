use crate::dto::OrderItemRequest;
use crate::validation::constants::MIN_ORDER_ITEMS;

pub fn validate_order_items(items: &[OrderItemRequest]) -> Result<(), String> {
    if items.is_empty() {
        return Err(format!("At least {} item is required.", MIN_ORDER_ITEMS));
    }

    for (idx, item) in items.iter().enumerate() {
        if item.quantity < 1 {
            return Err(format!("Item {} must have quantity >= 1.", idx + 1));
        }
    }

    Ok(())
}
