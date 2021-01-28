use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Merge {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "mm")]
    pub merge_mode: i64,
}
