// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The [project type](https://confluence.atlassian.com/x/GwiiLQ#Jiraapplicationsoverview-Productfeaturesandprojecttypes) of the project.
    pub enum ProjectDetailsProjectTypeKey {
        Software => "software",
        ServiceDesk => "service_desk",
        Business => "business",
        ProductDiscovery => "product_discovery",
        CustomerService => "customer_service",
    }
}

/// Details about a project.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectDetails {
    #[serde(rename = "avatarUrls", default, skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<AvatarUrls>,
    /// The ID of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectCategory", default, skip_serializing_if = "Option::is_none")]
    pub project_category: Option<UpdatedProjectCategory>,
    /// The [project type](https://confluence.atlassian.com/x/GwiiLQ#Jiraapplicationsoverview-Productfeaturesandprojecttypes) of the project.
    #[serde(rename = "projectTypeKey", default, skip_serializing_if = "Option::is_none")]
    pub project_type_key: Option<ProjectDetailsProjectTypeKey>,
    /// The URL of the project details.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Whether or not the project is simplified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simplified: Option<bool>,
}
