// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequestTypeCreate {
    /// Description of the request type on the service desk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Help text for the request type on the service desk.
    #[serde(rename = "helpText", default, skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
    /// ID of the request type to add to the service desk.
    #[serde(rename = "issueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
    /// Name of the request type on the service desk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
