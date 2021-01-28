use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rect {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "d")]
    pub direction: f64,
    #[serde(rename = "p")]
    pub position: properties::EitherMultiDimensional,
    #[serde(rename = "s")]
    pub size: properties::EitherMultiDimensional,
    #[serde(rename = "r")]
    pub rounded_corners: properties::EitherValue,
}
