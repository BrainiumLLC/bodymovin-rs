use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Angle {
    #[serde(rename = "ix")]
    pub index: i64,
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "v")]
    pub value: properties::Scalar,
}

impl Angle {
    pub const TY: u8 = 1;
}
