# Changelog

All notable changes to this crate are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate follows
[Semantic Versioning](https://semver.org/spec/v2.0.0.html) — before `1.0.0`, a breaking change raises the minor
version.

## [Unreleased]

### Added

- The transport: a client carrying auth, retry policy and error handling, shared by every API surface.
- `webhooks`: the events Jira sends, the payloads it posts, and `verify_signature`, which is what tells a
  delivery from Jira apart from a POST anyone can make to the same URL.
- Authentication by API token, by personal access token, by a token minted per request, by Jira Cloud OAuth 2.0 (3LO)
  and by a Data Center instance's own OAuth 2.0 provider. Refreshes are single-flighted and a 401 buys one retry.
- Eleven API surfaces behind a cargo feature each, generated from the same OpenAPI pipeline as `jira.js`: Cloud,
  Agile, Service Management, Assets, Data Center, Service Management Data Center, Assets Data Center, administration,
  Teams, user management and user provisioning.
- `jira::Error`, one enum with predicates that read the status and the OAuth code — including the `X-Seraph-LoginReason`
  header, which is how Jira reports a refused credential behind a `200`.
- `with_retry`, applying the client's own transient-failure policy around a call.
- `jira::jql`, which builds a query out of values that came from somewhere else. A value reaches Jira quoted and
  escaped rather than able to end the literal it sits in and add a clause of its own.
- `stream` on the JQL search, which follows the endpoint's page token to the last page so a caller writes the query
  rather than the loop.
- `Extensible`, on every type that carries fields the schema does not describe: `custom` reads them into a type of
  the caller's own, `with_custom` and `with` write them beside the described fields, and a key the schema already
  describes is refused rather than sent twice.
- Wiki markup written as a string into a description, an environment, a comment or a worklog reaches the `v2` twin
  of the endpoint, which is the one that reads it — at creation and at edit alike — and comes back as the document
  Jira parsed it into.
- `get_tenant_context`, resolving a site's cloud id, organization id and host name.
- The `audit` feature, which collects the fields the API sends that the generated types do not describe.
- The `chrono` feature, which turns every `date-time` into `Option<chrono::DateTime<Utc>>`. Off by default,
  because the type of a field is not something two crates depending on this one get to disagree about. The
  reader takes the spelling Atlassian documents — which is not RFC 3339 — as well as RFC 3339, an instant
  without an offset, a bare date, and the epoch milliseconds the bulk queue sends; a value it does not
  recognise becomes `None` rather than failing the response around it.
- `#[deprecated]` on the operations, parameters and fields Atlassian deprecates in prose rather than with the
  OpenAPI flag, which is how it deprecates nearly all of them.
- `#[non_exhaustive]` on the types a caller only ever reads, so a field Atlassian adds is a minor release. The
  types a caller builds stay open, so a request body can still be assembled with `..Default::default()`.
- `PartialEq` on every generated type.
- `reqwest` is re-exported, so a caller passing their own client through `ClientBuilder::http_client` can name
  the version this crate was built against.
- The JQL search is `search_issues` rather than `search_and_reconsile_issues_using_jql`: the specification misspells
  the operation and spells the model it answers with correctly, in the same document, and this is the most called
  operation in the API. The generator renames it, so `jira.js` carries the same name with the old one kept as a
  deprecated alias; nothing was published under the old name here.
- Wiki markup where v3 wants a document: a rich-text field written as a plain string goes to the v2 twin of the
  endpoint, which converts it, and the result is read back through v3.
- `IssueFields`: the fields of an issue are a struct rather than a map of raw JSON. The system fields Jira
  documents in prose — summary, description, project, issue type, status, assignee, the timestamps, links, subtasks,
  attachments, comments and the rest — are typed, and a custom field lands in `additional`, keyed the way the site
  names it. The same struct is what `create_issue` and `edit_issue` take.
- `stream` on every request that pages by offset — projects, boards, users, filters, dashboards and the rest across
  the five surfaces that page this way — so the JQL search is no longer the only listing a caller does not write the
  loop for.
- `error_messages` and `field_errors` on `Error`, reading the two shapes Jira puts a refusal in: the list of
  messages, and the map from a field to what was wrong with it.
- The `tracing` feature: a `jira.request` span around every request, with an event per attempt. Off by default, and
  nothing in it is a credential or a body.
- `SECURITY.md`, dependency updates through Dependabot, and `cargo deny` in CI, so an advisory against a dependency
  fails the build rather than waits to be noticed.
- Every public item of the transport carries documentation, and the crate warns on one that does not.
- The administration surface names a role, an account status, a membership status, a claim status and a
  combined status once each. The specification restates the same lists per operation, which used to become
  twenty-six types for six concepts, so a role read from one call could not be passed to another.

### Credits

- Harold Dost published `0.0.1` of this crate name in 2021. That release shares no code with this one, but it kept
  the name alive.

### Fixed

- A value interpolated into a request path is escaped. A property key holding a `/` used to split the
  segment in two, a `?` turned the rest of the path into a query string, and `..` addressed a different
  endpoint altogether.

- A request body the specification types as binary is sent as bytes rather than as a JSON array of them, and carries
  a `content_type` the caller declares — Jira reads the declared type rather than sniffing the bytes.
- A `date-time` field reads a number as well as a string. Jira declares every timestamp a string and the bulk queue
  answers epoch milliseconds.
- A generated union ends in a catch-all, so a shape the specification does not list no longer fails the response.
