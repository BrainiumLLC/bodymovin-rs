use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transform {
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "a")]
    pub anchor_point: properties::EitherMultiDimensional,
    #[serde(rename = "p")]
    pub position: properties::EitherMultiDimensional,
    #[serde(rename = "s")]
    pub scale: properties::EitherMultiDimensional,
    #[serde(rename = "r")]
    pub rotation: properties::EitherValue,
    #[serde(rename = "o")]
    pub opacity: properties::EitherValue,
    #[serde(rename = "sk")]
    pub skew: properties::EitherValue,
    #[serde(rename = "sa")]
    pub skew_axis: properties::EitherValue,
}
