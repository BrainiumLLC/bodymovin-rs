use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum LineJoin {
    Miter = 1,
    Round = 2,
    Bevel = 3,
}

impl Default for LineJoin {
    fn default() -> Self {
        Self::Round
    }
}
