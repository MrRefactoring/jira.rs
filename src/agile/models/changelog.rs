// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangelogAuthorAvatarUrls {
    /// The URL of the item's 16x16 pixel avatar.
    #[serde(rename = "16x16", default, skip_serializing_if = "Option::is_none")]
    pub n16x16: Option<String>,
    /// The URL of the item's 24x24 pixel avatar.
    #[serde(rename = "24x24", default, skip_serializing_if = "Option::is_none")]
    pub n24x24: Option<String>,
    /// The URL of the item's 32x32 pixel avatar.
    #[serde(rename = "32x32", default, skip_serializing_if = "Option::is_none")]
    pub n32x32: Option<String>,
    /// The URL of the item's 48x48 pixel avatar.
    #[serde(rename = "48x48", default, skip_serializing_if = "Option::is_none")]
    pub n48x48: Option<String>,
}

/// User details permitted by the user's Atlassian Account privacy settings. However, be aware of these exceptions:
///
///  *  User record deleted from Atlassian: This occurs as the result of a right to be forgotten request. In this case, `displayName` provides an indication and other parameters have default values or are blank (for example, email is blank).
///  *  User record corrupted: This occurs as a results of events such as a server import and can only happen to deleted users. In this case, `accountId` returns *unknown* and all other parameters have fallback values.
///  *  User record unavailable: This usually occurs due to an internal service outage. In this case, all parameters have fallback values.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangelogAuthor {
    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*.
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The type of account represented by this user. This will be one of 'atlassian' (normal users), 'app' (application user) or 'customer' (Jira Service Desk customer user)
    #[serde(rename = "accountType", default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Whether the user is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "avatarUrls", default, skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<ChangelogAuthorAvatarUrls>,
    /// The display name of the user. Depending on the user’s privacy settings, this may return an alternative value.
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The email address of the user. Depending on the user’s privacy settings, this may be returned as null.
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The URL of the user.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The time zone specified in the user's profile. Depending on the user’s privacy settings, this may be returned as null.
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// Details of user or system associated with a issue history metadata item.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangelogHistoryMetadataActor {
    /// The URL to an avatar for the user or system associated with a history record.
    #[serde(rename = "avatarUrl", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// The display name of the user or system associated with a history record.
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The key of the display name of the user or system associated with a history record.
    #[serde(rename = "displayNameKey", default, skip_serializing_if = "Option::is_none")]
    pub display_name_key: Option<String>,
    /// The ID of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The URL of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Details of user or system associated with a issue history metadata item.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangelogHistoryMetadataCause {
    /// The URL to an avatar for the user or system associated with a history record.
    #[serde(rename = "avatarUrl", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// The display name of the user or system associated with a history record.
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The key of the display name of the user or system associated with a history record.
    #[serde(rename = "displayNameKey", default, skip_serializing_if = "Option::is_none")]
    pub display_name_key: Option<String>,
    /// The ID of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The URL of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Details of user or system associated with a issue history metadata item.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangelogHistoryMetadataGenerator {
    /// The URL to an avatar for the user or system associated with a history record.
    #[serde(rename = "avatarUrl", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// The display name of the user or system associated with a history record.
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The key of the display name of the user or system associated with a history record.
    #[serde(rename = "displayNameKey", default, skip_serializing_if = "Option::is_none")]
    pub display_name_key: Option<String>,
    /// The ID of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The URL of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Details of issue history metadata.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangelogHistoryMetadata {
    /// The activity described in the history record.
    #[serde(rename = "activityDescription", default, skip_serializing_if = "Option::is_none")]
    pub activity_description: Option<String>,
    /// The key of the activity described in the history record.
    #[serde(rename = "activityDescriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub activity_description_key: Option<String>,
    /// Details of user or system associated with a issue history metadata item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<ChangelogHistoryMetadataActor>,
    /// Details of user or system associated with a issue history metadata item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<ChangelogHistoryMetadataCause>,
    /// The description of the history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The description key of the history record.
    #[serde(rename = "descriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub description_key: Option<String>,
    /// The description of the email address associated the history record.
    #[serde(rename = "emailDescription", default, skip_serializing_if = "Option::is_none")]
    pub email_description: Option<String>,
    /// The description key of the email address associated the history record.
    #[serde(rename = "emailDescriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub email_description_key: Option<String>,
    /// Additional arbitrary information about the history record.
    #[serde(rename = "extraData", default, skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Details of user or system associated with a issue history metadata item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<ChangelogHistoryMetadataGenerator>,
    /// The type of the history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// A log of changes made to issue fields. Changelogs related to workflow associations are currently being deprecated.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Changelog {
    /// User details permitted by the user's Atlassian Account privacy settings. However, be aware of these exceptions:
    ///
    ///  *  User record deleted from Atlassian: This occurs as the result of a right to be forgotten request. In this case, `displayName` provides an indication and other parameters have default values or are blank (for example, email is blank).
    ///  *  User record corrupted: This occurs as a results of events such as a server import and can only happen to deleted users. In this case, `accountId` returns *unknown* and all other parameters have fallback values.
    ///  *  User record unavailable: This usually occurs due to an internal service outage. In this case, all parameters have fallback values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<ChangelogAuthor>,
    /// The date on which the change took place.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// The date on which the change took place.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// Details of issue history metadata.
    #[serde(rename = "historyMetadata", default, skip_serializing_if = "Option::is_none")]
    pub history_metadata: Option<ChangelogHistoryMetadata>,
    /// The ID of the changelog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The list of items changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ChangeDetails>>,
}
