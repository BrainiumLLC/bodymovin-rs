use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MultiDimensionalKeyframed {
    #[serde(rename = "k")]
    pub keyframes: Vec<properties::OffsetKeyframe>,
    #[serde(rename = "x")]
    pub expression: Option<String>,
    #[serde(rename = "ix")]
    pub index: i64,
}
