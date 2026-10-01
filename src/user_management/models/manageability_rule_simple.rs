// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ManageabilityRuleSimple {
    ManageabilityAllowed(ManageabilityAllowed),
    ManageabilityUnallowed(ManageabilityUnallowed),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(ManageabilityRuleSimple { ManageabilityAllowed, ManageabilityUnallowed, Other });
