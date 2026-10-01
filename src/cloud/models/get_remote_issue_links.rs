// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetRemoteIssueLinks {
    Variant0(Vec<RemoteIssueLink>),
    RemoteIssueLink(RemoteIssueLink),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(GetRemoteIssueLinks { Variant0, RemoteIssueLink, Other });
