// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The FeatureFlagData schema version used for this flag data.
    ///
    /// Placeholder to support potential schema changes in the future.
    pub enum SubmitFeatureFlagsRequestFlagsSchemaVersion {
        N10 => "1.0",
    }
}

/// Information about the rollout of a Feature Flag in an environment (or in summary).
///
/// Only one of 'percentage', 'text', or 'rules' should be provided. They will be used in that order if multiple are present.
///
/// This information may be presented to the user in the UI.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitFeatureFlagsRequestFlagsSummaryStatusRollout {
    /// If the Feature Flag rollout is a simple percentage rollout
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    /// A text status to display that represents the rollout. This could be e.g. a named cohort.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// A count of the number of rules active for this Feature Flag in an environment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<i64>,
}

/// Status information about a single Feature Flag.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitFeatureFlagsRequestFlagsSummaryStatus {
    /// Whether the Feature Flag is enabled in the given environment (or in summary).
    ///
    /// Enabled may imply a partial rollout, which can be described using the 'rollout' field.
    pub enabled: bool,
    /// The value served by this Feature Flag when it is disabled. This could be the actual value or an alias, as appropriate.
    ///
    /// This value may be presented to the user in the UI.
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Information about the rollout of a Feature Flag in an environment (or in summary).
    ///
    /// Only one of 'percentage', 'text', or 'rules' should be provided. They will be used in that order if multiple are present.
    ///
    /// This information may be presented to the user in the UI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollout: Option<SubmitFeatureFlagsRequestFlagsSummaryStatusRollout>,
}

/// Summary information for a single Feature Flag.
///
/// Providers may elect to provide information from a specific environment, or they may choose to 'roll up' information from across multiple environments - whatever makes most sense in the Provider system.
///
/// This is the summary information that will be presented to the user on e.g. the Jira issue screen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitFeatureFlagsRequestFlagsSummary {
    /// A URL users can use to link to a summary view of this flag, if appropriate.
    ///
    /// This could be any location that makes sense in the Provider system (e.g. if the summary information comes from a specific environment, it might make sense to link the user to the flag in that environment).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Status information about a single Feature Flag.
    pub status: SubmitFeatureFlagsRequestFlagsSummaryStatus,
    /// The last-updated timestamp to present to the user as a summary of the state of the Feature Flag.
    ///
    /// Providers may choose to supply the last-updated timestamp from a specific environment, or the 'most recent' last-updated timestamp across all environments - whatever makes sense in the Provider system.
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
    /// The last-updated timestamp to present to the user as a summary of the state of the Feature Flag.
    ///
    /// Providers may choose to supply the last-updated timestamp from a specific environment, or the 'most recent' last-updated timestamp across all environments - whatever makes sense in the Provider system.
    ///
    /// Expected format is an RFC3339 formatted string.
    #[cfg(not(feature = "chrono"))]
    #[serde(rename = "lastUpdated", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub last_updated: String,
}

crate::open_enum! {
    /// The 'type' or 'category' of environment this environment belongs to.
    pub enum SubmitFeatureFlagsRequestFlagsDetailsEnvironmentType {
        Development => "development",
        Testing => "testing",
        Staging => "staging",
        Production => "production",
    }
}

/// Details of a single environment.
///
/// At the simplest this must be the name of the environment.
///
/// Ideally there is also type information which may be used to group data from multiple Feature Flags and other entities for visualisation in the UI.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitFeatureFlagsRequestFlagsDetailsEnvironment {
    /// The name of the environment.
    pub name: String,
    /// The 'type' or 'category' of environment this environment belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SubmitFeatureFlagsRequestFlagsDetailsEnvironmentType>,
}

/// Information about the rollout of a Feature Flag in an environment (or in summary).
///
/// Only one of 'percentage', 'text', or 'rules' should be provided. They will be used in that order if multiple are present.
///
/// This information may be presented to the user in the UI.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitFeatureFlagsRequestFlagsDetailsStatusRollout {
    /// If the Feature Flag rollout is a simple percentage rollout
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    /// A text status to display that represents the rollout. This could be e.g. a named cohort.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// A count of the number of rules active for this Feature Flag in an environment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<i64>,
}

