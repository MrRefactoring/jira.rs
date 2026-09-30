// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The renderer type for the field, null to preserve current renderer type
    pub enum FieldsSchemeItemParameterRendererType {
        JiraTextRenderer => "jira-text-renderer",
        AtlassianWikiRenderer => "atlassian-wiki-renderer",
    }
}

/// The default parameters to apply to the field across all work types in the specified schemes, may be null if only work type-specific updates are needed
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldsSchemeItemParameter {
    /// The custom description for the field, null to preserve current description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the field is required, null to preserve current requirement setting
    #[serde(rename = "isRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// The renderer type for the field, null to preserve current renderer type
    #[serde(rename = "rendererType", default, skip_serializing_if = "Option::is_none")]
    pub renderer_type: Option<FieldsSchemeItemParameterRendererType>,
}
