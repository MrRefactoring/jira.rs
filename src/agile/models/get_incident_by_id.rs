// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The IncidentData schema version used for this incident data.
    ///
    /// Placeholder to support potential schema changes in the future.
    pub enum GetIncidentByIdSchemaVersion {
        N10 => "1.0",
    }
}

crate::open_enum! {
    /// The severity level of the Incident with P1 being the highest and P5 being the lowest
    pub enum GetIncidentByIdSeverityLevel {
        P1 => "P1",
        P2 => "P2",
        P3 => "P3",
        P4 => "P4",
        P5 => "P5",
        Unknown => "unknown",
    }
}

/// Severity information for a single Incident.
///
/// This is the severity information that will be presented to the user on e.g. the Jira Incidents screen.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIncidentByIdSeverity {
    /// The severity level of the Incident with P1 being the highest and P5 being the lowest
    pub level: GetIncidentByIdSeverityLevel,
}

crate::open_enum! {
    /// The current status of the Incident.
    pub enum GetIncidentByIdStatus {
        Open => "open",
        Resolved => "resolved",
        Unknown => "unknown",
    }
}

crate::open_enum! {
    /// the type of the association being made
    pub enum GetIncidentByIdAssociationsAssociationType {
        IssueIdOrKeys => "issueIdOrKeys",
        ServiceIdOrKeys => "serviceIdOrKeys",
        AtiCloudCompassEventSource => "ati:cloud:compass:event-source",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIncidentByIdAssociations {
    /// the type of the association being made
    #[serde(rename = "associationType", default, skip_serializing_if = "Option::is_none")]
    pub association_type: Option<GetIncidentByIdAssociationsAssociationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Data related to a specific incident in a specific container that the incident is present in. Must specify at least one association to a component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIncidentById {
    /// The IncidentData schema version used for this incident data.
    ///
    /// Placeholder to support potential schema changes in the future.
    #[serde(rename = "schemaVersion")]
    pub schema_version: GetIncidentByIdSchemaVersion,
    /// The identifier for the Incident. Must be unique for a given Provider.
    pub id: String,
    /// An ID used to apply an ordering to updates for this Incident in the case of out-of-order receipt of update requests.
    ///
    /// This can be any monotonically increasing number. A suggested implementation is to use epoch millis from the Provider system, but other alternatives are valid (e.g. a Provider could store a counter against each Incident and increment that on each update to Jira).
    ///
    /// Updates for a Incident that are received with an updateSqeuenceId lower than what is currently stored will be ignored.
    #[serde(rename = "updateSequenceNumber")]
    pub update_sequence_number: i64,
    /// The IDs of the Components impacted by this Incident. Must be unique for a given Provider.
    #[serde(rename = "affectedComponents")]
    pub affected_components: Vec<String>,
    /// The human-readable summary for the Incident. Will be shown in the UI.
    ///
    /// If not provided, will use the ID for display.
    pub summary: String,
    /// A description of the issue in Markdown format. Will be shown in the UI and used when creating Jira Issues.
    pub description: String,
    /// A URL users can use to link to a summary view of this incident, if appropriate.
    ///
    /// This could be any location that makes sense in the Provider system (e.g. if the summary information comes from a specific project, it might make sense to link the user to the incident in that project).
    pub url: String,
    /// The timestamp to present to the user that shows when the Incident was raised.
    ///
    /// Expected format is an RFC3339 formatted string.
    #[serde(rename = "createdDate", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub created_date: String,
    /// The last-updated timestamp to present to the user the last time the Incident was updated.
    ///
    /// Expected format is an RFC3339 formatted string.
    #[serde(rename = "lastUpdated", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub last_updated: String,
    /// Severity information for a single Incident.
    ///
    /// This is the severity information that will be presented to the user on e.g. the Jira Incidents screen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<GetIncidentByIdSeverity>,
    /// The current status of the Incident.
    pub status: GetIncidentByIdStatus,
    /// The IDs of the Jira issues related to this Incident. Must be unique for a given Provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<GetIncidentByIdAssociations>>,
}
