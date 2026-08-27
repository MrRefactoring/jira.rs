// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailDomains {
    /// The list of email domains to filter by, eg: for `abc@hello.com`, emailDomain is `hello.com`. Sample query param `{"emailDomains":{"eq":["hello.com"]}}`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    /// Partial email domain filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
}
