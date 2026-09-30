// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApiError {
    /// A unique identifier for this particular occurrence of the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The code of the error. It can be one of the following values: - ADMIN-UAM-400-1: Bad Request - ADMIN-UAM-400-2: Inactive user - ADMIN-UAM-400-3: Invalid Role for Resource - ADMIN-UAM-400-4: Product not licensed - ADMIN-UAM-401-1: Unauthenticated - ADMIN-UAM-401-2: Unauthorized - ADMIN-UAM-402-1: Payment Required - ADMIN-UAM-403-1: Forbidden - ADMIN-UAM-403-2: Not allowed to manage the org - ADMIN-UAM-404-1: Unknown Resource - ADMIN-UAM-404-2: Organization not found - ADMIN-UAM-404-3: Group Not Found - ADMIN-UAM-405-1: Method Not Supported - ADMIN-UAM-409-1: Cannot create default group - ADMIN-UAM-409-2: Resource Conflicts - ADMIN-UAM-409-3: Product License Limit Exceeded - ADMIN-UAM-415-1: Unsupported Media Type - ADMIN-UAM-500-1: Internal Error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The HTTP status code applicable to this error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Human-readable summary of the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Human-readable explanation specific to this occurrence of the error, and a suggested action to resolve it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