/// Status information about a single Feature Flag.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitFeatureFlagsRequestFlagsDetailsStatus {
    /// Whether the Feature Flag is enabled in the given environment (or in summary).
    ///
    /// Enabled may imply a partial rollout, which can be described using the 'rollout' field.
    pub enabled: bool,
    /// The value served by this Feature Flag when it is disabled. This could be the actual value or an alias, as appropriate.
    ///
    /// This value may be presented to the user in the UI.
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Information about the rollout of a Feature Flag in an environment (or in summary).
    ///
    /// Only one of 'percentage', 'text', or 'rules' should be provided. They will be used in that order if multiple are present.
    ///
    /// This information may be presented to the user in the UI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollout: Option<SubmitFeatureFlagsRequestFlagsDetailsStatusRollout>,
}

/// Details of a Feature Flag for a single environment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitFeatureFlagsRequestFlagsDetails {
    /// A URL users can use to link to this Feature Flag, in this environment.
    pub url: String,
    /// The last-updated timestamp for this Feature Flag, in this environment.
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
    /// The last-updated timestamp for this Feature Flag, in this environment.
    ///
    /// Expected format is an RFC3339 formatted string.
    #[cfg(not(feature = "chrono"))]
    #[serde(rename = "lastUpdated", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub last_updated: String,
    /// Details of a single environment.
    ///
    /// At the simplest this must be the name of the environment.
    ///
    /// Ideally there is also type information which may be used to group data from multiple Feature Flags and other entities for visualisation in the UI.
    pub environment: SubmitFeatureFlagsRequestFlagsDetailsEnvironment,
    /// Status information about a single Feature Flag.
    pub status: SubmitFeatureFlagsRequestFlagsDetailsStatus,
}

/// Data related to a single Feature Flag, across any Environment that the flag is present in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitFeatureFlagsRequestFlags {
    /// The FeatureFlagData schema version used for this flag data.
    ///
    /// Placeholder to support potential schema changes in the future.
    #[serde(rename = "schemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<SubmitFeatureFlagsRequestFlagsSchemaVersion>,
    /// The identifier for the Feature Flag. Must be unique for a given Provider.
    pub id: String,
    /// The identifier that users would use to reference the Feature Flag in their source code etc.
    ///
    /// Will be made available via the UI for users to copy into their source code etc.
    pub key: String,
    /// An ID used to apply an ordering to updates for this Feature Flag in the case of out-of-order receipt of update requests.
    ///
    /// This can be any monotonically increasing number. A suggested implementation is to use epoch millis from the Provider system, but other alternatives are valid (e.g. a Provider could store a counter against each Feature Flag and increment that on each update to Jira).
    ///
    /// Updates for a Feature Flag that are received with an updateSqeuenceId lower than what is currently stored will be ignored.
    #[serde(rename = "updateSequenceId")]
    pub update_sequence_id: i64,
    /// The human-readable name for the Feature Flag. Will be shown in the UI.
    ///
    /// If not provided, will use the ID for display.
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The Jira issue keys or IDs to associate the feature flag with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<IssueIdOrKeysAssociation>>,
    /// Summary information for a single Feature Flag.
    ///
    /// Providers may elect to provide information from a specific environment, or they may choose to 'roll up' information from across multiple environments - whatever makes most sense in the Provider system.
    ///
    /// This is the summary information that will be presented to the user on e.g. the Jira issue screen.
    pub summary: SubmitFeatureFlagsRequestFlagsSummary,
    /// Detail information for this Feature Flag.
    ///
    /// This may be information for each environment the Feature Flag is defined in or a selection of environments made by the user, as appropriate.
    pub details: Vec<SubmitFeatureFlagsRequestFlagsDetails>,
}

