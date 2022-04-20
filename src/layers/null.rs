use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NullMixin {}

pub type Null = super::Layer<NullMixin>;
