use crate::validation::constants::{MAX_NAME_LENGTH, MIN_NAME_LENGTH};

pub fn validate_customer_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Err("Customer name is required.".to_string());
    }

    if trimmed.len() < MIN_NAME_LENGTH {
        return Err(format!(
            "Customer name must be at least {} characters.",
            MIN_NAME_LENGTH
        ));
    }

    if trimmed.len() > MAX_NAME_LENGTH {
        return Err(format!(
            "Customer name cannot exceed {} characters.",
            MAX_NAME_LENGTH
        ));
    }

    Ok(())
}

pub fn validate_phone_number(phone: &str) -> Result<(), String> {
    if phone.trim().is_empty() {
        return Err("Phone number is required.".to_string());
    }
    Ok(())
}
