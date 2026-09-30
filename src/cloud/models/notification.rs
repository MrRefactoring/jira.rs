// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about a notification.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Notification {
    /// The HTML body of the email notification for the issue.
    #[serde(rename = "htmlBody", default, skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrict: Option<NotificationRecipientsRestrictions>,
    /// The subject of the email notification for the issue. If this is not specified, then the subject is set to the issue key and summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The plain text body of the email notification for the issue.
    #[serde(rename = "textBody", default, skip_serializing_if = "Option::is_none")]
    pub text_body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<NotificationRecipients>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for Notification {
    const FIELDS: &'static [&'static str] = &["htmlBody", "restrict", "subject", "textBody", "to"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
