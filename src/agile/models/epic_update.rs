// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum EpicUpdateColorKey {
        Color1 => "color_1",
        Color2 => "color_2",
        Color3 => "color_3",
        Color4 => "color_4",
        Color5 => "color_5",
        Color6 => "color_6",
        Color7 => "color_7",
        Color8 => "color_8",
        Color9 => "color_9",
        Color10 => "color_10",
        Color11 => "color_11",
        Color12 => "color_12",
        Color13 => "color_13",
        Color14 => "color_14",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EpicUpdateColor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<EpicUpdateColorKey>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EpicUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<EpicUpdateColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
