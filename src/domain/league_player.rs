use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::dto::league_player::JoinRequest;

use super::enums::league_player_status::LeaguePlayerStatus;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct LeaguePlayer {
    pub id: u32,
    pub league_id: u32,
    pub player_id: u32,
    pub time_created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub status: String
}
impl From<JoinRequest> for LeaguePlayer {
    fn from(join_req: JoinRequest) -> Self {
        Self { id: 0, league_id: join_req.league_id, player_id: join_req.user_id, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), status: LeaguePlayerStatus::Requested.to_string() }
    }
}