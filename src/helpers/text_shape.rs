use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum TextShape {
    Square = 1,
    RampUp = 2,
    RampDown = 3,
    Triangle = 4,
    Round = 5,
    Smooth = 6,
}

impl Default for TextShape {
    fn default() -> Self {
        Self::Square
    }
}
