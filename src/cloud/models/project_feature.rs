// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The state of the feature. When updating the state of a feature, only ENABLED and DISABLED are supported. Responses can contain all values
    pub enum ProjectFeatureState2 {
        Enabled => "ENABLED",
        Disabled => "DISABLED",
        ComingSoon => "COMING_SOON",
    }
}

/// Details of a project feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectFeature {
    /// The key of the feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// URI for the image representing the feature.
    #[serde(rename = "imageUri", default, skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    /// Localized display description for the feature.
    #[serde(rename = "localisedDescription", default, skip_serializing_if = "Option::is_none")]
    pub localised_description: Option<String>,
    /// Localized display name for the feature.
    #[serde(rename = "localisedName", default, skip_serializing_if = "Option::is_none")]
    pub localised_name: Option<String>,
    /// List of keys of the features required to enable the feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerequisites: Option<Vec<String>>,
    /// The ID of the project.
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// The state of the feature. When updating the state of a feature, only ENABLED and DISABLED are supported. Responses can contain all values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ProjectFeatureState2>,
    /// Whether the state of the feature can be updated.
    #[serde(rename = "toggleLocked", default, skip_serializing_if = "Option::is_none")]
    pub toggle_locked: Option<bool>,
}
