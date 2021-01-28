use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum TextGrouping {
    Characters = 1,
    Word = 2,
    Line = 3,
    All = 4,
}

impl Default for TextGrouping {
    fn default() -> Self {
        Self::Characters
    }
}
