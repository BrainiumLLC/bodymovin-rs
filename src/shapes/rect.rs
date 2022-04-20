use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rect {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "d")]
    pub direction: f64,
    #[serde(rename = "p")]
    pub position: properties::MultiDimensional,
    #[serde(rename = "s")]
    pub size: properties::MultiDimensional,
    #[serde(rename = "r")]
    pub rounded_corners: properties::Scalar,
}
