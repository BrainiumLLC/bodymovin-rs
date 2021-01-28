use crate::{properties, util};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OffsetKeyframe {
    #[serde(rename = "s")]
    pub start_value: Vec<f64>,
    #[serde(rename = "t")]
    pub start_time: f64,
    #[serde(rename = "h", deserialize_with = "util::bool_from_int", default)]
    pub hold: bool,
    #[serde(flatten)]
    pub bezier: Option<properties::Bezier1d>,
    #[serde(flatten)]
    pub spatial_bezier: Option<properties::SpatialBezier>,
}
