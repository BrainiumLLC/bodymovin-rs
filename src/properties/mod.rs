mod double_keyframe;
mod multi_dimensional;
mod multi_dimensional_keyframed;
mod offset_keyframe;
mod shape;
mod shape_keyframed;
mod shape_prop;
mod shape_prop_keyframe;
mod value;
mod value_keyframe;
mod value_keyframed;

pub use self::{
    double_keyframe::*, multi_dimensional::*, multi_dimensional_keyframed::*, offset_keyframe::*,
    shape::*, shape_keyframed::*, shape_prop::*, shape_prop_keyframe::*, value::*,
    value_keyframe::*, value_keyframed::*,
};
use crate::util;
use serde::{de::Deserializer, Deserialize};

fn detuple<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    <(f64,)>::deserialize(deserializer).map(|(n,)| n)
}

fn dearray<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    <[f64; 1]>::deserialize(deserializer).map(|[n]| n)
}

fn try_dearray<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    if let Some(value) = value.as_f64() {
        Ok(value)
    } else {
        dearray(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ControlPoint1d {
    #[serde(deserialize_with = "try_dearray")]
    pub x: f64,
    #[serde(deserialize_with = "try_dearray")]
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

#[derive(Debug)]
pub enum MaybeAnimated<F, A>
where
    F: for<'r> Deserialize<'r>,
    A: for<'r> Deserialize<'r>,
{
    Fixed(F),
    Animated(A),
}

impl<'de, F, A> Deserialize<'de> for MaybeAnimated<F, A>
where
    F: for<'r> Deserialize<'r>,
    A: for<'r> Deserialize<'r>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // lol https://github.com/serde-rs/serde/pull/1392
        use serde::__private::de::{ContentDeserializer, TaggedContentVisitor};
        use serde_repr::Deserialize_repr;

        #[derive(Debug, Deserialize_repr)]
        #[repr(u8)]
        enum MaybeAnimatedTag {
            Fixed = 0,
            Animated = 1,
        }

        let tagged =
            deserializer.deserialize_any(TaggedContentVisitor::<MaybeAnimatedTag>::new(
                "a",
                "internally tagged enum MaybeAnimated",
            ))?;
        match tagged.tag {
            MaybeAnimatedTag::Fixed => {
                log::info!("parsing `{}`", util::type_name::<F>());
                F::deserialize(ContentDeserializer::<D::Error>::new(tagged.content))
                    .map(MaybeAnimated::Fixed)
            }
            MaybeAnimatedTag::Animated => {
                log::info!("parsing `{}`", util::type_name::<A>());
                A::deserialize(ContentDeserializer::<D::Error>::new(tagged.content))
                    .map(MaybeAnimated::Animated)
            }
        }
    }
}

pub type EitherMultiDimensional = MaybeAnimated<MultiDimensional, MultiDimensionalKeyframed>;

impl EitherMultiDimensional {
    pub(crate) fn fixed(value: Vec<f64>) -> Self {
        Self::Fixed(MultiDimensional {
            value,
            ..Default::default()
        })
    }

    pub(crate) fn zero() -> Self {
        Self::fixed(vec![0.0; 3])
    }

    pub(crate) fn hundred() -> Self {
        Self::fixed(vec![100.0; 3])
    }
}

pub type EitherShape = MaybeAnimated<Shape, ShapeKeyframed>;

pub type EitherValue = MaybeAnimated<Value, ValueKeyframed>;

impl EitherValue {
    pub(crate) fn fixed(value: f64) -> Self {
        Self::Fixed(Value {
            value,
            ..Default::default()
        })
    }

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
