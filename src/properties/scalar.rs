use crate::{properties, util};
use serde::Deserialize;

pub type ScalarValue = f64;

#[derive(Clone, Copy, Debug, Deserialize)]
#[repr(transparent)]
pub struct DestructuredScalarValue(
    #[serde(deserialize_with = "properties::destructure")] pub ScalarValue,
);

impl Into<ScalarValue> for DestructuredScalarValue {
    fn into(self) -> ScalarValue {
        self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct ScalarKeyframe {
    #[serde(rename = "s")]
    pub start_value: Option<DestructuredScalarValue>,
    #[serde(rename = "e")]
    pub end_value: Option<DestructuredScalarValue>,
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
