use crate::effects;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tritone {
    #[serde(rename = "ix")]
    pub index: i64,
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "ef")]
    pub effects: (
        effects::Color,
        effects::Color,
        effects::Color,
        effects::Slider,
    ),
}

impl Tritone {
    pub const TY: u8 = 23;
}
