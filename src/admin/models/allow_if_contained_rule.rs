// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Applicable when policy type is `ip-allowlist` or `data-residency`
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AllowIfContainedRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#in: Option<Vec<String>>,
}
