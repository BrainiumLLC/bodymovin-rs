use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProLevels {
    #[serde(rename = "ix")]
    pub index: i64,
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    // #[serde(rename = "ef")]
    // pub effects: (
    //     effects::DropDown,
    //     effects::NoValue,
    //     effects::NoValue,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::NoValue,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::NoValue,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::NoValue,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::NoValue,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    //     effects::Slider,
    // ),
}

impl ProLevels {
    pub const TY: u8 = 23;
}
