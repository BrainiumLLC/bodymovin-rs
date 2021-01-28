use crate::shapes;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Group {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "np")]
    pub number_of_properties: i64,
    #[serde(rename = "it")]
    pub items: Vec<shapes::AnyShape>,
}
