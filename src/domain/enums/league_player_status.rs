use std::{fmt::{Display}, str::FromStr};

use serde::{Serialize, Deserialize};
use err::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LeaguePlayerStatus {
    /// The player requested to join but got denied by the league owner
    Denied,
    /// The player joined either by getting accepted or by joining an open league
    /// Active group (Can only have one at a time)
    Joined,
    /// The player is requesting to join a league
    /// Active group (Can only have one at a time)
    #[default]
    Requested,
    /// The player was already in a league and was kicked out.
    /// Can't request to join after this.
    Kicked,
    /// The player exited the league queue (Exited before request being approved)
    Left,
    /// Player was invited to join league
    Invited,
    /// Player was already in a league and decided to canel (Only way to come back is through invites)
    Canceled,
}
impl Display for LeaguePlayerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeaguePlayerStatus::Denied => write!(f, "Denied"),
            LeaguePlayerStatus::Joined => write!(f, "Joined"),
            LeaguePlayerStatus::Requested => write!(f, "Requested"),
            LeaguePlayerStatus::Kicked => write!(f, "Kicked"),
            LeaguePlayerStatus::Left => write!(f, "Left"),
            LeaguePlayerStatus::Invited => write!(f, "Invited"),
            LeaguePlayerStatus::Canceled => write!(f, "Canceled")
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
            "Canceled" => Ok(Self::Canceled),
            _ => Err(Error::Unspecified),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatusType {
    /// Inside a league
    Active,
    /// Either outside of a league, or attempting to join
    Inactive,
}

impl LeaguePlayerStatus {
    pub fn get_status_type(&self) -> StatusType {
        match self {
            LeaguePlayerStatus::Denied => StatusType::Inactive,
            LeaguePlayerStatus::Joined => StatusType::Active,
            LeaguePlayerStatus::Requested => StatusType::Active,
            LeaguePlayerStatus::Kicked => StatusType::Inactive,
            LeaguePlayerStatus::Left => StatusType::Inactive,
            LeaguePlayerStatus::Invited => StatusType::Inactive,
            LeaguePlayerStatus::Canceled => StatusType::Inactive,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApprovalStatus {
    Approved,
    Denied,
}