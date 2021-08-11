use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Trim {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "s")]
    pub start: properties::Scalar,
    #[serde(rename = "e")]
    pub end: properties::Scalar,
    #[serde(rename = "o")]
    pub offset: properties::Scalar,
}
