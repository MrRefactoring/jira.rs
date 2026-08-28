// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Fields that uniquely reference a deployment.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitDeploymentsAcceptedDeployments {
    /// The identifier of a pipeline, must be unique for the provider.
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// The identifier of an environment, must be unique for the provider so that it can be shared across pipelines.
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    /// This is the identifier for the deployment. It must be unique for the specified pipeline and environment. It must be a monotonically increasing number, as this is used to sequence the deployments.
    #[serde(rename = "deploymentSequenceNumber")]
    pub deployment_sequence_number: i64,
}

/// Fields that uniquely reference a deployment.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitDeploymentsRejectedDeploymentsKey {
    /// The identifier of a pipeline, must be unique for the provider.
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// The identifier of an environment, must be unique for the provider so that it can be shared across pipelines.
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    /// This is the identifier for the deployment. It must be unique for the specified pipeline and environment. It must be a monotonically increasing number, as this is used to sequence the deployments.
    #[serde(rename = "deploymentSequenceNumber")]
    pub deployment_sequence_number: i64,
}

/// A message supplied in the case of an error.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitDeploymentsRejectedDeploymentsErrors {
    /// A human-readable message describing the error.
    pub message: String,
    /// An optional trace ID that can be used by Jira developers to locate the source of the error.
    #[serde(rename = "errorTraceId", default, skip_serializing_if = "Option::is_none")]
    pub error_trace_id: Option<String>,
}

/// A deployment that has not been accepted for submission, usually due to a problem with the request data.
///
/// The object is comprised of the key of the rejected deployment and the corresponding error messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitDeploymentsRejectedDeployments {
    /// Fields that uniquely reference a deployment.
    pub key: SubmitDeploymentsRejectedDeploymentsKey,
    /// The error messages for the rejected deployment
    pub errors: Vec<SubmitDeploymentsRejectedDeploymentsErrors>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SubmitDeploymentsUnknownAssociations {
    IssueIdOrKeysAssociation(IssueIdOrKeysAssociation),
    ServiceIdOrKeysAssociation(ServiceIdOrKeysAssociation),
    EntityAssociation(EntityAssociation),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The result of a successful submitDeployments request.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitDeployments {
    /// The keys of deployments that have been accepted for submission. A deployment key is a composite key that consists of `pipelineId`, `environmentId` and `deploymentSequenceNumber`.
    ///
    /// A deployment may be rejected if it was only associated with unknown issue keys.
    ///
    /// Note that a deployment that isn't updated due to it's updateSequenceNumber being out of order is not considered a failed submission.
    #[serde(rename = "acceptedDeployments", default, skip_serializing_if = "Option::is_none")]
    pub accepted_deployments: Option<Vec<SubmitDeploymentsAcceptedDeployments>>,
    /// Details of deployments that have not been accepted for submission, usually due to a problem with the request data.
    ///
    /// The object will contain the deployment key and any errors associated with that deployment that have prevented it being submitted.
    #[serde(rename = "rejectedDeployments", default, skip_serializing_if = "Option::is_none")]
    pub rejected_deployments: Option<Vec<SubmitDeploymentsRejectedDeployments>>,
    /// Issue keys that are not known on this Jira instance (if any).
    ///
    /// These may be invalid keys (e.g. `UTF-8` is sometimes incorrectly identified as a Jira issue key), or they may be for projects that no longer exist.
    ///
    /// If a deployment has been associated with issue keys other than those in this array it will still be stored against those valid keys.
    /// If a deployment was only associated with issue keys deemed to be invalid it won't be persisted.
    #[serde(rename = "unknownIssueKeys", default, skip_serializing_if = "Option::is_none")]
    pub unknown_issue_keys: Option<Vec<String>>,
    /// Associations (e.g. Issue Keys or Service IDs) that are not known on this Jira instance (if any).
    ///
    /// These may be invalid keys (e.g. `UTF-8` is sometimes incorrectly identified as a Jira issue key), or they may be for projects that no longer exist.
    ///
    /// If a deployment has been associated with any other association other than those in this array it will still be stored against those valid associations.
    /// If a deployment was only associated with the associations in this array, it is deemed to be invalid and it won't be persisted.
    #[serde(rename = "unknownAssociations", default, skip_serializing_if = "Option::is_none")]
    pub unknown_associations: Option<Vec<SubmitDeploymentsUnknownAssociations>>,
}
