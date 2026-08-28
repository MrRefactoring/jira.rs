// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The DevOpsComponentData schema version used for this devops component data.
    ///
    /// Placeholder to support potential schema changes in the future.
    pub enum SubmitComponentsRequestDevopsComponentsSchemaVersion {
        N10 => "1.0",
    }
}

crate::open_enum! {
    /// The tier of the component. Will be shown in the UI.
    pub enum SubmitComponentsRequestDevopsComponentsTier {
        Tier1 => "Tier 1",
        Tier2 => "Tier 2",
        Tier3 => "Tier 3",
        Tier4 => "Tier 4",
    }
}

crate::open_enum! {
    /// The type of the component. Will be shown in the UI.
    pub enum SubmitComponentsRequestDevopsComponentsComponentType {
        Service => "Service",
        Application => "Application",
        Library => "Library",
        Capability => "Capability",
        CloudResource => "Cloud resource",
        DataPipeline => "Data pipeline",
        MachineLearningModel => "Machine learning model",
        UiElement => "UI element",
        Website => "Website",
        Other2 => "Other",
    }
}

/// Data related to a specific component in a specific workspace that is affected by incidents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitComponentsRequestDevopsComponents {
    /// The DevOpsComponentData schema version used for this devops component data.
    ///
    /// Placeholder to support potential schema changes in the future.
    #[serde(rename = "schemaVersion")]
    pub schema_version: SubmitComponentsRequestDevopsComponentsSchemaVersion,
    /// The identifier for the DevOps Component. Must be unique for a given Provider.
    pub id: String,
    /// An ID used to apply an ordering to updates for this DevOps Component in the case of out-of-order receipt of update requests.
    ///
    /// This can be any monotonically increasing number. A suggested implementation is to use epoch millis from the Provider system, but other alternatives are valid (e.g. a Provider could store a counter against each DevOps Component and increment that on each update to Jira).
    ///
    /// Updates for a DevOps Component that are received with an updateSqeuenceId lower than what is currently stored will be ignored.
    #[serde(rename = "updateSequenceNumber")]
    pub update_sequence_number: i64,
    /// The human-readable name for the DevOps Component. Will be shown in the UI.
    pub name: String,
    /// The human-readable name for the Provider that owns this DevOps Component. Will be shown in the UI.
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// A description of the DevOps Component in Markdown format. Will be shown in the UI.
    pub description: String,
    /// A URL users can use to link to a summary view of this devops component, if appropriate.
    ///
    /// This could be any location that makes sense in the Provider system (e.g. if the summary information comes from a specific project, it might make sense to link the user to the component in that project).
    pub url: String,
    /// A URL to display a logo representing this devops component, if available.
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
    /// The tier of the component. Will be shown in the UI.
    pub tier: SubmitComponentsRequestDevopsComponentsTier,
    /// The type of the component. Will be shown in the UI.
    #[serde(rename = "componentType")]
    pub component_type: SubmitComponentsRequestDevopsComponentsComponentType,
    /// The last-updated timestamp to present to the user the last time the DevOps Component was updated.
    ///
    /// Expected format is an RFC3339 formatted string.
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    /// The last-updated timestamp to present to the user the last time the DevOps Component was updated.
    ///
    /// Expected format is an RFC3339 formatted string.
    #[cfg(not(feature = "chrono"))]
    #[serde(rename = "lastUpdated", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub last_updated: String,
}

/// Information about the provider. This is useful for auditing, logging, debugging,
/// and other internal uses. It is not considered private information. Hence, it may not contain personally
/// identifiable information.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitComponentsRequestProviderMetadata {
    /// An optional name of the source of the incidents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}

/// The DevopsComponents operations.
pub struct DevopsComponentsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> DevopsComponentsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Update / insert DevOps Component data.
    ///
    /// Components are identified by their ID, and existing Component data for the same ID will be replaced if it exists and the updateSequenceNumber of existing data is less than the incoming data.
    ///
    /// Submissions are performed asynchronously. Submitted data will eventually be available in Jira; most updates are available within a short period of time, but may take some time during peak load and/or maintenance times. The getComponentById operation can be used to confirm that data has been stored successfully (if needed).
    ///
    /// In the case of multiple Components being submitted in one request, each is validated individually prior to submission. Details of which Components failed submission (if any) are available in the response object.
    ///
    /// A maximum of 1000 components can be submitted in one request.
    ///
    /// Only Connect apps that define the `jiraDevOpsComponentProvider` module can access this resource.
    /// This resource requires the 'WRITE' scope for Connect apps.
    pub fn submit_components(
        &self,
        devops_components: impl IntoIterator<Item = SubmitComponentsRequestDevopsComponents>,
    ) -> SubmitComponentsRequest<'a> {
        SubmitComponentsRequest::new(self.client, devops_components)
    }

    /// Bulk delete all Components that match the given request.
    ///
    /// One or more query params must be supplied to specify Properties to delete by.
    /// If more than one Property is provided, data will be deleted that matches ALL of the Properties (e.g. treated as an AND).
    /// See the documentation for the submitComponents operation for more details.
    ///
    /// e.g. DELETE /bulkByProperties?accountId=account-123&createdBy=user-456
    ///
    /// Deletion is performed asynchronously. The getComponentById operation can be used to confirm that data has been deleted successfully (if needed).
    ///
    /// Only Connect apps that define the `jiraDevOpsComponentProvider` module can access this resource.
    /// This resource requires the 'DELETE' scope for Connect apps.
    pub fn delete_components_by_property(
        &self,
        account_id: impl Into<String>,
    ) -> DeleteComponentsByPropertyRequest<'a> {
        DeleteComponentsByPropertyRequest::new(self.client, account_id)
    }

    /// Retrieve the currently stored Component data for the given ID.
    ///
    /// The result will be what is currently stored, ignoring any pending updates or deletes.
    ///
    /// Only Connect apps that define the `jiraDevOpsComponentProvider` module can access this resource.
    /// This resource requires the 'READ' scope for Connect apps.
    pub fn get_component_by_id(&self, component_id: impl Into<String>) -> GetComponentByIdRequest<'a> {
        GetComponentByIdRequest::new(self.client, component_id)
    }

    /// Delete the Component data currently stored for the given ID.
    ///
    /// Deletion is performed asynchronously. The getComponentById operation can be used to confirm that data has been deleted successfully (if needed).
    ///
    /// Only Connect apps that define the `jiraDevOpsComponentProvider` module can access this resource.
    /// This resource requires the 'DELETE' scope for Connect apps.
    pub fn delete_component_by_id(&self, component_id: impl Into<String>) -> DeleteComponentByIdRequest<'a> {
        DeleteComponentByIdRequest::new(self.client, component_id)
    }
}

