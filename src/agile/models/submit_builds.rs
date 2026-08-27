// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Fields that uniquely reference a build.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubmitBuildsAcceptedBuilds {
    /// An ID that relates a sequence of builds. Depending on your system this might be a project ID, pipeline ID,
    /// plan key etc. - whatever logical unit you use to group a sequence of builds.
    ///
    /// The combination of `pipelineId` and `buildNumber` must uniquely identify the build.
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// Identifies a build within the sequence of builds identified by the build `pipelineId`.
    ///
    /// Used to identify the 'most recent' build in that sequence of builds.
    ///
    /// The combination of `pipelineId` and `buildNumber` must uniquely identify the build.
    #[serde(rename = "buildNumber")]
    pub build_number: i64,
}

/// Fields that uniquely reference a build.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubmitBuildsRejectedBuildsKey {
    /// An ID that relates a sequence of builds. Depending on your system this might be a project ID, pipeline ID,
    /// plan key etc. - whatever logical unit you use to group a sequence of builds.
    ///
    /// The combination of `pipelineId` and `buildNumber` must uniquely identify the build.
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// Identifies a build within the sequence of builds identified by the build `pipelineId`.
    ///
    /// Used to identify the 'most recent' build in that sequence of builds.
    ///
    /// The combination of `pipelineId` and `buildNumber` must uniquely identify the build.
    #[serde(rename = "buildNumber")]
    pub build_number: i64,
}

/// A message supplied in the case of an error.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubmitBuildsRejectedBuildsErrors {
    /// A human-readable message describing the error.
    pub message: String,
    /// An optional trace ID that can be used by Jira developers to locate the source of the error.
    #[serde(rename = "errorTraceId", default, skip_serializing_if = "Option::is_none")]
    pub error_trace_id: Option<String>,
}

/// A build that has not been accepted for submission, usually due to a problem with the request data.
///
/// The object is comprised of the key of the rejected build and the corresponding error messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitBuildsRejectedBuilds {
    /// Fields that uniquely reference a build.
    pub key: SubmitBuildsRejectedBuildsKey,
    /// The error messages for the rejected build
    pub errors: Vec<SubmitBuildsRejectedBuildsErrors>,
}

/// The result of a successful `submitBuilds` request.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubmitBuilds {
    /// The keys of builds that have been accepted for submission. A build key is a composite key that consists of
    /// `pipelineId` and `buildNumber`.
    ///
    /// A build may be rejected if it was only associated with unknown issue keys, or if the submitted data for that
    /// build does not match the required schema.
    ///
    /// Note that a build that isn't updated due to it's `updateSequenceNumber` being out of order is not
    /// considered a failed submission.
    #[serde(rename = "acceptedBuilds", default, skip_serializing_if = "Option::is_none")]
    pub accepted_builds: Option<Vec<SubmitBuildsAcceptedBuilds>>,
    /// Details of builds that have not been accepted for submission.
    ///
    /// A build may be rejected if it was only associated with unknown issue keys, or if the submitted data for the
    /// build does not match the required schema.
    #[serde(rename = "rejectedBuilds", default, skip_serializing_if = "Option::is_none")]
    pub rejected_builds: Option<Vec<SubmitBuildsRejectedBuilds>>,
    /// Issue keys that are not known on this Jira instance (if any).
    ///
    /// These may be invalid keys (e.g. `UTF-8` is sometimes incorrectly identified as a Jira issue key), or they
    /// may be for projects that no longer exist.
    ///
    /// If a build has been associated with issue keys other than those in this array it will still be stored against
    /// those valid keys. If a build was only associated with issue keys deemed to be invalid it won't be persisted.
    #[serde(rename = "unknownIssueKeys", default, skip_serializing_if = "Option::is_none")]
    pub unknown_issue_keys: Option<Vec<String>>,
    /// Associations that are not known on this Jira instance (if any).
    ///
    /// These may be invalid keys (e.g. `UTF-8` is sometimes incorrectly identified as a Jira issue key), or they may be for projects that no longer exist.
    ///
    /// If a build has been associated with any other association other than those in this array it will still be stored against those valid associations.
    /// If a build was only associated with the associations in this array, it is deemed to be invalid and it won't be persisted.
    #[serde(rename = "unknownAssociations", default, skip_serializing_if = "Option::is_none")]
    pub unknown_associations: Option<Vec<IssueIdOrKeysAssociation>>,
}
