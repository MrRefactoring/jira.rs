// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The gating status
    pub enum GetDeploymentGatingStatusByKeyGatingStatus {
        Allowed => "allowed",
        Prevented => "prevented",
        Awaiting => "awaiting",
        Invalid => "invalid",
    }
}

crate::open_enum! {
    /// The type of the gating status details.
    pub enum GetDeploymentGatingStatusByKeyDetailsType {
        Issue => "issue",
    }
}

/// Details related to the gating status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDeploymentGatingStatusByKeyDetails {
    /// The type of the gating status details.
    pub r#type: GetDeploymentGatingStatusByKeyDetailsType,
    /// An issue key that references an issue in Jira.
    #[serde(rename = "issueKey")]
    pub issue_key: String,
    /// A full HTTPS link to the Jira issue for the change request gating this Deployment. This field is provided if the details type is issue.
    #[serde(rename = "issueLink")]
    pub issue_link: String,
}

/// The current gating status for the given Deployment.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDeploymentGatingStatusByKey {
    /// This is the identifier for the Deployment.
    #[serde(rename = "deploymentSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub deployment_sequence_number: Option<i64>,
    /// The ID of the Deployment's pipeline.
    #[serde(rename = "pipelineId", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    /// The ID of the Deployment's environment.
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// Time the deployment gating status was updated.
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "updatedTimestamp",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /// Time the deployment gating status was updated.
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "updatedTimestamp",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub updated_timestamp: Option<String>,
    /// The gating status
    #[serde(rename = "gatingStatus", default, skip_serializing_if = "Option::is_none")]
    pub gating_status: Option<GetDeploymentGatingStatusByKeyGatingStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<GetDeploymentGatingStatusByKeyDetails>>,
}
