// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// An icon.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IconBean {
    /// The URL of the tooltip, used only for a status icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The title of the icon, for use as a tooltip on the icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL of a 16x16 pixel icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url16x16: Option<String>,
}
