use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Color {
    #[serde(rename = "ix")]
    pub index: i64,
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "v")]
    pub value: properties::MultiDimensional,
}

impl Color {
    pub const TY: u8 = 2;
}
