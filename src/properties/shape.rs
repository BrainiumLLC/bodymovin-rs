use crate::{properties, util};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ShapeValue {
    #[serde(rename = "c")]
    pub closed: Option<bool>,
    #[serde(rename = "i")]
    pub in_point: Vec<(f64, f64)>,
    #[serde(rename = "o")]
    pub out_point: Vec<(f64, f64)>,
    #[serde(rename = "v")]
    pub vertices: Vec<(f64, f64)>,
}

#[derive(Debug, Deserialize)]
pub struct ShapeKeyframe {
    #[serde(rename = "s")]
    pub start_value: Option<Vec<ShapeValue>>,
    #[serde(rename = "e")]
    pub end_value: Option<Vec<ShapeValue>>,
    #[serde(rename = "t")]
    pub start_time: f64,
    #[serde(rename = "h", deserialize_with = "util::bool_from_int", default)]
    pub hold: bool,
    #[serde(flatten)]
    pub bezier: Option<properties::Bezier1d>,
    #[serde(flatten)]
    pub spatial_bezier: Option<properties::SpatialBezier>,
}

pub type Shape = properties::Property<ShapeValue, ShapeKeyframe>;
