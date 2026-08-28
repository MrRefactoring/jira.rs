// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The direction in which to order the results.
    pub enum JqlQueryOrderByClauseElementDirection {
        Asc => "asc",
        Desc => "desc",
    }
}

/// An element of the order-by JQL clause.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JqlQueryOrderByClauseElement {
    /// The direction in which to order the results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<JqlQueryOrderByClauseElementDirection>,
    pub field: JqlQueryField,
}
