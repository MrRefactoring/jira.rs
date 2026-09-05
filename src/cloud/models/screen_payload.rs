// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Defines the payload for the field screens. See <https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-screens/#api-rest-api-3-screens-post>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScreenPayload {
    /// The description of the screen
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the screen
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
    /// The tabs of the screen. See <https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-screen-tab-fields/#api-rest-api-3-screens-screenid-tabs-tabid-fields-post>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tabs: Option<Vec<TabPayload>>,
}
