// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The schema version used for this data.
    ///
    /// Placeholder to support potential schema changes in the future.
    pub enum GetRemoteLinkByIdSchemaVersion {
        N10 => "1.0",
    }
}

crate::open_enum! {
    /// The type of the Remote Link. The current supported types are 'document', 'alert', 'test',
    /// 'security', 'logFile', 'prototype', 'coverage', 'bugReport' and 'other'
    pub enum GetRemoteLinkByIdType {
        Document => "document",
        Alert => "alert",
        Test => "test",
        Security => "security",
        LogFile => "logFile",
        Prototype => "prototype",
        Coverage => "coverage",
        BugReport => "bugReport",
        Other2 => "other",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetRemoteLinkByIdAssociations {
    IssueIdOrKeysAssociation(IssueIdOrKeysAssociation),
    ServiceIdOrKeysAssociation(ServiceIdOrKeysAssociation),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    /// Appearance is a fixed set of appearance types affecting the colour
    /// of the status lozenge in the UI. The colours they correspond to are
    /// equivalent to atlaskit's [Lozenge](https://atlaskit.atlassian.com/packages/core/lozenge) component.
    pub enum GetRemoteLinkByIdStatusAppearance {
        Default => "default",
        Inprogress => "inprogress",
        Moved => "moved",
        New => "new",
        Removed => "removed",
        Prototype => "prototype",
        Success => "success",
    }
}

/// The status of a Remote Link.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetRemoteLinkByIdStatus {
    /// Appearance is a fixed set of appearance types affecting the colour
    /// of the status lozenge in the UI. The colours they correspond to are
    /// equivalent to atlaskit's [Lozenge](https://atlaskit.atlassian.com/packages/core/lozenge) component.
    pub appearance: GetRemoteLinkByIdStatusAppearance,
    /// The human-readable description for the Remote Link status.
    ///
    /// Will be shown in the UI.
    pub label: String,
}

/// Data related to a single Remote Link.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetRemoteLinkById {
    /// The schema version used for this data.
    ///
    /// Placeholder to support potential schema changes in the future.
    #[serde(rename = "schemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<GetRemoteLinkByIdSchemaVersion>,
    /// The identifier for the Remote Link. Must be unique for a given Provider.
    pub id: String,
    /// An ID used to apply an ordering to updates for this Remote Link in the case of out-of-order receipt of
    /// update requests.
    ///
    /// It must be a monotonically increasing number. For example, epoch time could be one way to generate the
    /// `updateSequenceNumber`.
    ///
    /// Updates for a Remote Link that is received with an `updateSqeuenceNumber` less than or equal to what is currently
    /// stored will be ignored.
    #[serde(rename = "updateSequenceNumber")]
    pub update_sequence_number: i64,
    /// The human-readable name for the Remote Link.
    ///
    /// Will be shown in the UI.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// The URL to this Remote Link in your system.
    pub url: String,
    /// The type of the Remote Link. The current supported types are 'document', 'alert', 'test',
    /// 'security', 'logFile', 'prototype', 'coverage', 'bugReport' and 'other'
    pub r#type: GetRemoteLinkByIdType,
    /// An optional description to attach to this Remote Link.
    ///
    /// This may be anything that makes sense in your system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The last-updated timestamp to present to the user as a summary of when Remote Link was last updated.
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    /// The last-updated timestamp to present to the user as a summary of when Remote Link was last updated.
    #[cfg(not(feature = "chrono"))]
    #[serde(rename = "lastUpdated", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub last_updated: String,
    /// The entities to associate the Remote Link information with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<GetRemoteLinkByIdAssociations>>,
    /// The status of a Remote Link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GetRemoteLinkByIdStatus>,
    /// Optional list of actionIds. They are associated with the actions the provider is able to provide when they
    /// registered. Indicates which actions this Remote Link has.
    ///
    /// If any actions have a templateUrl that requires string substitution, then `attributeMap` must be passed in.
    #[serde(rename = "actionIds", default, skip_serializing_if = "Option::is_none")]
    pub action_ids: Option<Vec<String>>,
    /// Map of key/values (string to string mapping). This is used to build the urls for actions from the
    /// templateUrl the provider registered their available actions with.
    #[serde(rename = "attributeMap", default, skip_serializing_if = "Option::is_none")]
    pub attribute_map: Option<std::collections::HashMap<String, serde_json::Value>>,
}
