// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Content {
    /// Url containing the body of the article (without title), suitable for rendering in an iframe
    #[serde(rename = "iframeSrc", default, skip_serializing_if = "Option::is_none")]
    pub iframe_src: Option<String>,
}
