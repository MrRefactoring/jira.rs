# Changelog

All notable changes to this crate are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate follows
[Semantic Versioning](https://semver.org/spec/v2.0.0.html) — before `1.0.0`, a breaking change raises the minor
version.

## [Unreleased]

### Added

- The transport: a client carrying auth, retry policy and error handling, shared by every API surface.
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
