// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`.
    pub enum DashboardGadgetColor {
        Blue => "blue",
        Red => "red",
        Yellow => "yellow",
        Green => "green",
        Cyan => "cyan",
        Purple => "purple",
        Gray => "gray",
        White => "white",
    }
}

/// Details of a gadget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardGadget {
    /// The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`.
    pub color: DashboardGadgetColor,
    /// The ID of the gadget instance.
    pub id: i64,
    /// The module key of the gadget type.
    #[serde(rename = "moduleKey", default, skip_serializing_if = "Option::is_none")]
    pub module_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<DashboardGadgetPosition>,
    /// The title of the gadget.
    pub title: String,
    /// The URI of the gadget type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
