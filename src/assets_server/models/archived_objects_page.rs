// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// One page of archived objects.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ArchivedObjectsPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ArchivedObject>>,
}
