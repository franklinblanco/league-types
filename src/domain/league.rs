use std::{fmt::Display, str::FromStr};

use chrono::{Utc, DateTime};
use err::Error;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;


use crate::dto::league::LeagueForCreationDto;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct League {
    pub id: u32,
    pub owner_id: u32,
    pub sport_id: u32,
    pub place_id: u32,
    pub time_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    /// State as in: Is the league open or closed? Not the geographical sense.
    pub state: String,
    pub visibility: String,
    /// When is the league happening?
    pub date_and_time: DateTime<Utc>,
    /// This will be stored as a Decimal in the database but the actual input from the user
    /// will not be in rust_decimal::Decimal type.
    pub cost_to_join: Decimal,
    /// This is a string because it's actually meaningless right now. 
    /// We're not taking payments so this doesn't matter, it's just what the user wants.
    pub currency: Option<String>,
    pub max_players: u32,
    pub description: Option<String>
}

impl From<LeagueForCreationDto> for League {
    fn from(league_dto: LeagueForCreationDto) -> Self {
        Self { 
            id: 0, owner_id: league_dto.user_id, sport_id: league_dto.sport_id, place_id:league_dto.place_id, time_created: Utc::now(), last_updated: Utc::now(), state: LeagueState::Open.to_string(),
            visibility: match league_dto.visibility {
                Some(visibility) => visibility.to_string(),
                None => LeagueVisibility::Public.to_string(),
        },
        date_and_time: league_dto.date_and_time, cost_to_join: league_dto.cost_to_join, currency: league_dto.currency, max_players: league_dto.max_players, description: league_dto.description 
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LeagueState {
    /// Taking new players
    #[default]
    Open,
    /// No more people
    Closed
}
impl Display for LeagueState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeagueState::Open => write!(f, "Open"),
            LeagueState::Closed => write!(f, "Closed"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LeagueVisibility {
    /// Open to anyone, anyone can join
    #[default]
    Public,
    /// People request to join
    Private,
    /// Only owner can see
    Unlisted
}

impl Display for LeagueVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeagueVisibility::Public => write!(f, "Public"), 
            LeagueVisibility::Private => write!(f, "Private"),
            LeagueVisibility::Unlisted => write!(f, "Unlisted"),
        }
    }
}
impl FromStr for LeagueVisibility {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Public" => Ok(LeagueVisibility::Public),
            "Private" => Ok(LeagueVisibility::Private),
            "Unlisted" => Ok(LeagueVisibility::Unlisted),
            _ => Err(Error::Unspecified)
        }
    }
}
