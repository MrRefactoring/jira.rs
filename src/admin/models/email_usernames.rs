// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailUsernames {
    /// The list of email usernames to filter by, eg: for `abc@hello.com`, emailUsername is `abc`. Sample query param `{"emailUsernames":{"eq":["abc"]}}`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    /// Partial email username filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
}
