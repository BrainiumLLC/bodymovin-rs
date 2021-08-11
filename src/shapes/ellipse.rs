use crate::{properties, util};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ellipse {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "d", default = "util::one_please")]
    pub direction: f64,
    #[serde(rename = "p")]
    pub position: properties::MultiDimensional,
    #[serde(rename = "s")]
    pub size: properties::MultiDimensional,
}
