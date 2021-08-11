use crate::effects;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Fill {
    #[serde(rename = "ix")]
    pub index: i64,
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "ef")]
    pub effects: (
        effects::Point,
        effects::DropDown,
        effects::Color,
        effects::DropDown,
        effects::Slider,
        effects::Slider,
        effects::Slider,
    ),
}

impl Fill {
    pub const TY: u8 = 21;
}
