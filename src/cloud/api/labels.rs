// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Labels operations.
pub struct LabelsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> LabelsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of labels.
    pub fn get_all_labels(&self) -> GetAllLabelsRequest<'a> {
        GetAllLabelsRequest::new(self.client)
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of labels.
#[derive(Clone)]
pub struct GetAllLabelsRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<i64>,
    max_results: Option<i64>,
    query: Option<String>,
    scope: Option<String>,
}

impl<'a> GetAllLabelsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start_at: None, max_results: None, query: None, scope: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Case-insensitive prefix used to filter labels before pagination.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// RECENT\_PROJECTS or ALL\_BROWSEABLE\_PROJECTS. Omitted scope retains the unbounded search. Blank or unrecognised values use ALL\_BROWSEABLE\_PROJECTS.
    #[must_use]
    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/label".to_owned());

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.scope {
            config.query.push(("scope".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<String>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values.unwrap_or_default();
            Ok(crate::core::PageStep::offset(
                items,
                page.start_at.unwrap_or(position),
                page.is_last.unwrap_or(false),
                page.total,
                None,
            ))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PageString> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
