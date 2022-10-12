use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Place{
    pub id: u32,
    pub time_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub name: String,
    pub sport_id: u32,
    pub country: String,
    pub state: Option<String>,
    pub city: String,
    pub address: String,
    pub maps_url: Option<String>,
    pub contact_number: Option<String>,
    pub picture_url: Option<String>
}