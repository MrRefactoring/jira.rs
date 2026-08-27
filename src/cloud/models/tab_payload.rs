// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Defines the payload for the tabs of the screen. See <https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-screen-tab-fields/\#api-rest-api-3-screens-screenid-tabs-tabid-fields-post>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TabPayload {
    /// The list of resource identifier of the field associated to the tab. See <https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-screen-tab-fields/\#api-rest-api-3-screens-screenid-tabs-tabid-fields-post>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ProjectCreateResourceIdentifier>>,
    /// The name of the tab
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
