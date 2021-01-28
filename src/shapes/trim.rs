use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Trim {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "s")]
    pub start: properties::EitherValue,
    #[serde(rename = "e")]
    pub end: properties::EitherValue,
    #[serde(rename = "o")]
    pub offset: properties::EitherValue,
}
