use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ShapeKeyframed {
    #[serde(rename = "k")]
    pub keyframes: Vec<properties::ShapePropKeyframe>,
    #[serde(rename = "x")]
    pub expression: Option<String>,
    #[serde(rename = "ix")]
    pub index: i64,
}
