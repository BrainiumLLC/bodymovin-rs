use crate::{helpers, properties};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Repeater {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "c", default = "properties::Scalar::one")]
    pub copies: properties::Scalar,
    #[serde(rename = "o", default = "properties::Scalar::zero")]
    pub offset: properties::Scalar,
    #[serde(rename = "m", default)]
    pub composite: helpers::Composite,
    #[serde(rename = "tr")]
    pub tr: helpers::Transform,
}
