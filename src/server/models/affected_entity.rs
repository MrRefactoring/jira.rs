// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum AffectedEntityType {
        Anonymize => "ANONYMIZE",
        TransferOwnership => "TRANSFER_OWNERSHIP",
        Remove => "REMOVE",
        Manual => "MANUAL",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AffectedEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "numberOfOccurrences", default, skip_serializing_if = "Option::is_none")]
    pub number_of_occurrences: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AffectedEntityType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "uriDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub uri_display_name: Option<String>,
}
