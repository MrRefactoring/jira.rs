// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A list of connected issues
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tickets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tickets: Option<Vec<Ticket>>,
    /// A query to find all the connected issues
    #[serde(rename = "allTicketsQuery")]
    pub all_tickets_query: String,
}
