use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum TextBased {
    Characters = 1,
    CharactersExcludingSpaces = 2,
    Words = 3,
    Lines = 4,
}

impl Default for TextBased {
    fn default() -> Self {
        Self::Characters
    }
}
