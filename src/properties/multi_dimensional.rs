use crate::{properties, util};
use serde::Deserialize;

pub type MultiDimensionalValue = Vec<properties::ScalarValue>;

#[derive(Debug, Deserialize)]
pub struct MultiDimensionalKeyframe {
    #[serde(rename = "s")]
    pub start_value: Option<MultiDimensionalValue>,
    #[serde(rename = "t")]
    pub start_time: f64,
    #[serde(rename = "h", deserialize_with = "util::bool_from_int", default)]
    pub hold: bool,
    #[serde(flatten)]
    pub bezier: Option<properties::Bezier1d>,
    #[serde(flatten)]
    pub spatial_bezier: Option<properties::SpatialBezier>,
}

pub type MultiDimensional = properties::Property<MultiDimensionalValue, MultiDimensionalKeyframe>;

impl MultiDimensional {
    pub(crate) fn zero() -> Self {
        Self::fixed(vec![0.0; 3])
    }

    pub(crate) fn hundred() -> Self {
        Self::fixed(vec![100.0; 3])
    }
}
