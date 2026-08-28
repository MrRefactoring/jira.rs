// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServiceDeskCustomerInvite {
    /// Customer's name for display in the UI.
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Customer's email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
