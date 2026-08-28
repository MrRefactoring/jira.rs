// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SubmitDeploymentsRequestDeploymentsAssociations {
    IssueIdOrKeysAssociation(IssueIdOrKeysAssociation),
    ServiceIdOrKeysAssociation(ServiceIdOrKeysAssociation),
    EntityAssociation(EntityAssociation),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    /// The state of the deployment
    pub enum SubmitDeploymentsRequestDeploymentsState {
        Unknown => "unknown",
        Pending => "pending",
        InProgress => "in_progress",
        Cancelled => "cancelled",
        Failed => "failed",
        RolledBack => "rolled_back",
        Successful => "successful",
    }
}

/// This object models the Continuous Delivery (CD) Pipeline concept, an automated process (usually comprised of multiple stages)
///
/// for getting software from version control right through to the production environment.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitDeploymentsRequestDeploymentsPipeline {
    /// The identifier of this pipeline, must be unique for the provider.
    pub id: String,
    /// The name of the pipeline to present to the user.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// A URL users can use to link to this deployment pipeline.
    pub url: String,
}

crate::open_enum! {
    /// The type of the environment.
    pub enum SubmitDeploymentsRequestDeploymentsEnvironmentType {
        Unmapped => "unmapped",
        Development => "development",
        Testing => "testing",
        Staging => "staging",
        Production => "production",
    }
}

/// The environment that the deployment is present in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitDeploymentsRequestDeploymentsEnvironment {
    /// The identifier of this environment, must be unique for the provider so that it can be shared across pipelines.
    pub id: String,
    /// The name of the environment to present to the user.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// The type of the environment.
    pub r#type: SubmitDeploymentsRequestDeploymentsEnvironmentType,
}

/// A command to be actioned for this Deployment
/// - command
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitDeploymentsRequestDeploymentsCommands {
    /// The command name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}

crate::open_enum! {
    /// The DeploymentData schema version used for this deployment data.
    ///
    /// Placeholder to support potential schema changes in the future.
    pub enum SubmitDeploymentsRequestDeploymentsSchemaVersion {
        N10 => "1.0",
    }
}

/// Data related to a specific deployment in a specific environment that the deployment is present in.
/// Must specify one of `issueKeys` or `associations`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitDeploymentsRequestDeployments {
    /// This is the identifier for the deployment. It must be unique for the specified pipeline and environment. It must be a monotonically increasing number, as this is used to sequence the deployments.
    #[serde(rename = "deploymentSequenceNumber")]
    pub deployment_sequence_number: i64,
    /// A number used to apply an order to the updates to the deployment, as identified by the deploymentSequenceNumber, in the case of out-of-order receipt of update requests. It must be a monotonically increasing number. For example, epoch time could be one way to generate the updateSequenceNumber.
    #[serde(rename = "updateSequenceNumber")]
    pub update_sequence_number: i64,
    /// The entities to associate the Deployment information with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<SubmitDeploymentsRequestDeploymentsAssociations>>,
    /// The human-readable name for the deployment. Will be shown in the UI.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// A URL users can use to link to this deployment, in this environment.
    pub url: String,
    /// A short description of the deployment
    pub description: String,
    /// The last-updated timestamp to present to the user as a summary of the state of the deployment.
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    /// The last-updated timestamp to present to the user as a summary of the state of the deployment.
    #[cfg(not(feature = "chrono"))]
    #[serde(rename = "lastUpdated", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub last_updated: String,
    /// An (optional) additional label that may be displayed with deployment information. Can be used to display version information etc. for the deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The duration of the deployment (in seconds).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// The state of the deployment
    pub state: SubmitDeploymentsRequestDeploymentsState,
    /// This object models the Continuous Delivery (CD) Pipeline concept, an automated process (usually comprised of multiple stages)
    ///
    /// for getting software from version control right through to the production environment.
    pub pipeline: SubmitDeploymentsRequestDeploymentsPipeline,
    /// The environment that the deployment is present in.
    pub environment: SubmitDeploymentsRequestDeploymentsEnvironment,
    /// A list of commands to be actioned for this Deployment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<SubmitDeploymentsRequestDeploymentsCommands>>,
    /// The DeploymentData schema version used for this deployment data.
    ///
    /// Placeholder to support potential schema changes in the future.
    #[serde(rename = "schemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<SubmitDeploymentsRequestDeploymentsSchemaVersion>,
}

/// Information about the provider. This is useful for auditing, logging, debugging,
/// and other internal uses. It is not considered private information. Hence, it may not contain personally
/// identifiable information.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitDeploymentsRequestProviderMetadata {
    /// An optional name of the source of the deployments data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}

