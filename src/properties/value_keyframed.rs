use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValueKeyframed {
    #[serde(rename = "k")]
    pub keyframes: Vec<properties::ValueKeyframe>,
    #[serde(rename = "x")]
    pub expression: Option<String>,
    #[serde(rename = "ix")]
    pub index: i64,
}
