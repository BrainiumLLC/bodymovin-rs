pub mod effects;
pub mod helpers;
pub mod layers;
pub mod properties;
pub mod shapes;
mod util;

use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    LoadFailed(#[from] std::io::Error),
    #[error(transparent)]
    ParseFailed(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub struct Bodymovin {
    #[serde(rename = "ip")]
    pub in_point: f64,
    #[serde(rename = "op")]
    pub out_point: f64,
    #[serde(rename = "fr")]
    pub frame_rate: f64,
    #[serde(rename = "w")]
    pub width: i64,
    #[serde(rename = "ddd", deserialize_with = "util::bool_from_int", default)]
    pub is_3d: bool,
    #[serde(rename = "h")]
    pub height: i64,
    #[serde(rename = "v")]
    pub version: String,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(default)]
    pub layers: Vec<layers::Layer>,
    // pub assets: Vec<Asset>,
    // pub chars: Vec<Char>,
}

impl Bodymovin {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        Self::from_bytes(std::fs::read(path)?)
    }

    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        serde_json::from_slice(bytes.as_ref()).map_err(Error::from)
    }
}
