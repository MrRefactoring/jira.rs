// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Card layout configuration.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CardLayout {
    /// Whether to show days in column
    #[serde(rename = "showDaysInColumn", default, skip_serializing_if = "Option::is_none")]
    pub show_days_in_column: Option<bool>,
}
