use crate::layers;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PreComp {
    pub id: String,
    #[serde(default)]
    pub layers: Vec<layers::AnyLayer>,
}
