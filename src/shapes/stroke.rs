use crate::{helpers, properties};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Stroke {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "lc")]
    pub line_cap: helpers::LineCap,
    #[serde(rename = "lj")]
    pub line_join: helpers::LineJoin,
    #[serde(rename = "ml")]
    pub miter_limit: Option<f64>,
    #[serde(rename = "o")]
    pub opacity: properties::EitherValue,
    #[serde(rename = "w")]
    pub width: properties::EitherValue,
    #[serde(rename = "c")]
    pub color: properties::EitherMultiDimensional,
}
