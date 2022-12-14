use chrono::{Utc, DateTime, NaiveDate};
use serde::{Serialize, Deserialize};

use crate::dto::player::PlayerForCreationDto;

//TODO: Remove sensitive information from player struct
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Player {
    pub id: u32,
    pub time_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub name: String,
    pub birth_date: NaiveDate,
    pub country: String,
    pub city: String,
    pub identification_number: Option<String>,
    pub bio: Option<String>,
    pub profile_picture_url: Option<String>,
    pub id_verified: bool,
    pub phone_number_verified: bool,
}

impl From<PlayerForCreationDto> for Player {
    fn from(player_dto: PlayerForCreationDto) -> Self {
        Player {  id: Default::default(), time_created: Utc::now(), last_updated: Utc::now().with_timezone(&Utc), name: player_dto.name.clone(), birth_date: player_dto.birth_date.clone(), country: player_dto.country.clone(), city: player_dto.city.clone(), identification_number: None, bio: None, profile_picture_url: None, id_verified: false, phone_number_verified: false }
    }
}

impl Player{
    pub fn clear_all_sensitive_fields(mut self) -> Self {
        self.birth_date = Utc::now().date_naive();
        self.city = "".to_string();
        self.id_verified = false;
        self.identification_number = None;
        self.last_updated = Utc::now();
        self
    }
}