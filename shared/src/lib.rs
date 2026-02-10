// Shared module for Royal Pizza
// Contains data models, DTOs, and validation logic shared between frontend and backend

pub mod models;
pub mod dto;
pub mod validation;

// Re-export commonly used types
pub use dto::*;
pub use models::*;
