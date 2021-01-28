use crate::{helpers, properties};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Repeater {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "c", default = "properties::EitherValue::one")]
    pub copies: properties::EitherValue,
    #[serde(rename = "o", default = "properties::EitherValue::zero")]
    pub offset: properties::EitherValue,
    #[serde(rename = "m", default)]
    pub composite: helpers::Composite,
    #[serde(rename = "tr")]
    pub tr: helpers::Transform,
}
