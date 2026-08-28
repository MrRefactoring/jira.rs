//! Following a paginated search to its end.

use futures_util::stream::{self, BoxStream, StreamExt, TryStreamExt};

use crate::cloud::{Issue, SearchIssuesRequest};
use crate::core::{Error, Result};

impl<'a> SearchIssuesRequest<'a> {
    /// Every issue the query matches, one page fetched at a time.
    ///
    /// The search endpoint pages with an opaque token rather than an offset: the first page is asked for without one,
    /// and each answer carries the token for the next until the one that does not. This walks that to its end, so a
    /// caller writes the query rather than the loop.
    ///
    /// Reading it needs `TryStreamExt` in scope — re-exported as [`crate::futures_util`], so no dependency of your
    /// own is required — and the same field selection the single-page call needs:
    /// without `fields` the search answers with identifiers and nothing else.
    ///
    /// ```no_run
    /// use jira::futures_util::TryStreamExt;
    /// use jira::cloud::CloudClient;
    /// use jira::jql::field;
    ///
    /// # async fn example(jira: CloudClient) -> jira::Result<()> {
    /// let mut issues = jira
    ///     .issue_search()
    ///     .search_issues()
    ///     .jql(field("project").eq("PROJ").order_by_desc("created"))
    ///     .fields(["summary"])
    ///     .stream();
    ///
    /// while let Some(issue) = issues.try_next().await? {
    ///     println!("{}", issue.key.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// The stream is boxed so that reading it needs no pinning at the call site: one allocation for the whole walk,
    /// against one request per page.
    ///
    /// The stream ends when Jira stops handing back a token. It is not restartable: a token expires seven days after
    /// it was issued, and the pages either side of a pause can disagree, since the index moves under a running search.
    pub fn stream(self) -> BoxStream<'a, Result<Issue>> {
        stream::try_unfold(Some((self, None::<String>)), |state| async move {
            let Some((request, token)) = state else {
                return Ok::<_, Error>(None);
            };

            let mut page = request.clone();

            if let Some(token) = token {
                page = page.next_page_token(token);
            }

            let page = page.send().await?;
            let issues = page.issues.unwrap_or_default();

            Ok::<_, Error>(Some((issues, page.next_page_token.map(|token| (request, Some(token))))))
        })
        .map_ok(|issues| stream::iter(issues.into_iter().map(Ok)))
        .try_flatten()
        .boxed()
    }
}
