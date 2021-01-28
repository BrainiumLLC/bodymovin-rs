use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Shape {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "d")]
    pub direction: f64,
    #[serde(rename = "ks")]
    pub vertices: properties::EitherShape,
}
