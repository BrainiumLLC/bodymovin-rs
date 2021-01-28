use crate::effects;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Index {
    Slider(effects::Slider),
    Angle(effects::Angle),
    Color(effects::Color),
    Point(effects::Point),
    CheckBox(effects::CheckBox),
    Group(effects::Group),
    NoValue(effects::NoValue),
    DropDown(effects::DropDown),
    CustomValue(effects::CustomValue),
    Layer(effects::Layer),
    Tint(effects::Tint),
    Fill(effects::Fill),
    Stroke(effects::Stroke),
    Tritone(effects::Tritone),
    ProLevels(effects::ProLevels),
}
