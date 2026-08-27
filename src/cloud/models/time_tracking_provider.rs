// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about the time tracking provider.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeTrackingProvider {
    /// The key for the time tracking provider. For example, *JIRA*.
    pub key: String,
    /// The name of the time tracking provider. For example, *JIRA provided time tracking*.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the configuration page for the time tracking provider app. For example, */example/config/url*. This property is only returned if the `adminPageKey` property is set in the module descriptor of the time tracking provider app.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
