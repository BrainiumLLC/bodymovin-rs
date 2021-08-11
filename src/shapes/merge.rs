use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Merge {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "mm")]
    pub merge_mode: i64,
}
