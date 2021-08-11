use crate::{properties, shapes};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GradientFill {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "o")]
    pub opacity: properties::Scalar,
    #[serde(rename = "s")]
    pub start_point: properties::MultiDimensional,
    #[serde(rename = "e")]
    pub end_point: properties::MultiDimensional,
    #[serde(rename = "t")]
    pub ty: shapes::GradientType,
    #[serde(rename = "h")]
    pub highlight_length: Option<properties::Scalar>,
    #[serde(rename = "a")]
    pub highlight_angle: Option<properties::Scalar>,
    #[serde(rename = "g")]
    pub gradient_colors: serde_json::Value,
}
