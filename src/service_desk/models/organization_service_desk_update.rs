// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OrganizationServiceDeskUpdate {
    /// List of organizations, specified by 'ID' field values, to add to or remove from the service desk.
    #[serde(rename = "organizationId")]
    pub organization_id: i64,
    /// Service desk Id for which, organization needs to be updated
    #[serde(rename = "serviceDeskId", default, skip_serializing_if = "Option::is_none")]
    pub service_desk_id: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
