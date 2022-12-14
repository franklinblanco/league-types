use chrono::{Utc, DateTime};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Field {
    pub id: u32,
    pub place_id: u32,
    pub time_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub country: String,
    pub city: String,
    pub name: String,
    pub price_per_hour: Decimal,
    pub currency: String,
    pub description: Option<String>,
}

impl Field {
    pub fn new() -> Field {
        Field { id: 0, place_id: 0, time_created: Utc::now(), last_updated: Utc::now(), country: "".to_string(), city: "".to_string(), name: "".to_string(), price_per_hour: Decimal::new(0, 0), currency: "".to_string(), description: None }
    }
}