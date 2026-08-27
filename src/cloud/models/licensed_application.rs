// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The licensing plan.
    pub enum LicensedApplicationPlan {
        Unlicensed => "UNLICENSED",
        Free => "FREE",
        Paid => "PAID",
    }
}

/// Details about a licensed Jira application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensedApplication {
    /// The ID of the application.
    pub id: String,
    /// The licensing plan.
    pub plan: LicensedApplicationPlan,
}
