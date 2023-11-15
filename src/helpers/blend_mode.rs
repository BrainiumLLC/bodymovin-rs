use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum BlendMode {
    Normal = 0,
    Multiply = 1,
    Screen = 2,
    Overlay = 3,
    Darken = 4,
    Lighten = 5,
    ColorDodge = 6,
    ColorBurn = 7,
    HardLight = 8,
    SoftLight = 9,
    Difference = 10,
    Exclusion = 11,
    Hue = 12,
    Saturation = 13,
    Color = 14,
    Luminosity = 15,
    Add = 16,
    HardMix = 17,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Normal
    }
}
