use serde::de::{self, Deserialize as _, Deserializer, Unexpected};

pub fn type_name<T>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.split("::").last().unwrap_or_else(|| name)
}

pub const fn one_please() -> f64 {
    1.0
}

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
