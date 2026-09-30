// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// IDs of entities grouped by entity type
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoreDevelopmentInformationAcceptedDevinfoEntities {
    /// Commits IDs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<String>>,
    /// Branch IDs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<String>>,
    /// Pull request IDs
    #[serde(rename = "pullRequests", default, skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<Vec<String>>,
}

/// A message supplied in the case of an error.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoreDevelopmentInformationFailedDevinfoEntitiesErrorMessages {
    /// A human-readable message describing the error.
    pub message: String,
    /// An optional trace ID that can be used by Jira developers to locate the source of the error.
    #[serde(rename = "errorTraceId", default, skip_serializing_if = "Option::is_none")]
    pub error_trace_id: Option<String>,
}

/// A message supplied in the case of an error.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoreDevelopmentInformationFailedDevinfoEntitiesCommitsErrorMessages {
    /// A human-readable message describing the error.
    pub message: String,
    /// An optional trace ID that can be used by Jira developers to locate the source of the error.
    #[serde(rename = "errorTraceId", default, skip_serializing_if = "Option::is_none")]
    pub error_trace_id: Option<String>,
}

/// Represents an error that happened with particular entity.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoreDevelopmentInformationFailedDevinfoEntitiesCommits {
    /// Entity id
    pub id: String,
    /// Error message
    #[serde(rename = "errorMessages", default, skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<StoreDevelopmentInformationFailedDevinfoEntitiesCommitsErrorMessages>>,
}

/// A message supplied in the case of an error.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoreDevelopmentInformationFailedDevinfoEntitiesBranchesErrorMessages {
    /// A human-readable message describing the error.
    pub message: String,
    /// An optional trace ID that can be used by Jira developers to locate the source of the error.
    #[serde(rename = "errorTraceId", default, skip_serializing_if = "Option::is_none")]
    pub error_trace_id: Option<String>,
}

/// Represents an error that happened with particular entity.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoreDevelopmentInformationFailedDevinfoEntitiesBranches {
    /// Entity id
    pub id: String,
    /// Error message
    #[serde(rename = "errorMessages", default, skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<StoreDevelopmentInformationFailedDevinfoEntitiesBranchesErrorMessages>>,
}

/// A message supplied in the case of an error.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoreDevelopmentInformationFailedDevinfoEntitiesPullRequestsErrorMessages {
    /// A human-readable message describing the error.
    pub message: String,
    /// An optional trace ID that can be used by Jira developers to locate the source of the error.
    #[serde(rename = "errorTraceId", default, skip_serializing_if = "Option::is_none")]
    pub error_trace_id: Option<String>,
}

/// Represents an error that happened with particular entity.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoreDevelopmentInformationFailedDevinfoEntitiesPullRequests {
    /// Entity id
    pub id: String,
    /// Error message
    #[serde(rename = "errorMessages", default, skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<StoreDevelopmentInformationFailedDevinfoEntitiesPullRequestsErrorMessages>>,
}

/// Represents errors related to a particular repository and its entities
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoreDevelopmentInformationFailedDevinfoEntities {
    /// Repository errors
    #[serde(rename = "errorMessages", default, skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<StoreDevelopmentInformationFailedDevinfoEntitiesErrorMessages>>,
    /// Commits errors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<StoreDevelopmentInformationFailedDevinfoEntitiesCommits>>,
    /// Branches errors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<StoreDevelopmentInformationFailedDevinfoEntitiesBranches>>,
    /// Pull requests errors
    #[serde(rename = "pullRequests", default, skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<Vec<StoreDevelopmentInformationFailedDevinfoEntitiesPullRequests>>,
}

/// The result of a successful store development information request
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoreDevelopmentInformation {
    /// The IDs of devinfo entities that have been accepted for submission grouped by their repository IDs. Note that a devinfo entity that isn't updated due to it's updateSequenceId being out of order is not considered a failed submission.
    #[serde(rename = "acceptedDevinfoEntities", default, skip_serializing_if = "Option::is_none")]
    pub accepted_devinfo_entities:
        Option<std::collections::HashMap<String, StoreDevelopmentInformationAcceptedDevinfoEntities>>,
    /// IDs of devinfo entities that have not been accepted for submission and caused error descriptions, usually due to a problem with the request data. The entities (if present) will be grouped by their repository id and type. Entity IDs are listed with errors associated with that devinfo entity that have prevented it being submitted.
    #[serde(rename = "failedDevinfoEntities", default, skip_serializing_if = "Option::is_none")]
    pub failed_devinfo_entities:
        Option<std::collections::HashMap<String, StoreDevelopmentInformationFailedDevinfoEntities>>,
    /// Issue keys that are not known on this Jira instance (if any). These may be invalid keys (e.g. `UTF-8` is sometimes incorrectly identified as a Jira issue key), or they may be for projects that no longer exist. If a devinfo entity has been associated with issue keys other than those in this array it will still be stored against those valid keys.
    #[serde(rename = "unknownIssueKeys", default, skip_serializing_if = "Option::is_none")]
    pub unknown_issue_keys: Option<Vec<String>>,
    /// Associations that are not known on this Jira instance (if any).
    ///
    /// These may be invalid keys (e.g. `UTF-8` is sometimes incorrectly identified as a Jira issue key), or they may be for projects that no longer exist.
    ///
    /// If a development information entity has been associated with any other association other than those in this array it will still be stored against those valid associations.
    /// If a development information entity was only associated with the associations in this array, it is deemed to be invalid and it won't be persisted.
    #[serde(rename = "unknownAssociations", default, skip_serializing_if = "Option::is_none")]
    pub unknown_associations: Option<Vec<IssueIdOrKeysAssociation>>,
}
