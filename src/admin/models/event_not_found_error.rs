// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum EventNotFoundErrorErrorsCode {
        Admin4044 => "ADMIN-404-4",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventNotFoundErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<EventNotFoundErrorErrorsCode>,
}

/// Event not found
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventNotFoundError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<EventNotFoundErrorErrors>>,
}
