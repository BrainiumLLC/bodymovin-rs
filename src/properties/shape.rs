use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Shape {
    #[serde(rename = "k")]
    pub value: properties::ShapeProp,
    #[serde(rename = "x")]
    pub expression: Option<String>,
    #[serde(rename = "ix")]
    pub index: i64,
}
