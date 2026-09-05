// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The feature state.
    pub enum ProjectFeatureStateState {
        Enabled => "ENABLED",
        Disabled => "DISABLED",
        ComingSoon => "COMING_SOON",
    }
}

/// Details of the feature state.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectFeatureState {
    /// The feature state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ProjectFeatureStateState>,
}