/// Information about the provider. This is useful for auditing, logging, debugging,
/// and other internal uses. It is not considered private information. Hence, it may not contain personally
/// identifiable information.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitFeatureFlagsRequestProviderMetadata {
    /// An optional name of the source of the feature flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}

/// The FeatureFlags operations.
pub struct FeatureFlagsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> FeatureFlagsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Update / insert Feature Flag data.
    ///
    /// Feature Flags are identified by their ID, and existing Feature Flag data for the same ID will be replaced if it exists and the updateSequenceId of existing data is less than the incoming data.
    ///
    /// Submissions are performed asynchronously. Submitted data will eventually be available in Jira; most updates are available within a short period of time, but may take some time during peak load and/or maintenance times. The getFeatureFlagById operation can be used to confirm that data has been stored successfully (if needed).
    ///
    /// In the case of multiple Feature Flags being submitted in one request, each is validated individually prior to submission. Details of which Feature Flags failed submission (if any) are available in the response object.
    pub fn submit_feature_flags(
        &self,
        flags: impl IntoIterator<Item = SubmitFeatureFlagsRequestFlags>,
    ) -> SubmitFeatureFlagsRequest<'a> {
        SubmitFeatureFlagsRequest::new(self.client, flags)
    }

    /// Bulk delete all Feature Flags that match the given request.
    ///
    /// One or more query params must be supplied to specify Properties to delete by. Optional param `_updateSequenceId` is no longer supported.
    /// If more than one Property is provided, data will be deleted that matches ALL of the Properties (e.g. treated as an AND).
    /// See the documentation for the submitFeatureFlags operation for more details.
    ///
    /// e.g. DELETE /bulkByProperties?accountId=account-123&createdBy=user-456
    ///
    /// Deletion is performed asynchronously. The getFeatureFlagById operation can be used to confirm that data has been deleted successfully (if needed).
    pub fn delete_feature_flags_by_property(
        &self,
        account_id: impl Into<String>,
    ) -> DeleteFeatureFlagsByPropertyRequest<'a> {
        DeleteFeatureFlagsByPropertyRequest::new(self.client, account_id)
    }

    /// Retrieve the currently stored Feature Flag data for the given ID.
    ///
    /// The result will be what is currently stored, ignoring any pending updates or deletes.
    pub fn get_feature_flag_by_id(&self, feature_flag_id: impl Into<String>) -> GetFeatureFlagByIdRequest<'a> {
        GetFeatureFlagByIdRequest::new(self.client, feature_flag_id)
    }

    /// Delete the Feature Flag data currently stored for the given ID.
    ///
    /// Deletion is performed asynchronously. The getFeatureFlagById operation can be used to confirm that data has been deleted successfully (if needed).
    pub fn delete_feature_flag_by_id(&self, feature_flag_id: impl Into<String>) -> DeleteFeatureFlagByIdRequest<'a> {
        DeleteFeatureFlagByIdRequest::new(self.client, feature_flag_id)
    }
}

/// Update / insert Feature Flag data.
///
/// Feature Flags are identified by their ID, and existing Feature Flag data for the same ID will be replaced if it exists and the updateSequenceId of existing data is less than the incoming data.
///
/// Submissions are performed asynchronously. Submitted data will eventually be available in Jira; most updates are available within a short period of time, but may take some time during peak load and/or maintenance times. The getFeatureFlagById operation can be used to confirm that data has been stored successfully (if needed).
///
/// In the case of multiple Feature Flags being submitted in one request, each is validated individually prior to submission. Details of which Feature Flags failed submission (if any) are available in the response object.
pub struct SubmitFeatureFlagsRequest<'a> {
    client: &'a crate::core::Client,
    properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    flags: Vec<SubmitFeatureFlagsRequestFlags>,
    provider_metadata: Option<SubmitFeatureFlagsRequestProviderMetadata>,
}

