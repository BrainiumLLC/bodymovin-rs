use crate::properties;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum StarType {
    Star = 1,
    Polygon = 2,
}

#[derive(Debug, Deserialize)]
pub struct Star {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "d")]
    pub direction: f64,
    #[serde(rename = "p")]
    pub position: properties::EitherMultiDimensional,
    #[serde(rename = "ir")]
    pub inner_radius: Option<properties::EitherValue>,
    #[serde(rename = "is")]
    pub inner_roundness: Option<properties::EitherValue>,
    #[serde(rename = "or")]
    pub outer_radius: properties::EitherValue,
    #[serde(rename = "os")]
    pub outer_roundness: properties::EitherValue,
    #[serde(rename = "r")]
    pub rotation: properties::EitherValue,
    #[serde(rename = "pt")]
    pub points: properties::EitherValue,
    #[serde(rename = "sy")]
    pub ty: StarType,
}

impl Star {
    pub const TY: &'static str = "sr";
}
