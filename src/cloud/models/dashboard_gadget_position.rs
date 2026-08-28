// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of a gadget position.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DashboardGadgetPosition {
    #[serde(rename = "The column position of the gadget.")]
    pub the_column_position_of_the_gadget: i64,
    #[serde(rename = "The row position of the gadget.")]
    pub the_row_position_of_the_gadget: i64,
}
