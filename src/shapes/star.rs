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
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "d")]
    pub direction: f64,
    #[serde(rename = "p")]
    pub position: properties::MultiDimensional,
    #[serde(rename = "ir")]
    pub inner_radius: Option<properties::Scalar>,
    #[serde(rename = "is")]
    pub inner_roundness: Option<properties::Scalar>,
    #[serde(rename = "or")]
    pub outer_radius: properties::Scalar,
    #[serde(rename = "os")]
    pub outer_roundness: properties::Scalar,
    #[serde(rename = "r")]
    pub rotation: properties::Scalar,
    #[serde(rename = "pt")]
    pub points: properties::Scalar,
    #[serde(rename = "sy")]
    pub ty: StarType,
}

impl Star {
    pub const TY: &'static str = "sr";
}