/// The Deployments operations.
pub struct DeploymentsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> DeploymentsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Update / insert deployment data.
    ///
    /// Deployments are identified by the combination of `pipelineId`, `environmentId` and `deploymentSequenceNumber`, and existing deployment data for the same deployment will be replaced if it exists and the `updateSequenceNumber` of existing data is less than the incoming data.
    ///
    /// Submissions are processed asynchronously. Submitted data will eventually be available in Jira. Most updates are available within a short period of time, but may take some time during peak load and/or maintenance times. The `getDeploymentByKey` operation can be used to confirm that data has been stored successfully (if needed).
    ///
    /// In the case of multiple deployments being submitted in one request, each is validated individually prior to submission. Details of which deployments failed submission (if any) are available in the response object.
    pub fn submit_deployments(
        &self,
        deployments: impl IntoIterator<Item = SubmitDeploymentsRequestDeployments>,
    ) -> SubmitDeploymentsRequest<'a> {
        SubmitDeploymentsRequest::new(self.client, deployments)
    }

    /// Bulk delete all deployments that match the given request.
    ///
    /// One or more query params must be supplied to specify the Properties to delete by. Optional param `_updateSequenceNumber` is no longer supported.
    /// If more than one Property is provided, data will be deleted that matches ALL of the Properties (i.e. treated as AND).
    /// See the documentation for the `submitDeployments` operation for more details.
    ///
    /// Example operation: DELETE /bulkByProperties?accountId=account-123&createdBy=user-456
    ///
    /// Deletion is performed asynchronously. The `getDeploymentByKey` operation can be used to confirm that data has been deleted successfully (if needed).
    pub fn delete_deployments_by_property(
        &self,
        account_id: impl Into<String>,
    ) -> DeleteDeploymentsByPropertyRequest<'a> {
        DeleteDeploymentsByPropertyRequest::new(self.client, account_id)
    }

    /// Retrieve the currently stored deployment data for the given `pipelineId`, `environmentId` and `deploymentSequenceNumber` combination.
    ///
    /// The result will be what is currently stored, ignoring any pending updates or deletes.
    pub fn get_deployment_by_key(
        &self,
        pipeline_id: impl Into<String>,
        environment_id: impl Into<String>,
        deployment_sequence_number: i64,
    ) -> GetDeploymentByKeyRequest<'a> {
        GetDeploymentByKeyRequest::new(self.client, pipeline_id, environment_id, deployment_sequence_number)
    }

    /// Delete the currently stored deployment data for the given `pipelineId`, `environmentId` and `deploymentSequenceNumber` combination.
    ///
    /// Deletion is performed asynchronously. The `getDeploymentByKey` operation can be used to confirm that data has been deleted successfully (if needed).
    pub fn delete_deployment_by_key(
        &self,
        pipeline_id: impl Into<String>,
        environment_id: impl Into<String>,
        deployment_sequence_number: i64,
    ) -> DeleteDeploymentByKeyRequest<'a> {
        DeleteDeploymentByKeyRequest::new(self.client, pipeline_id, environment_id, deployment_sequence_number)
    }

    /// Retrieve the  Deployment gating status for the given `pipelineId + environmentId + deploymentSequenceNumber` combination.
    /// Only apps that define the `jiraDeploymentInfoProvider` module can access this resource. This resource requires the 'READ' scope.
    pub fn get_deployment_gating_status_by_key(
        &self,
        pipeline_id: impl Into<String>,
        environment_id: impl Into<String>,
        deployment_sequence_number: i64,
    ) -> GetDeploymentGatingStatusByKeyRequest<'a> {
        GetDeploymentGatingStatusByKeyRequest::new(self.client, pipeline_id, environment_id, deployment_sequence_number)
    }
}

/// Update / insert deployment data.
///
/// Deployments are identified by the combination of `pipelineId`, `environmentId` and `deploymentSequenceNumber`, and existing deployment data for the same deployment will be replaced if it exists and the `updateSequenceNumber` of existing data is less than the incoming data.
///
/// Submissions are processed asynchronously. Submitted data will eventually be available in Jira. Most updates are available within a short period of time, but may take some time during peak load and/or maintenance times. The `getDeploymentByKey` operation can be used to confirm that data has been stored successfully (if needed).
///
/// In the case of multiple deployments being submitted in one request, each is validated individually prior to submission. Details of which deployments failed submission (if any) are available in the response object.
#[derive(Clone)]
pub struct SubmitDeploymentsRequest<'a> {
    client: &'a crate::core::Client,
    properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    deployments: Vec<SubmitDeploymentsRequestDeployments>,
    provider_metadata: Option<SubmitDeploymentsRequestProviderMetadata>,
}

