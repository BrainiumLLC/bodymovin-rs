mod chars;
mod image;
mod pre_comp;

pub use self::{chars::*, image::*, pre_comp::*};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Asset {
    Image(Image),
    PreComp(PreComp),
}
