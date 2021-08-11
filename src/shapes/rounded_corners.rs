use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RoundedCorners {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "r")]
    pub radius: properties::Scalar,
}
