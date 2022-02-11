use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ImageMixin {
    #[serde(rename = "refId")]
    pub ref_id: String,
}

pub type Image = super::Layer<ImageMixin>;
