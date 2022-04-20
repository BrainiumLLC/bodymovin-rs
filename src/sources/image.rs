use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Image {
    pub id: String,
    #[serde(rename = "w")]
    pub width: u64,
    #[serde(rename = "h")]
    pub height: u64,
    #[serde(rename = "u")]
    pub path: String,
    #[serde(rename = "p")]
    pub name: String,
}
