use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transform {
    #[serde(rename = "a", default = "properties::MultiDimensional::zero")]
    pub anchor_point: properties::MultiDimensional,
    #[serde(rename = "p", default = "properties::MultiDimensional::zero")]
    pub position: properties::MultiDimensional,
    #[serde(rename = "s", default = "properties::MultiDimensional::hundred")]
    pub scale: properties::MultiDimensional,
    #[serde(rename = "r", default = "properties::Scalar::zero")]
    pub rotation: properties::Scalar,
    #[serde(rename = "o", default = "properties::Scalar::hundred")]
    pub opacity: properties::Scalar,
    #[serde(rename = "px", default = "properties::Scalar::zero")]
    pub position_x: properties::Scalar,
    #[serde(rename = "py", default = "properties::Scalar::zero")]
    pub position_y: properties::Scalar,
    #[serde(rename = "pz", default = "properties::Scalar::zero")]
    pub position_z: properties::Scalar,
    #[serde(rename = "sk", default = "properties::Scalar::zero")]
    pub skew: properties::Scalar,
    #[serde(rename = "sa", default = "properties::Scalar::zero")]
    pub skew_axis: properties::Scalar,
}
