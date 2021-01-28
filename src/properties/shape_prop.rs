use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ShapeProp {
    #[serde(rename = "c")]
    pub closed: bool,
    #[serde(rename = "i")]
    pub in_point: (f64, f64),
    #[serde(rename = "o")]
    pub out_point: (f64, f64),
    #[serde(rename = "v")]
    pub vertices: Vec<(f64, f64)>,
}
