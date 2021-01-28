use crate::{properties, util};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ellipse {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "d", default = "util::one_please")]
    pub direction: f64,
    #[serde(rename = "p")]
    pub position: properties::EitherMultiDimensional,
    #[serde(rename = "s")]
    pub size: properties::EitherMultiDimensional,
}
