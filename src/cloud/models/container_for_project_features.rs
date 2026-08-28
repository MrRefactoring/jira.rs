// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The list of features on a project.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContainerForProjectFeatures {
    /// The project features.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<ProjectFeature>>,
}
