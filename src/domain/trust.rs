use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};

use crate::dto::trust::TrustRequestDto;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Trust {
    pub id: u32,
    /// The player who is trusting (sending the trust request)
    pub truster_id: u32,
    /// The player who is being trusted (recieving the trust request)
    pub trustee_id: u32,
    pub time_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>
}

impl From<TrustRequestDto> for Trust {
    fn from(trust_dto: TrustRequestDto) -> Self {
        Trust { id: 0, truster_id: trust_dto.truster_id, trustee_id: trust_dto.trustee_id, time_created: Utc::now(), last_updated: Utc::now() }
    }
}