use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Fill {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "fillEnabled")]
    pub fill_enabled: Option<bool>,
    #[serde(rename = "o")]
    pub opacity: properties::Scalar,
    #[serde(rename = "c")]
    pub color: properties::MultiDimensional,
}
