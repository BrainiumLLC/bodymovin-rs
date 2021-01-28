use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct MultiDimensional {
    #[serde(rename = "k")]
    pub value: Vec<f64>,
    #[serde(rename = "x")]
    pub expression: Option<String>,
    #[serde(rename = "ix")]
    pub index: Option<i64>,
}