/// Update / insert DevOps Component data.
///
/// Components are identified by their ID, and existing Component data for the same ID will be replaced if it exists and the updateSequenceNumber of existing data is less than the incoming data.
///
/// Submissions are performed asynchronously. Submitted data will eventually be available in Jira; most updates are available within a short period of time, but may take some time during peak load and/or maintenance times. The getComponentById operation can be used to confirm that data has been stored successfully (if needed).
///
/// In the case of multiple Components being submitted in one request, each is validated individually prior to submission. Details of which Components failed submission (if any) are available in the response object.
///
/// A maximum of 1000 components can be submitted in one request.
///
/// Only Connect apps that define the `jiraDevOpsComponentProvider` module can access this resource.
/// This resource requires the 'WRITE' scope for Connect apps.
pub struct SubmitComponentsRequest<'a> {
    client: &'a crate::core::Client,
    properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    devops_components: Vec<SubmitComponentsRequestDevopsComponents>,
    provider_metadata: Option<SubmitComponentsRequestProviderMetadata>,
}

impl<'a> SubmitComponentsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        devops_components: impl IntoIterator<Item = SubmitComponentsRequestDevopsComponents>,
    ) -> Self {
        Self {
            client,
            devops_components: devops_components.into_iter().collect(),
            properties: None,
            provider_metadata: None,
        }
    }

    /// Properties assigned to incidents/components/review data that can then be used for delete / query operations.
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
    pub fn provider_metadata(mut self, value: SubmitComponentsRequestProviderMetadata) -> Self {
        self.provider_metadata = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/devopscomponents/1.0/bulk".to_owned());

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.properties {
            body.insert("properties".to_owned(), serde_json::to_value(value)?);
        }

        body.insert("devopsComponents".to_owned(), serde_json::to_value(&self.devops_components)?);

        if let Some(value) = &self.provider_metadata {
            body.insert("providerMetadata".to_owned(), serde_json::to_value(value)?);
        }

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SubmitComponents> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Bulk delete all Components that match the given request.
///
/// One or more query params must be supplied to specify Properties to delete by.
/// If more than one Property is provided, data will be deleted that matches ALL of the Properties (e.g. treated as an AND).
/// See the documentation for the submitComponents operation for more details.
///
/// e.g. DELETE /bulkByProperties?accountId=account-123&createdBy=user-456
///
/// Deletion is performed asynchronously. The getComponentById operation can be used to confirm that data has been deleted successfully (if needed).
///
/// Only Connect apps that define the `jiraDevOpsComponentProvider` module can access this resource.
/// This resource requires the 'DELETE' scope for Connect apps.
pub struct DeleteComponentsByPropertyRequest<'a> {
    client: &'a crate::core::Client,
    account_id: String,
    created_by: Option<String>,
}

impl<'a> DeleteComponentsByPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, account_id: impl Into<String>) -> Self {
        Self { client, account_id: account_id.into(), created_by: None }
    }

    /// Optional additional property filter combined with accountId (AND). Must match a key previously supplied in submitComponents `properties`. Example: createdBy=user-456.
    #[must_use]
    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            "/rest/devopscomponents/1.0/bulkByProperties".to_owned(),
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

/// Retrieve the currently stored Component data for the given ID.
///
/// The result will be what is currently stored, ignoring any pending updates or deletes.
///
/// Only Connect apps that define the `jiraDevOpsComponentProvider` module can access this resource.
/// This resource requires the 'READ' scope for Connect apps.
pub struct GetComponentByIdRequest<'a> {
    client: &'a crate::core::Client,
    component_id: String,
}

impl<'a> GetComponentByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, component_id: impl Into<String>) -> Self {
        Self { client, component_id: component_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/devopscomponents/1.0/devopscomponents/{}",
                crate::core::encode_path_segment(&self.component_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetComponentById> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete the Component data currently stored for the given ID.
///
/// Deletion is performed asynchronously. The getComponentById operation can be used to confirm that data has been deleted successfully (if needed).
///
/// Only Connect apps that define the `jiraDevOpsComponentProvider` module can access this resource.
/// This resource requires the 'DELETE' scope for Connect apps.
pub struct DeleteComponentByIdRequest<'a> {
    client: &'a crate::core::Client,
    component_id: String,
}

impl<'a> DeleteComponentByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, component_id: impl Into<String>) -> Self {
        Self { client, component_id: component_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/devopscomponents/1.0/devopscomponents/{}",
                crate::core::encode_path_segment(&self.component_id)
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
