use crate::{properties, util};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValueKeyframe {
    #[serde(rename = "s", deserialize_with = "properties::detuple")]
    pub start_value: f64,
    #[serde(rename = "t")]
    pub start_time: f64,
    #[serde(rename = "h", deserialize_with = "util::bool_from_int", default)]
    pub hold: bool,
    #[serde(flatten)]
    pub bezier: Option<properties::Bezier1d>,
}
