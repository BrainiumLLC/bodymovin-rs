use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum MaskMode {
    #[serde(rename = "n")]
    None,
    #[serde(rename = "a")]
    Additive,
    #[serde(rename = "s")]
    Subtract,
    #[serde(rename = "i")]
    Intersect,
    #[serde(rename = "l")]
    Lighten,
    #[serde(rename = "d")]
    Darken,
    #[serde(rename = "f")]
    Difference,
}

impl Default for MaskMode {
    fn default() -> Self {
        Self::Additive
    }
}

#[derive(Debug, Deserialize)]
pub enum MaskVertices {
    Shape(properties::Shape),
    ShapeKeyframed(properties::ShapeKeyframed),
}

#[derive(Debug, Deserialize)]
pub struct Mask {
    #[serde(rename = "inv")]
    pub inverted: bool,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "pt")]
    pub points: MaskVertices,
    #[serde(rename = "o", default = "properties::EitherValue::hundred")]
    pub opacity: properties::EitherValue,
    #[serde(default)]
    pub mode: MaskMode,
}
