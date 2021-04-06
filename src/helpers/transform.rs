use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transform {
    #[serde(rename = "a", default = "properties::EitherMultiDimensional::zero")]
    pub anchor_point: properties::EitherMultiDimensional,
    #[serde(rename = "p", default = "properties::EitherMultiDimensional::zero")]
    pub position: properties::EitherMultiDimensional,
    #[serde(rename = "s", default = "properties::EitherMultiDimensional::hundred")]
    pub scale: properties::EitherMultiDimensional,
    #[serde(rename = "r", default = "properties::EitherValue::zero")]
    pub rotation: properties::EitherValue,
    #[serde(rename = "o", default = "properties::EitherValue::hundred")]
    pub opacity: properties::EitherValue,
    #[serde(rename = "px", default = "properties::EitherValue::zero")]
    pub position_x: properties::EitherValue,
    #[serde(rename = "py", default = "properties::EitherValue::zero")]
    pub position_y: properties::EitherValue,
    #[serde(rename = "pz", default = "properties::EitherValue::zero")]
    pub position_z: properties::EitherValue,
    #[serde(rename = "sk", default = "properties::EitherValue::zero")]
    pub skew: properties::EitherValue,
    #[serde(rename = "sa", default = "properties::EitherValue::zero")]
    pub skew_axis: properties::EitherValue,
}
