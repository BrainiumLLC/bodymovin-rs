mod ellipse;
mod fill;
mod gradient_fill;
mod gradient_stroke;
mod group;
mod merge;
mod rect;
mod repeater;
mod rounded_corners;
mod shape;
mod star;
mod stroke;
mod transform;
mod trim;

pub use self::{
    ellipse::*, fill::*, gradient_fill::*, gradient_stroke::*, group::*, merge::*, rect::*,
    repeater::*, rounded_corners::*, shape::*, star::*, stroke::*, transform::*, trim::*,
};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum GradientType {
    Linear = 1,
    Radial = 2,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "ty")]
pub enum AnyShape {
    #[serde(rename = "sh")]
    Shape(Shape),
    #[serde(rename = "rc")]
    Rect(Rect),
    #[serde(rename = "el")]
    Ellipse(Ellipse),
    #[serde(rename = "sr")]
    Star(Star),
    #[serde(rename = "fl")]
    Fill(Fill),
    #[serde(rename = "gf")]
    GradientFill(GradientFill),
    #[serde(rename = "gs")]
    GradientStroke(GradientStroke),
    #[serde(rename = "st")]
    Stroke(Stroke),
    #[serde(rename = "mm")]
    Merge(Merge),
    #[serde(rename = "tm")]
    Trim(Trim),
    #[serde(rename = "gr")]
    Group(Group),
    #[serde(rename = "rd")]
    RoundedCorners(RoundedCorners),
    #[serde(rename = "tr")]
    Transform(Transform),
}
