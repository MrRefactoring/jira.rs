// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The searcher that defines the way the field is searched in Jira. It can be set to `null`, otherwise you must specify the valid searcher for the field type, as listed below (abbreviated values shown):
    ///
    ///  *  `cascadingselect`: `cascadingselectsearcher`
    ///  *  `datepicker`: `daterange`
    ///  *  `datetime`: `datetimerange`
    ///  *  `float`: `exactnumber` or `numberrange`
    ///  *  `grouppicker`: `grouppickersearcher`
    ///  *  `importid`: `exactnumber` or `numberrange`
    ///  *  `labels`: `labelsearcher`
    ///  *  `multicheckboxes`: `multiselectsearcher`
    ///  *  `multigrouppicker`: `multiselectsearcher`
    ///  *  `multiselect`: `multiselectsearcher`
    ///  *  `multiuserpicker`: `userpickergroupsearcher`
    ///  *  `multiversion`: `versionsearcher`
    ///  *  `project`: `projectsearcher`
    ///  *  `radiobuttons`: `multiselectsearcher`
    ///  *  `readonlyfield`: `textsearcher`
    ///  *  `select`: `multiselectsearcher`
    ///  *  `textarea`: `textsearcher`
    ///  *  `textfield`: `textsearcher`
    ///  *  `url`: `exacttextsearcher`
    ///  *  `userpicker`: `userpickergroupsearcher`
    ///  *  `version`: `versionsearcher`
    pub enum UpdateCustomFieldDetailsSearcherKey {
        ComAtlassianJiraPluginSystemCustomfieldtypesCascadingselectsearcher => "com.atlassian.jira.plugin.system.customfieldtypes:cascadingselectsearcher",
        ComAtlassianJiraPluginSystemCustomfieldtypesDaterange => "com.atlassian.jira.plugin.system.customfieldtypes:daterange",
        ComAtlassianJiraPluginSystemCustomfieldtypesDatetimerange => "com.atlassian.jira.plugin.system.customfieldtypes:datetimerange",
        ComAtlassianJiraPluginSystemCustomfieldtypesExactnumber => "com.atlassian.jira.plugin.system.customfieldtypes:exactnumber",
        ComAtlassianJiraPluginSystemCustomfieldtypesExacttextsearcher => "com.atlassian.jira.plugin.system.customfieldtypes:exacttextsearcher",
        ComAtlassianJiraPluginSystemCustomfieldtypesGrouppickersearcher => "com.atlassian.jira.plugin.system.customfieldtypes:grouppickersearcher",
        ComAtlassianJiraPluginSystemCustomfieldtypesLabelsearcher => "com.atlassian.jira.plugin.system.customfieldtypes:labelsearcher",
        ComAtlassianJiraPluginSystemCustomfieldtypesMultiselectsearcher => "com.atlassian.jira.plugin.system.customfieldtypes:multiselectsearcher",
        ComAtlassianJiraPluginSystemCustomfieldtypesNumberrange => "com.atlassian.jira.plugin.system.customfieldtypes:numberrange",
        ComAtlassianJiraPluginSystemCustomfieldtypesProjectsearcher => "com.atlassian.jira.plugin.system.customfieldtypes:projectsearcher",
        ComAtlassianJiraPluginSystemCustomfieldtypesTextsearcher => "com.atlassian.jira.plugin.system.customfieldtypes:textsearcher",
        ComAtlassianJiraPluginSystemCustomfieldtypesUserpickergroupsearcher => "com.atlassian.jira.plugin.system.customfieldtypes:userpickergroupsearcher",
        ComAtlassianJiraPluginSystemCustomfieldtypesVersionsearcher => "com.atlassian.jira.plugin.system.customfieldtypes:versionsearcher",
    }
}

/// Details of a custom field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCustomFieldDetails {
    /// The description of the custom field. The maximum length is 40000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the custom field. It doesn't have to be unique. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The searcher that defines the way the field is searched in Jira. It can be set to `null`, otherwise you must specify the valid searcher for the field type, as listed below (abbreviated values shown):
    ///
    ///  *  `cascadingselect`: `cascadingselectsearcher`
    ///  *  `datepicker`: `daterange`
    ///  *  `datetime`: `datetimerange`
    ///  *  `float`: `exactnumber` or `numberrange`
    ///  *  `grouppicker`: `grouppickersearcher`
    ///  *  `importid`: `exactnumber` or `numberrange`
    ///  *  `labels`: `labelsearcher`
    ///  *  `multicheckboxes`: `multiselectsearcher`
    ///  *  `multigrouppicker`: `multiselectsearcher`
    ///  *  `multiselect`: `multiselectsearcher`
    ///  *  `multiuserpicker`: `userpickergroupsearcher`
    ///  *  `multiversion`: `versionsearcher`
    ///  *  `project`: `projectsearcher`
    ///  *  `radiobuttons`: `multiselectsearcher`
    ///  *  `readonlyfield`: `textsearcher`
    ///  *  `select`: `multiselectsearcher`
    ///  *  `textarea`: `textsearcher`
    ///  *  `textfield`: `textsearcher`
    ///  *  `url`: `exacttextsearcher`
    ///  *  `userpicker`: `userpickergroupsearcher`
    ///  *  `version`: `versionsearcher`
    #[serde(rename = "searcherKey", default, skip_serializing_if = "Option::is_none")]
    pub searcher_key: Option<UpdateCustomFieldDetailsSearcherKey>,
}
