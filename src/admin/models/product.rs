// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Unique key of the Product
    pub enum ProductKey {
        JiraSoftware => "jira-software",
        JiraServiceDesk => "jira-service-desk",
        JiraCore => "jira-core",
        JiraOps => "jira-ops",
        Stride => "stride",
        Hipchat => "hipchat",
        Confluence => "confluence",
        Bitbucket => "bitbucket",
        Trello => "trello",
        Opsgenie => "opsgenie",
        Statuspage => "statuspage",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Product {
    /// Unique key of the Product
    pub key: ProductKey,
    /// Name of the Product
    pub name: String,
    /// URL of the Product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Last active date for a product
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_active: Option<chrono::DateTime<chrono::Utc>>,
    /// Last active date for a product
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub last_active: Option<String>,
}
