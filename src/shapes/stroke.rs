use crate::{helpers, properties};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Stroke {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "lc")]
    pub line_cap: helpers::LineCap,
    #[serde(rename = "lj")]
    pub line_join: helpers::LineJoin,
    #[serde(rename = "ml")]
    pub miter_limit: Option<f64>,
    #[serde(rename = "o")]
    pub opacity: properties::Scalar,
    #[serde(rename = "w")]
    pub width: properties::Scalar,
    #[serde(rename = "c")]
    pub color: properties::MultiDimensional,
}
