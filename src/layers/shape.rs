use crate::{effects, helpers, shapes, util};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Bounds {
    #[serde(rename = "l")]
    pub left: f64,
    #[serde(rename = "t")]
    pub top: f64,
    #[serde(rename = "b")]
    pub bottom: f64,
    #[serde(rename = "r")]
    pub right: f64,
}

#[derive(Debug, Deserialize)]
pub struct Shape {
    #[serde(rename = "ks")]
    pub transform: helpers::Transform,
    pub bounds: Option<Bounds>,
    #[serde(deserialize_with = "util::bool_from_int", default)]
    pub auto_orient: bool,
    #[serde(rename = "bm", default)]
    pub blend_mode: helpers::BlendMode,
    #[serde(rename = "ddd", deserialize_with = "util::bool_from_int", default)]
    pub is_3d: bool,
    #[serde(rename = "ind")]
    pub index: i64,
    #[serde(rename = "cl")]
    pub html_class: Option<String>,
    #[serde(rename = "ln")]
    pub html_id: Option<String>,
    #[serde(rename = "ip")]
    pub in_point: f64,
    #[serde(rename = "op")]
    pub out_point: f64,
    #[serde(rename = "st")]
    pub start_time: f64,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "masksProperties", default)]
    pub masks: Vec<helpers::Mask>,
    #[serde(rename = "ef", default)]
    pub effects: Vec<effects::Index>,
    #[serde(rename = "sr", default = "util::one_please")]
    pub stretch: f64,
    #[serde(rename = "parent")]
    pub parent: Option<i64>,
    #[serde(default)]
    pub shapes: Vec<shapes::AnyShape>,
}
