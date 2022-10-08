use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Sport{
    pub id: u32,
    pub name: String,
    pub category_id: u32
}