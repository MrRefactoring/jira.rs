// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookStatisticsCountsWindow {
    /// Epoch milliseconds the window opened at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    /// How long the window is, in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookStatisticsCounts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<WebhookStatisticsCountsWindow>,
}

/// How a webhook has been delivering, over the window the instance keeps.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counts: Option<WebhookStatisticsCounts>,
    /// The most recent delivery failure, absent while there has been none.
    #[serde(rename = "lastError", default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<std::collections::HashMap<String, serde_json::Value>>,
}
