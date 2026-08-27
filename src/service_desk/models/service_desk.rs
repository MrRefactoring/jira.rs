// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServiceDesk {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<SelfLink>,
    /// ID of the service desk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// ID of the peer project for the service desk.
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Key of the peer project of the service desk.
    #[serde(rename = "projectKey", default, skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    /// Name of the project and service desk.
    #[serde(rename = "projectName", default, skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    /// Key of the project type.
    #[serde(rename = "projectTypeKey", default, skip_serializing_if = "Option::is_none")]
    pub project_type_key: Option<String>,
}
