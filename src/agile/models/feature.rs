// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum FeatureBoardFeature {
        SimpleRoadmap => "SIMPLE_ROADMAP",
        Backlog => "BACKLOG",
        Sprints => "SPRINTS",
        Calendar => "CALENDAR",
        Devtools => "DEVTOOLS",
        Reports => "REPORTS",
        Estimation => "ESTIMATION",
        Pages => "PAGES",
        Code => "CODE",
        Security => "SECURITY",
        Requests => "REQUESTS",
        Incidents => "INCIDENTS",
        Releases => "RELEASES",
        Deployments => "DEPLOYMENTS",
        IssueNavigator => "ISSUE_NAVIGATOR",
        OnCallSchedule => "ON_CALL_SCHEDULE",
        Board => "BOARD",
        Goals => "GOALS",
        ListView => "LIST_VIEW",
    }
}

crate::open_enum! {
    pub enum FeatureFeatureType {
        Basic => "BASIC",
        Estimation => "ESTIMATION",
    }
}

crate::open_enum! {
    pub enum FeatureState {
        Enabled => "ENABLED",
        Disabled => "DISABLED",
        ComingSoon => "COMING_SOON",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Feature {
    #[serde(rename = "boardFeature", default, skip_serializing_if = "Option::is_none")]
    pub board_feature: Option<FeatureBoardFeature>,
    #[serde(rename = "boardId", default, skip_serializing_if = "Option::is_none")]
    pub board_id: Option<i64>,
    #[serde(rename = "featureId", default, skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<String>,
    #[serde(rename = "featureType", default, skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<FeatureFeatureType>,
    #[serde(rename = "imageUri", default, skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    #[serde(rename = "learnMoreArticleId", default, skip_serializing_if = "Option::is_none")]
    pub learn_more_article_id: Option<String>,
    #[serde(rename = "learnMoreLink", default, skip_serializing_if = "Option::is_none")]
    pub learn_more_link: Option<String>,
    #[serde(rename = "localisedDescription", default, skip_serializing_if = "Option::is_none")]
    pub localised_description: Option<String>,
    #[serde(rename = "localisedGroup", default, skip_serializing_if = "Option::is_none")]
    pub localised_group: Option<String>,
    #[serde(rename = "localisedName", default, skip_serializing_if = "Option::is_none")]
    pub localised_name: Option<String>,
    #[serde(rename = "permissibleEstimationTypes", default, skip_serializing_if = "Option::is_none")]
    pub permissible_estimation_types: Option<Vec<EstimationConfiguration>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<FeatureState>,
    #[serde(rename = "toggleLocked", default, skip_serializing_if = "Option::is_none")]
    pub toggle_locked: Option<bool>,
}
