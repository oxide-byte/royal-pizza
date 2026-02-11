// Validation logic shared between frontend and backend
pub mod customer;
pub mod order;
pub mod pickup_time;

pub use customer::{validate_customer_name, validate_phone_number};
pub use order::validate_order_items;
pub use pickup_time::validate_pickup_time;

pub mod constants {
    pub const MIN_NAME_LENGTH: usize = 2;
    pub const MAX_NAME_LENGTH: usize = 100;
    pub const MIN_PICKUP_LEAD_TIME_MINUTES: i64 = 30;
    pub const MIN_ORDER_ITEMS: usize = 1;
}
