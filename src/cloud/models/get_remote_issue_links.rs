// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetRemoteIssueLinks {
    Variant0(Vec<RemoteIssueLink>),
    RemoteIssueLink(RemoteIssueLink),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}
