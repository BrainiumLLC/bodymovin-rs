use super::Value;
use crate::{properties, util};
use serde::Deserialize;

pub type MultiDimensionalValue = Vec<properties::ScalarValue>;

#[derive(Debug, Deserialize)]
pub struct MultiDimensionalKeyframe {
    #[serde(rename = "s")]
    pub start_value: Option<MultiDimensionalValue>,
    #[serde(rename = "e")]
    pub end_value: Option<MultiDimensionalValue>,
    #[serde(rename = "t")]
    pub start_time: f64,
    #[serde(rename = "h", deserialize_with = "util::bool_from_int", default)]
    pub hold: bool,
    #[serde(flatten)]
    pub bezier: Option<properties::Bezier2d>,
    #[serde(flatten)]
    pub spatial_bezier: Option<properties::SpatialBezier>,
}

impl MultiDimensionalKeyframe {
    pub fn scaled(self, scale: &Vec<f64>) -> Self {
        let start_value = self.start_value.as_ref().map(|val| {
            val.iter()
                .zip(scale.iter())
                .map(|(val, scale)| val * scale)
                .collect::<Vec<_>>()
        });

        let end_value = self.end_value.as_ref().map(|val| {
            val.iter()
                .zip(scale.iter())
                .map(|(val, scale)| val * scale)
                .collect::<Vec<_>>()
        });

        let spatial_bezier = if let Some(bezier) = &self.spatial_bezier {
            Some(bezier.scaled(scale))
        } else {
            None
        };

        Self {
            start_value,
            end_value,
            start_time: self.start_time,
            hold: self.hold,
            bezier: self.bezier,
            spatial_bezier,
        }
    }
}

pub type MultiDimensional = properties::Property<MultiDimensionalValue, MultiDimensionalKeyframe>;

impl MultiDimensional {
    pub(crate) fn zero() -> Self {
        Self::fixed(vec![0.0; 2])
    }

    pub(crate) fn hundred() -> Self {
        Self::fixed(vec![100.0; 2])
    }

    pub fn scaled(self, scale: &Vec<f64>) -> Self {
        Self {
            value: match self.value {
                Value::Fixed(val) => Value::Fixed(
                    val.iter()
                        .zip(scale.iter())
                        .map(|(val, scale)| val * scale)
                        .collect::<Vec<_>>(),
                ),
                Value::Animated(vals) => {
                    Value::Animated(vals.into_iter().map(|val| val.scaled(scale)).collect())
                }
            },
            expression: self.expression,
            index: self.index,
        }
    }
}