impl<'a> SubmitDeploymentsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        deployments: impl IntoIterator<Item = SubmitDeploymentsRequestDeployments>,
    ) -> Self {
        Self { client, deployments: deployments.into_iter().collect(), properties: None, provider_metadata: None }
    }

    /// Properties assigned to deployment data that can then be used for delete / query operations.
    ///
    /// Examples might be an account or user ID that can then be used to clean up data if an account is removed from the Provider system.
    ///
    /// Properties are supplied as key/value pairs, and a maximum of 5 properties can be supplied, keys cannot contain ':' or start with '_'.
    #[must_use]
    pub fn properties(mut self, value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);

        self
    }

    /// Information about the provider. This is useful for auditing, logging, debugging,
    /// and other internal uses. It is not considered private information. Hence, it may not contain personally
    /// identifiable information.
    #[must_use]
    pub fn provider_metadata(mut self, value: SubmitDeploymentsRequestProviderMetadata) -> Self {
        self.provider_metadata = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/deployments/0.1/bulk".to_owned());

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.properties {
            body.insert("properties".to_owned(), serde_json::to_value(value)?);
        }

        body.insert("deployments".to_owned(), serde_json::to_value(&self.deployments)?);

        if let Some(value) = &self.provider_metadata {
            body.insert("providerMetadata".to_owned(), serde_json::to_value(value)?);
        }

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SubmitDeployments> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Bulk delete all deployments that match the given request.
///
/// One or more query params must be supplied to specify the Properties to delete by. Optional param `_updateSequenceNumber` is no longer supported.
/// If more than one Property is provided, data will be deleted that matches ALL of the Properties (i.e. treated as AND).
/// See the documentation for the `submitDeployments` operation for more details.
///
/// Example operation: DELETE /bulkByProperties?accountId=account-123&createdBy=user-456
///
/// Deletion is performed asynchronously. The `getDeploymentByKey` operation can be used to confirm that data has been deleted successfully (if needed).
#[derive(Clone)]
pub struct DeleteDeploymentsByPropertyRequest<'a> {
    client: &'a crate::core::Client,
    account_id: String,
    created_by: Option<String>,
}

impl<'a> DeleteDeploymentsByPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, account_id: impl Into<String>) -> Self {
        Self { client, account_id: account_id.into(), created_by: None }
    }

    /// Optional additional property filter combined with accountId (AND). Must match a key previously supplied in submitDeployments `properties`. Example: createdBy=user-456.
    #[must_use]
    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            "/rest/deployments/0.1/bulkByProperties".to_owned(),
        );

        config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(self.account_id.clone())));

        if let Some(value) = &self.created_by {
            config.query.push(("createdBy".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Retrieve the currently stored deployment data for the given `pipelineId`, `environmentId` and `deploymentSequenceNumber` combination.
///
/// The result will be what is currently stored, ignoring any pending updates or deletes.
#[derive(Clone)]
pub struct GetDeploymentByKeyRequest<'a> {
    client: &'a crate::core::Client,
    pipeline_id: String,
    environment_id: String,
    deployment_sequence_number: i64,
}

impl<'a> GetDeploymentByKeyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        pipeline_id: impl Into<String>,
        environment_id: impl Into<String>,
        deployment_sequence_number: i64,
    ) -> Self {
        Self {
            client,
            pipeline_id: pipeline_id.into(),
            environment_id: environment_id.into(),
            deployment_sequence_number,
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/deployments/0.1/pipelines/{}/environments/{}/deployments/{}",
                crate::core::encode_path_segment(&self.pipeline_id),
                crate::core::encode_path_segment(&self.environment_id),
                self.deployment_sequence_number
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetDeploymentByKey> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete the currently stored deployment data for the given `pipelineId`, `environmentId` and `deploymentSequenceNumber` combination.
///
/// Deletion is performed asynchronously. The `getDeploymentByKey` operation can be used to confirm that data has been deleted successfully (if needed).
#[derive(Clone)]
pub struct DeleteDeploymentByKeyRequest<'a> {
    client: &'a crate::core::Client,
    pipeline_id: String,
    environment_id: String,
    deployment_sequence_number: i64,
}

impl<'a> DeleteDeploymentByKeyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        pipeline_id: impl Into<String>,
        environment_id: impl Into<String>,
        deployment_sequence_number: i64,
    ) -> Self {
        Self {
            client,
            pipeline_id: pipeline_id.into(),
            environment_id: environment_id.into(),
            deployment_sequence_number,
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/deployments/0.1/pipelines/{}/environments/{}/deployments/{}",
                crate::core::encode_path_segment(&self.pipeline_id),
                crate::core::encode_path_segment(&self.environment_id),
                self.deployment_sequence_number
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Retrieve the  Deployment gating status for the given `pipelineId + environmentId + deploymentSequenceNumber` combination.
/// Only apps that define the `jiraDeploymentInfoProvider` module can access this resource. This resource requires the 'READ' scope.
#[derive(Clone)]
pub struct GetDeploymentGatingStatusByKeyRequest<'a> {
    client: &'a crate::core::Client,
    pipeline_id: String,
    environment_id: String,
    deployment_sequence_number: i64,
}

impl<'a> GetDeploymentGatingStatusByKeyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        pipeline_id: impl Into<String>,
        environment_id: impl Into<String>,
        deployment_sequence_number: i64,
    ) -> Self {
        Self {
            client,
            pipeline_id: pipeline_id.into(),
            environment_id: environment_id.into(),
            deployment_sequence_number,
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/deployments/0.1/pipelines/{}/environments/{}/deployments/{}/gating-status",
                crate::core::encode_path_segment(&self.pipeline_id),
                crate::core::encode_path_segment(&self.environment_id),
                self.deployment_sequence_number
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetDeploymentGatingStatusByKey> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
