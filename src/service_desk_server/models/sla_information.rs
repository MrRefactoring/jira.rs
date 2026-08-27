// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlaInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<SelfLink>,
    #[serde(rename = "completedCycles", default, skip_serializing_if = "Option::is_none")]
    pub completed_cycles: Option<Vec<SlaInformationCompletedCycle>>,
    #[serde(rename = "ongoingCycle", default, skip_serializing_if = "Option::is_none")]
    pub ongoing_cycle: Option<SlaInformationOngoingCycle>,
}
