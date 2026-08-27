// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// An icon. If no icon is defined:
///
///  *  for a status icon, no status icon displays in Jira.
///  *  for the remote object icon, the default link icon displays in Jira.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Icon {
    /// The URL of the tooltip, used only for a status icon. If not set, the status icon in Jira is not clickable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The title of the icon. This is used as follows:
    ///
    ///  *  For a status icon it is used as a tooltip on the icon. If not set, the status icon doesn't display a tooltip in Jira.
    ///  *  For the remote object icon it is used in conjunction with the application name to display a tooltip for the link's icon. The tooltip takes the format "\\[application name\\] icon title". Blank itemsare excluded from the tooltip title. If both items are blank, the icon tooltop displays as "Web Link".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL of an icon that displays at 16x16 pixel in Jira.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url16x16: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
