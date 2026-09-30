// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ProjectFeatures operations.
pub struct ProjectFeaturesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectFeaturesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the list of features for a project.
    pub fn get_features_for_project(&self, project_id_or_key: impl Into<String>) -> GetFeaturesForProjectRequest<'a> {
        GetFeaturesForProjectRequest::new(self.client, project_id_or_key)
    }

    /// Sets the state of a project feature.
    pub fn toggle_feature_for_project(
        &self,
        project_id_or_key: impl Into<String>,
        feature_key: impl Into<String>,
        project_feature_state: ProjectFeatureState,
    ) -> ToggleFeatureForProjectRequest<'a> {
        ToggleFeatureForProjectRequest::new(self.client, project_id_or_key, feature_key, project_feature_state)
    }
}

/// Returns the list of features for a project.
#[derive(Clone)]
pub struct GetFeaturesForProjectRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
}

impl<'a> GetFeaturesForProjectRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/project/{}/features", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ContainerForProjectFeatures> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the state of a project feature.
#[derive(Clone)]
pub struct ToggleFeatureForProjectRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    feature_key: String,
    project_feature_state: ProjectFeatureState,
}

impl<'a> ToggleFeatureForProjectRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_id_or_key: impl Into<String>,
        feature_key: impl Into<String>,
        project_feature_state: ProjectFeatureState,
    ) -> Self {
        Self {
            client,
            project_id_or_key: project_id_or_key.into(),
            feature_key: feature_key.into(),
            project_feature_state,
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/3/project/{}/features/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
                crate::core::encode_path_segment(&self.feature_key)
            ),
        );

        let body = match serde_json::to_value(&self.project_feature_state)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ContainerForProjectFeatures> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
