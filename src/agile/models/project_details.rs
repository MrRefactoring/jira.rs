// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectDetailsAvatarUrls {
    /// The URL of the item's 16x16 pixel avatar.
    #[serde(rename = "16x16", default, skip_serializing_if = "Option::is_none")]
    pub n16x16: Option<String>,
    /// The URL of the item's 24x24 pixel avatar.
    #[serde(rename = "24x24", default, skip_serializing_if = "Option::is_none")]
    pub n24x24: Option<String>,
    /// The URL of the item's 32x32 pixel avatar.
    #[serde(rename = "32x32", default, skip_serializing_if = "Option::is_none")]
    pub n32x32: Option<String>,
    /// The URL of the item's 48x48 pixel avatar.
    #[serde(rename = "48x48", default, skip_serializing_if = "Option::is_none")]
    pub n48x48: Option<String>,
}

/// A project category.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectDetailsProjectCategory {
    /// The name of the project category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the project category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The description of the project category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the project category.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

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
#[non_exhaustive]
pub struct ProjectDetails {
    #[serde(rename = "avatarUrls", default, skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<ProjectDetailsAvatarUrls>,
    /// The ID of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A project category.
    #[serde(rename = "projectCategory", default, skip_serializing_if = "Option::is_none")]
    pub project_category: Option<ProjectDetailsProjectCategory>,
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
