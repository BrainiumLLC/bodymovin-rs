use crate::{properties, util};
use serde::Deserialize;

pub type ScalarValue = f64;

#[derive(Debug, Deserialize)]
pub struct ScalarKeyframe {
    #[serde(rename = "s", deserialize_with = "properties::option_destructure")]
    pub start_value: Option<ScalarValue>,
    #[serde(rename = "t")]
    pub start_time: f64,
    #[serde(rename = "h", deserialize_with = "util::bool_from_int", default)]
    pub hold: bool,
    #[serde(flatten)]
    pub bezier: Option<properties::Bezier1d>,
}

pub type Scalar = properties::Property<ScalarValue, ScalarKeyframe>;

impl Scalar {
    pub(crate) fn zero() -> Self {
        Self::fixed(0.0)
    }

    pub(crate) fn one() -> Self {
        Self::fixed(1.0)
    }

    pub(crate) fn hundred() -> Self {
        Self::fixed(100.0)
    }
}
