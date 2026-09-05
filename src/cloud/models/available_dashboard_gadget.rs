// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The details of the available dashboard gadget.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AvailableDashboardGadget {
    /// The module key of the gadget type.
    #[serde(rename = "moduleKey", default, skip_serializing_if = "Option::is_none")]
    pub module_key: Option<String>,
    /// The title of the gadget.
    pub title: String,
    /// The URI of the gadget type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
