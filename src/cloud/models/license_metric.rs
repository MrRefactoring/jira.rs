// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A metric that provides insight into the active licence details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LicenseMetric {
    /// The key of a specific license metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The calculated value of a licence metric linked to the key. An example licence metric is the approximate number of user accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