impl<'a> SubmitFeatureFlagsRequest<'a> {
    fn new(client: &'a crate::core::Client, flags: impl IntoIterator<Item = SubmitFeatureFlagsRequestFlags>) -> Self {
        Self { client, flags: flags.into_iter().collect(), properties: None, provider_metadata: None }
    }

    /// Properties assigned to Feature Flag data that can then be used for delete / query operations.
    ///
    /// Examples might be an account or user ID that can then be used to clean up data if an account is removed from the Provider system.
    ///
    /// Note that these properties will never be returned with Feature Flag data. They are not intended for use as metadata to associate with a Feature Flag. Internally they are stored as a hash so that personal information etc. is never stored within Jira.
    ///
    /// Properties are supplied as key/value pairs, a maximum of 5 properties can be supplied, and keys must not contain ':' or start with '_'.
    #[must_use]
    pub fn properties(mut self, value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);

        self
    }

    /// Information about the provider. This is useful for auditing, logging, debugging,
    /// and other internal uses. It is not considered private information. Hence, it may not contain personally
    /// identifiable information.
    #[must_use]
    pub fn provider_metadata(mut self, value: SubmitFeatureFlagsRequestProviderMetadata) -> Self {
        self.provider_metadata = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/featureflags/0.1/bulk".to_owned());

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.properties {
            body.insert("properties".to_owned(), serde_json::to_value(value)?);
        }

        body.insert("flags".to_owned(), serde_json::to_value(&self.flags)?);

        if let Some(value) = &self.provider_metadata {
            body.insert("providerMetadata".to_owned(), serde_json::to_value(value)?);
        }

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SubmitFeatureFlags> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Bulk delete all Feature Flags that match the given request.
///
/// One or more query params must be supplied to specify Properties to delete by. Optional param `_updateSequenceId` is no longer supported.
/// If more than one Property is provided, data will be deleted that matches ALL of the Properties (e.g. treated as an AND).
/// See the documentation for the submitFeatureFlags operation for more details.
///
/// e.g. DELETE /bulkByProperties?accountId=account-123&createdBy=user-456
///
/// Deletion is performed asynchronously. The getFeatureFlagById operation can be used to confirm that data has been deleted successfully (if needed).
pub struct DeleteFeatureFlagsByPropertyRequest<'a> {
    client: &'a crate::core::Client,
    account_id: String,
    created_by: Option<String>,
}

impl<'a> DeleteFeatureFlagsByPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, account_id: impl Into<String>) -> Self {
        Self { client, account_id: account_id.into(), created_by: None }
    }

    /// Optional additional property filter combined with accountId (AND). Must match a key previously supplied in submitFeatureFlags `properties`. Example: createdBy=user-456.
    #[must_use]
    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            "/rest/featureflags/0.1/bulkByProperties".to_owned(),
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

/// Retrieve the currently stored Feature Flag data for the given ID.
///
/// The result will be what is currently stored, ignoring any pending updates or deletes.
pub struct GetFeatureFlagByIdRequest<'a> {
    client: &'a crate::core::Client,
    feature_flag_id: String,
}

impl<'a> GetFeatureFlagByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, feature_flag_id: impl Into<String>) -> Self {
        Self { client, feature_flag_id: feature_flag_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/featureflags/0.1/flag/{}", crate::core::encode_path_segment(&self.feature_flag_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetFeatureFlagById> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete the Feature Flag data currently stored for the given ID.
///
/// Deletion is performed asynchronously. The getFeatureFlagById operation can be used to confirm that data has been deleted successfully (if needed).
pub struct DeleteFeatureFlagByIdRequest<'a> {
    client: &'a crate::core::Client,
    feature_flag_id: String,
}

impl<'a> DeleteFeatureFlagByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, feature_flag_id: impl Into<String>) -> Self {
        Self { client, feature_flag_id: feature_flag_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/featureflags/0.1/flag/{}", crate::core::encode_path_segment(&self.feature_flag_id)),
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
