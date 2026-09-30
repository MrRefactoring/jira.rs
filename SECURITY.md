# Security

## Reporting a vulnerability

Report it privately through
[GitHub's advisory form](https://github.com/MrRefactoring/jira.rs/security/advisories/new) rather than in a public
issue. You will hear back within seven days, and a fix ships as a patch release with the advisory published alongside
it.

## Supported versions

The latest release on crates.io. Before `1.0.0` there are no maintenance lines: a fix goes into the next release and
nowhere else.

## What counts

Anything that lets a credential, a token or a request body reach somewhere it should not — a log line, a `Debug`
rendering, an error message, a URL — as well as anything in the transport that weakens TLS, and any input the client
accepts that can turn into a request the caller did not write. The JQL builder, the query-string serializer and the
webhook signature check are the places to look first.

A response Jira sends that the generated types read wrongly is a bug, not a vulnerability: open an issue.
