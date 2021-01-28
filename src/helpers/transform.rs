use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transform {
    #[serde(rename = "a", default = "properties::EitherMultiDimensional::zero")]
    anchor_point: properties::EitherMultiDimensional,
    #[serde(rename = "p", default = "properties::EitherMultiDimensional::zero")]
    position: properties::EitherMultiDimensional,
    #[serde(rename = "s", default = "properties::EitherMultiDimensional::hundred")]
    scale: properties::EitherMultiDimensional,
    #[serde(rename = "r", default = "properties::EitherValue::zero")]
    rotation: properties::EitherValue,
    #[serde(rename = "o", default = "properties::EitherValue::hundred")]
    opacity: properties::EitherValue,
    #[serde(rename = "px", default = "properties::EitherValue::zero")]
    position_x: properties::EitherValue,
    #[serde(rename = "py", default = "properties::EitherValue::zero")]
    position_y: properties::EitherValue,
    #[serde(rename = "pz", default = "properties::EitherValue::zero")]
    position_z: properties::EitherValue,
    #[serde(rename = "sk", default = "properties::EitherValue::zero")]
    skew: properties::EitherValue,
    #[serde(rename = "sa", default = "properties::EitherValue::zero")]
    skew_axis: properties::EitherValue,
}
