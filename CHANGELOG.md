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
- Wiki markup where v3 wants a document: a rich-text field written as a plain string goes to the v2 twin of the
  endpoint, which converts it, and the result is read back through v3.

### Fixed

- A value interpolated into a request path is escaped. A property key holding a `/` used to split the
  segment in two, a `?` turned the rest of the path into a query string, and `..` addressed a different
  endpoint altogether.

- A request body the specification types as binary is sent as bytes rather than as a JSON array of them, and carries
  a `content_type` the caller declares — Jira reads the declared type rather than sniffing the bytes.
- A `date-time` field reads a number as well as a string. Jira declares every timestamp a string and the bulk queue
  answers epoch milliseconds.
- A generated union ends in a catch-all, so a shape the specification does not list no longer fails the response.
