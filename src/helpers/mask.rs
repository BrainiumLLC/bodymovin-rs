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
pub struct Mask {
    #[serde(rename = "inv")]
    pub inverted: bool,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "pt")]
    pub points: properties::Shape,
    #[serde(rename = "o", default = "properties::Scalar::hundred")]
    pub opacity: properties::Scalar,
    #[serde(default)]
    pub mode: MaskMode,
}
