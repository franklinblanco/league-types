use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use crate::domain::league::LeagueVisibility;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct LeagueForCreationDto{
    #[serde(rename = "userId")]
    pub user_id: u32,
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "sportId")]
    pub sport_id: u32,
    #[serde(rename = "placeId")]
    pub place_id: u32,
    pub visibility: Option<LeagueVisibility>,
    #[serde(rename = "dateAndTime")]
    pub date_and_time: DateTime<Utc>,
    #[serde(rename = "costToJoin")]
    pub cost_to_join: Decimal,
    pub currency: Option<String>,
    #[serde(rename = "maxPlayers")]
    pub max_players: u32,
    pub description: Option<String>
}

