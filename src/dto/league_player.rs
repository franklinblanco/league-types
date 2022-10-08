use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct JoinRequest {
    #[serde(rename = "leagueId")]
    pub league_id: u32,
    #[serde(rename = "userId")]
    pub user_id: u32,
    #[serde(rename = "authToken")]
    pub auth_token: String,
}