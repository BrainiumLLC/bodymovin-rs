use crate::effects;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Group {
    #[serde(rename = "ix")]
    pub index: i64,
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "ef")]
    pub effects: Vec<effects::Index>,
    #[serde(rename = "en")]
    pub enabled: i64,
}

impl Group {
    pub const TY: u8 = 5;
}
