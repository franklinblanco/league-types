use std::{fmt::{Display}, str::FromStr};

use serde::{Serialize, Deserialize};
use err::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LeaguePlayerStatus {
    Denied,
    Joined,
    #[default]
    Requested,
    Kicked,
    Left,
    Invited
}
impl Display for LeaguePlayerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeaguePlayerStatus::Denied => write!(f, "Denied"),
            LeaguePlayerStatus::Joined => write!(f, "Joined"),
            LeaguePlayerStatus::Requested => write!(f, "Requested"),
            LeaguePlayerStatus::Kicked => write!(f, "Kicked"),
            LeaguePlayerStatus::Left => write!(f, "Left"),
            LeaguePlayerStatus::Invited => write!(f, "Invited")
        }
    }
}
impl FromStr for LeaguePlayerStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Denied" => Ok(Self::Denied),
            "Joined" => Ok(Self::Joined),
            "Requested" => Ok(Self::Requested),
            "Kicked" => Ok(Self::Kicked),
            "Left" => Ok(Self::Requested),
            "Invited" => Ok(Self::Invited),
            _ => Err(Error::Unspecified) //TODO: Create ParseStr error in actix_web_utils
        }
    }
}