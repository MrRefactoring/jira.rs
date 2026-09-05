// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The result of a successful submitFeatureFlags request.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitFeatureFlags {
    /// The IDs of Feature Flags that have been accepted for submission.
    ///
    /// A Feature Flag may be rejected if it was only associated with unknown issue keys.
    ///
    /// Note that a Feature Flag that isn't updated due to it's updateSequenceId being out of order is not considered a failed submission.
    #[serde(rename = "acceptedFeatureFlags", default, skip_serializing_if = "Option::is_none")]
    pub accepted_feature_flags: Option<Vec<String>>,
    /// Details of Feature Flags that have not been accepted for submission, usually due to a problem with the request data.
    ///
    /// The object (if present) will be keyed by Feature Flag ID and include any errors associated with that Feature Flag that have prevented it being submitted.
    #[serde(rename = "failedFeatureFlags", default, skip_serializing_if = "Option::is_none")]
    pub failed_feature_flags: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Issue keys that are not known on this Jira instance (if any).
    ///
    /// These may be invalid keys (e.g. `UTF-8` is sometimes incorrectly identified as a Jira issue key), or they may be for projects that no longer exist.
    ///
    /// If a Feature Flag has been associated with issue keys other than those in this array it will still be stored against those valid keys.
    /// If a Feature Flag was only associated with issue keys deemed to be invalid it won't be persisted.
    #[serde(rename = "unknownIssueKeys", default, skip_serializing_if = "Option::is_none")]
    pub unknown_issue_keys: Option<Vec<String>>,
    /// Associations that are not known on this Jira instance (if any).
    ///
    /// These may be invalid keys (e.g. `UTF-8` is sometimes incorrectly identified as a Jira issue key), or they may be for projects that no longer exist.
    ///
    /// If a feature flag has been associated with any other association other than those in this array it will still be stored against those valid associations.
    /// If a feature flag was only associated with the associations in this array, it is deemed to be invalid and it won't be persisted.
    #[serde(rename = "unknownAssociations", default, skip_serializing_if = "Option::is_none")]
    pub unknown_associations: Option<Vec<IssueIdOrKeysAssociation>>,
}
