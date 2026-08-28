// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Configuration of the announcement banner.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AnnouncementBannerConfigurationUpdate {
    /// Flag indicating if the announcement banner can be dismissed by the user.
    #[serde(rename = "isDismissible", default, skip_serializing_if = "Option::is_none")]
    pub is_dismissible: Option<bool>,
    /// Flag indicating if the announcement banner is enabled or not.
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// The text on the announcement banner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Visibility of the announcement banner. Can be public or private.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}
