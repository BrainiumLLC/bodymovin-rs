use crate::shapes;
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
pub struct ShapeMixin {
    pub bounds: Option<Bounds>,
    #[serde(default)]
    pub shapes: Vec<shapes::AnyShape>,
}

pub type Shape = super::Layer<ShapeMixin>;
