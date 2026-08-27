// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ClusterStateState {
        Stable => "STABLE",
        ReadyToUpgrade => "READY_TO_UPGRADE",
        Mixed => "MIXED",
        ReadyToRunUpgradeTasks => "READY_TO_RUN_UPGRADE_TASKS",
        RunningUpgradeTasks => "RUNNING_UPGRADE_TASKS",
        UpgradeTasksFailed => "UPGRADE_TASKS_FAILED",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClusterState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<NodeBuildInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ClusterStateState>,
}
