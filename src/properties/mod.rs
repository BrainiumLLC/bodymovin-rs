mod double_keyframe;
mod multi_dimensional;
mod scalar;
mod shape;

pub use self::{double_keyframe::*, multi_dimensional::*, scalar::*, shape::*};
use serde::{de::Deserializer, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Destructurer {
    Bare(f64),
    Tuple((f64,)),
    Array([f64; 1]),
}

impl Into<f64> for Destructurer {
    fn into(self) -> f64 {
        match self {
            Self::Bare(value) | Self::Tuple((value,)) | Self::Array([value]) => value,
        }
    }
}

fn destructure<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Destructurer::deserialize(deserializer).map(Into::into)
}

#[derive(Clone, Debug, Deserialize)]
pub struct ControlPoint1d {
    #[serde(deserialize_with = "destructure")]
    pub x: f64,
    #[serde(deserialize_with = "destructure")]
    pub y: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ControlPoint3d {
    pub x: [f64; 3],
    pub y: [f64; 3],
}

#[derive(Clone, Debug, Deserialize)]
pub struct Bezier1d {
    #[serde(rename = "i")]
    pub in_value: ControlPoint1d,
    #[serde(rename = "o")]
    pub out_value: ControlPoint1d,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Bezier3d {
    #[serde(rename = "i")]
    pub in_value: ControlPoint3d,
    #[serde(rename = "o")]
    pub out_value: ControlPoint3d,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpatialBezier {
    #[serde(rename = "ti")]
    pub in_value: Vec<f64>,
    #[serde(rename = "to")]
    pub out_value: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Value<F, A> {
    Fixed(F),
    Animated(Vec<A>),
}

impl<F, A> Default for Value<F, A>
where
    F: Default,
{
    fn default() -> Self {
        Self::Fixed(F::default())
    }
}

#[derive(Debug, Deserialize)]
pub struct Property<F, A> {
    #[serde(rename = "k")]
    pub value: Value<F, A>,
    #[serde(rename = "x")]
    pub expression: Option<String>,
    #[serde(rename = "ix")]
    pub index: Option<i64>,
}

impl<F, A> Default for Property<F, A>
where
    F: Default,
{
    fn default() -> Self {
        Self {
            value: Value::default(),
            expression: None,
            index: None,
        }
    }
}

impl<F, A> Property<F, A> {
    pub(crate) fn fixed(value: F) -> Self {
        Self {
            value: Value::Fixed(value),
            expression: None,
            index: None,
        }
    }
}
