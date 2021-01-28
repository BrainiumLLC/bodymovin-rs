use crate::{properties, shapes};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GradientFill {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "o")]
    pub opacity: properties::EitherValue,
    #[serde(rename = "s")]
    pub start_point: properties::EitherMultiDimensional,
    #[serde(rename = "e")]
    pub end_point: properties::EitherMultiDimensional,
    #[serde(rename = "t")]
    pub ty: shapes::GradientType,
    #[serde(rename = "h")]
    pub highlight_length: Option<properties::EitherValue>,
    #[serde(rename = "a")]
    pub highlight_angle: Option<properties::EitherValue>,
    #[serde(rename = "g")]
    pub gradient_colors: serde_json::Value,
}
