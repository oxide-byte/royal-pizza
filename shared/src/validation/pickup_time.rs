use chrono::{DateTime, Duration, Utc};

use crate::validation::constants::MIN_PICKUP_LEAD_TIME_MINUTES;

pub fn validate_pickup_time(pickup_time: DateTime<Utc>) -> Result<(), String> {
    let min_pickup_time = Utc::now() + Duration::minutes(MIN_PICKUP_LEAD_TIME_MINUTES);

    if pickup_time < min_pickup_time {
        return Err(format!(
            "Pickup time must be at least {} minutes from now.",
            MIN_PICKUP_LEAD_TIME_MINUTES
        ));
    }

    Ok(())
}
