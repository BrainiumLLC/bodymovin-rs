use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Shape {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "d")]
    pub direction: Option<f64>,
    pub closed: Option<bool>,
    #[serde(rename = "ks")]
    pub vertices: properties::Shape,
}
