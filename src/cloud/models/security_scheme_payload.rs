// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The payload for creating a security scheme. See <https://support.atlassian.com/jira-cloud-administration/docs/configure-issue-security-schemes/>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SecuritySchemePayload {
    /// The description of the security scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the security scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
    /// The security levels for the security scheme
    #[serde(rename = "securityLevels", default, skip_serializing_if = "Option::is_none")]
    pub security_levels: Option<Vec<SecurityLevelPayload>>,
}
