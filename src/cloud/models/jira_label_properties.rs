// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum JiraLabelPropertiesColor {
        GreyLightest => "GREY_LIGHTEST",
        GreyLighter => "GREY_LIGHTER",
        Grey => "GREY",
        GreyDarker => "GREY_DARKER",
        GreyDarkest => "GREY_DARKEST",
        PurpleLightest => "PURPLE_LIGHTEST",
        PurpleLighter => "PURPLE_LIGHTER",
        Purple => "PURPLE",
        PurpleDarker => "PURPLE_DARKER",
        PurpleDarkest => "PURPLE_DARKEST",
        BlueLightest => "BLUE_LIGHTEST",
        BlueLighter => "BLUE_LIGHTER",
        Blue => "BLUE",
        BlueDarker => "BLUE_DARKER",
        BlueDarkest => "BLUE_DARKEST",
        TealLightest => "TEAL_LIGHTEST",
        TealLighter => "TEAL_LIGHTER",
        Teal => "TEAL",
        TealDarker => "TEAL_DARKER",
        TealDarkest => "TEAL_DARKEST",
        GreenLightest => "GREEN_LIGHTEST",
        GreenLighter => "GREEN_LIGHTER",
        Green => "GREEN",
        GreenDarker => "GREEN_DARKER",
        GreenDarkest => "GREEN_DARKEST",
        LimeLightest => "LIME_LIGHTEST",
        LimeLighter => "LIME_LIGHTER",
        Lime => "LIME",
        LimeDarker => "LIME_DARKER",
        LimeDarkest => "LIME_DARKEST",
        YellowLightest => "YELLOW_LIGHTEST",
        YellowLighter => "YELLOW_LIGHTER",
        Yellow => "YELLOW",
        YellowDarker => "YELLOW_DARKER",
        YellowDarkest => "YELLOW_DARKEST",
        OrangeLightest => "ORANGE_LIGHTEST",
        OrangeLighter => "ORANGE_LIGHTER",
        Orange => "ORANGE",
        OrangeDarker => "ORANGE_DARKER",
        OrangeDarkest => "ORANGE_DARKEST",
        RedLightest => "RED_LIGHTEST",
        RedLighter => "RED_LIGHTER",
        Red => "RED",
        RedDarker => "RED_DARKER",
        RedDarkest => "RED_DARKEST",
        MagentaLightest => "MAGENTA_LIGHTEST",
        MagentaLighter => "MAGENTA_LIGHTER",
        Magenta => "MAGENTA",
        MagentaDarker => "MAGENTA_DARKER",
        MagentaDarkest => "MAGENTA_DARKEST",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraLabelProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<JiraLabelPropertiesColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
