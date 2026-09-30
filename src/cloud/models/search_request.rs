// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Determines how to validate the JQL query and treat the validation results. Supported values:
    ///
    ///  *  `strict` Returns a 400 response code if any errors are found, along with a list of all errors (and warnings).
    ///  *  `warn` Returns all errors as warnings.
    ///  *  `none` No validation is performed.
    ///  *  `true` *Deprecated* A legacy synonym for `strict`.
    ///  *  `false` *Deprecated* A legacy synonym for `warn`.
    ///
    /// The default is `strict`.
    ///
    /// Note: If the JQL is not correctly formed a 400 response code is returned, regardless of the `validateQuery` value.
    pub enum SearchRequestValidateQuery {
        Strict => "strict",
        Warn => "warn",
        None => "none",
        True => "true",
        False => "false",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SearchRequest {
    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about issues in the response. Note that, unlike the majority of instances where `expand` is specified, `expand` is defined as a list of values. The expand options are:
    ///
    ///  *  `renderedFields` Returns field values rendered in HTML format.
    ///  *  `names` Returns the display name of each field.
    ///  *  `schema` Returns the schema describing a field type.
    ///  *  `transitions` Returns all possible transitions for the issue.
    ///  *  `operations` Returns all possible operations for the issue.
    ///  *  `editmeta` Returns information about how each field can be edited.
    ///  *  `changelog` Returns a list of recent updates to an issue, sorted by date, starting from the most recent.
    ///  *  `versionedRepresentations` Instead of `fields`, returns `versionedRepresentations` a JSON array containing each version of a field's value, with the highest numbered item representing the most recent version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
    /// A list of fields to return for each issue, use it to retrieve a subset of fields. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `*all` Returns all fields.
    ///  *  `*navigable` Returns navigable fields.
    ///  *  Any issue field, prefixed with a minus to exclude.
    ///
    /// The default is `*navigable`.
    ///
    /// Examples:
    ///
    ///  *  `summary,comment` Returns the summary and comments fields only.
    ///  *  `-description` Returns all navigable (default) fields except description.
    ///  *  `*all,-comment` Returns all fields except comments.
    ///
    /// Multiple `fields` parameters can be included in a request.
    ///
    /// Note: All navigable fields are returned by default. This differs from [GET issue](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issues/#api-rest-api-3-issue-issueIdOrKey-get) where the default is all fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// Reference fields by their key (rather than ID). The default is `false`.
    #[serde(rename = "fieldsByKeys", default, skip_serializing_if = "Option::is_none")]
    pub fields_by_keys: Option<bool>,
    /// A [JQL](https://confluence.atlassian.com/x/egORLQ) expression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    /// The maximum number of items to return per page.
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// A list of up to 5 issue properties to include in the results. This parameter accepts a comma-separated list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,
    /// The index of the first item to return in the page of results (page offset). The base index is `0`.
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// Determines how to validate the JQL query and treat the validation results. Supported values:
    ///
    ///  *  `strict` Returns a 400 response code if any errors are found, along with a list of all errors (and warnings).
    ///  *  `warn` Returns all errors as warnings.
    ///  *  `none` No validation is performed.
    ///  *  `true` *Deprecated* A legacy synonym for `strict`.
    ///  *  `false` *Deprecated* A legacy synonym for `warn`.
    ///
    /// The default is `strict`.
    ///
    /// Note: If the JQL is not correctly formed a 400 response code is returned, regardless of the `validateQuery` value.
    #[deprecated(note = "* `true` *Deprecated* A legacy synonym for `strict`.")]
    #[serde(rename = "validateQuery", default, skip_serializing_if = "Option::is_none")]
    pub validate_query: Option<SearchRequestValidateQuery>,
}
