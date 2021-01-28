use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DropDown {
    #[serde(rename = "ix")]
    pub index: i64,
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "v")]
    pub value: properties::EitherValue,
}

impl DropDown {
    pub const TY: u8 = 7;
}
