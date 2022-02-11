use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TextData {
    // TODO: everything
}

#[derive(Debug, Deserialize)]
pub struct TextMixin {
    #[serde(rename = "t")]
    pub text: TextData,
}

pub type Text = super::Layer<TextMixin>;
