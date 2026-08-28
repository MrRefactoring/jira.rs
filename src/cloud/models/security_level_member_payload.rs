// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type of the security level member
    pub enum SecurityLevelMemberPayloadType {
        Group => "group",
        Reporter => "reporter",
        Users => "users",
    }
}

/// The payload for creating a security level member. See <https://support.atlassian.com/jira-cloud-administration/docs/configure-issue-security-schemes/>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityLevelMemberPayload {
    /// Defines the value associated with the type. For reporter this would be {"null"}; for users this would be the names of specific users); for group this would be group names like {"administrators", "jira-administrators", "jira-users"}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// The type of the security level member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SecurityLevelMemberPayloadType>,
}
