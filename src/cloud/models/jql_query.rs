// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A parsed JQL query.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JqlQuery {
    #[serde(rename = "orderBy", default, skip_serializing_if = "Option::is_none")]
    pub order_by: Option<JqlQueryOrderByClause>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#where: Option<Box<JqlQueryClause>>,
}
