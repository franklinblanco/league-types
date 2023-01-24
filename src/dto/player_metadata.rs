use serde::{Serialize, Deserialize};

use crate::domain::player::Player;


/// Mainly used for the Chat, so that a user can request and store many players info
/// Should be as small as possible to reduce strain on servers and client storage.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct PlayerMetadata {
    pub id: u32,
    pub name: String,
    pub profile_picture_url: Option<String>,
}

impl From<Player> for PlayerMetadata {
    fn from(value: Player) -> Self {
        PlayerMetadata { id: value.id, name: value.name, profile_picture_url: value.profile_picture_url }
    }
}

