use crate::{properties, util};
use serde::Deserialize;

use super::Value;

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
    pub bezier: Option<properties::Bezier2d>,
}

impl ScalarKeyframe {
    pub fn scaled(self, scale: f64) -> Self {
        Self {
            start_value: self
                .start_value
                .as_ref()
                .map(|val| DestructuredScalarValue(val.0 * scale)),
            end_value: self
                .end_value
                .as_ref()
                .map(|val| DestructuredScalarValue(val.0 * scale)),
            start_time: self.start_time,
            hold: self.hold,
            bezier: self.bezier,
        }
    }
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

    pub fn scaled(self, scale: f64) -> Self {
        Self {
            value: match self.value {
                Value::Fixed(val) => Value::Fixed(val * scale),
                Value::Animated(vals) => {
                    Value::Animated(vals.into_iter().map(|val| val.scaled(scale)).collect())
                }
            },
            expression: self.expression,
            index: self.index,
        }
    }
}
