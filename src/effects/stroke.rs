use crate::effects;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Stroke {
    #[serde(rename = "ix")]
    pub index: i64,
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "ef")]
    pub effects: (
        effects::Color,
        effects::CheckBox,
        effects::CheckBox,
        effects::Color,
        effects::Slider,
        effects::Slider,
        effects::Slider,
        effects::Slider,
        effects::Slider,
        effects::DropDown,
        effects::DropDown,
    ),
}

impl Stroke {
    pub const TY: u8 = 22;
}
