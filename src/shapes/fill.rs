use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Fill {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "o")]
    pub opacity: properties::EitherValue,
    #[serde(rename = "c")]
    pub color: properties::EitherMultiDimensional,
}
