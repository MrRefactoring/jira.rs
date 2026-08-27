# jira

[![crates.io](https://img.shields.io/crates/v/jira.svg?style=flat-square)](https://crates.io/crates/jira)
[![docs.rs](https://img.shields.io/docsrs/jira?style=flat-square)](https://docs.rs/jira)
[![build status](https://img.shields.io/github/actions/workflow/status/mrrefactoring/jira.rs/.github/workflows/ci.yaml?branch=master&style=flat-square)](https://github.com/MrRefactoring/jira.rs/actions/workflows/ci.yaml)
[![license](https://img.shields.io/crates/l/jira?style=flat-square)](https://github.com/MrRefactoring/jira.rs/blob/master/LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.91-blue?style=flat-square&logo=rust)](https://blog.rust-lang.org/)

> 🌐 **English** · [Русский](README.ru.md)

Rust client for the Atlassian Jira REST APIs — the Rust counterpart of
[jira.js](https://github.com/MrRefactoring/jira.js). The transport is written by hand; every operation and model is
generated from the same OpenAPI pipeline that produces `jira.js`, so the two cannot drift on anything but the language.
Five hundred and sixty-three live cases run against a real Jira site and the Data Center rigs in Docker, and they are
what found the defects the type checker could not.

## Installation

```sh
cargo add jira
```

Requires Rust 1.91 or newer, and a Tokio runtime.

## Quick example

```rust,no_run
use jira::{Auth, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .host("https://your-domain.atlassian.net")
        .auth(Auth::api_token("you@example.com", "YOUR_API_TOKEN"))
        .build()?;

    let jira = jira::cloud::CloudClient::new(client);

    let myself = jira.myself().get_current_user().send().await?;

    println!("{}", myself.display_name.unwrap_or_default());

    Ok(())
}
```

`host` is the bare site URL — the API path belongs to the request, not here.

Build the transport **once** and hand it to each surface. Under OAuth 2.0 this matters: two clients mean two token
states, and since Atlassian rotates the refresh token on every refresh, whichever refreshes first invalidates the
other\'s copy.

```rust,no_run
# use jira::{Auth, Client};
# fn example(client: Client) {
let jira = jira::cloud::CloudClient::new(client.clone());
let agile = jira::agile::AgileClient::new(client);
# }
```

Every operation is a builder: what the API requires is an argument, what it merely accepts is a method.

```rust,no_run
# use jira::cloud::CloudClient;
# async fn example(jira: &CloudClient) -> jira::Result<()> {
let issues = jira
    .issue_search()
    .search_and_reconsile_issues_using_jql()
    .jql("project = PROJ ORDER BY created DESC")
    .max_results(50)
    .fields(["summary", "status"])
    .send()
    .await?;
# Ok(())
# }
```

## Authentication

```rust
use jira::{Auth, core::{OAuth2Config, OAuth2ServerConfig}};

// Jira Cloud: an account address and an API token minted for it.
let basic = Auth::api_token("you@example.com", "YOUR_API_TOKEN");

// Data Center: a personal access token, which 8.14 and later prefer.
let bearer = Auth::bearer("YOUR_PAT");

// Data Center: a local account name and its password.
let password = Auth::password("username", "password");

// Jira Cloud OAuth 2.0 (3LO). The client refreshes ahead of expiry, retries a 401 once, and routes through
// the Atlassian gateway, so `host` is not needed.
let oauth = Auth::oauth2(OAuth2Config {
    refresh_token: Some("...".to_owned()),
    client_id: Some("...".to_owned()),
    client_secret: Some("...".to_owned()),
    ..OAuth2Config::default()
});

// Data Center OAuth 2.0, against the instance's own provider.
let oauth_server = Auth::oauth2_server(OAuth2ServerConfig {
    refresh_token: Some("...".to_owned()),
    client_id: Some("...".to_owned()),
    client_secret: Some("...".to_owned()),
    redirect_uri: Some("https://app.example.com/callback".to_owned()),
    ..OAuth2ServerConfig::default()
});
```

Atlassian rotates the refresh token on every refresh. Persist the new one through `on_token_refresh`, or the next
refresh fails.

## Errors

Every failure is a `jira::Error`. Branch on the predicates rather than the variant — they read the status and the
OAuth code for you:

```rust,no_run
# use jira::{Client, Error};
# async fn example(client: &Client) {
match client.get("/rest/api/3/issue/PROJ-1").send::<serde_json::Value>().await {
    Ok(issue) => println!("{issue}"),
    Err(error) if error.is_not_found() => println!("no such issue, or no permission to know"),
    Err(error) if error.is_rate_limit() => println!("wait {:?}", error.retry_after()),
    Err(error) if error.is_reauthorization_required() => println!("the grant is gone; authorize again"),
    Err(error) => eprintln!("{error}"),
}
# }
```

| Predicate | Means |
|---|---|
| `is_auth` | 401 — credentials missing, expired or rejected |
| `is_scope` | 401 with a scope the token never asked for; refreshing cannot help |
| `is_forbidden` | 403 — authenticated, not permitted |
| `is_not_found` | 404 — absent, or invisible to you |
| `is_rate_limit` | 429 — read `retry_after()` |
| `is_server` | 5xx |
| `is_network` | no HTTP answer at all |
| `is_oauth` | the token endpoint refused, or the cloud id would not resolve |
| `is_config` | the client cannot work as configured |
| `is_schema_mismatch` | a 2xx whose body is not what the type describes |

A refused credential is not always a 401: an endpoint permitting anonymous access answers `200` with an
anonymous-scope body and reports the refusal only in `X-Seraph-LoginReason`. That is read too, and reported with the
status that was actually on the wire.

## Retry

Off by default. It covers transient transport failures and 502/503/504 only — never a 4xx, never a 429, never a 500:

```rust,no_run
# use jira::{Client, RetryConfig};
# use std::time::Duration;
let client = Client::builder()
    .host("https://your-domain.atlassian.net")
    .retry(RetryConfig { max_attempts: 3, initial_delay: Duration::from_millis(500), backoff_factor: 2.0 })
    .build()?;
# Ok::<(), jira::Error>(())
```

`jira::with_retry` applies the same policy around a call you already have.

## Cancellation, proxies and timeouts

Cancellation is Rust's own: drop the future, or wrap it in `tokio::time::timeout`. Everything else the transport
offers goes through your own `reqwest::Client`:

```rust,no_run
# use jira::Client;
let http = reqwest::Client::builder()
    .proxy(reqwest::Proxy::all("http://proxy.internal:8080")?)
    .timeout(std::time::Duration::from_secs(30))
    .build()?;

let client = Client::builder().host("https://your-domain.atlassian.net").http_client(http).build()?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Feature flags

| Feature | What it adds |
|---|---|
| `audit` | Collects the gaps between the generated types and what the API actually sends. For this crate's own audit run. |

| Feature | Surface |
|---|---|
| `cloud` (default) | Jira Cloud platform — issues, projects, fields, workflows |
| `agile` | Jira Agile — boards, sprints, backlog |
| `service-desk` | Jira Service Management |
| `server` | Jira Data Center, platform and Agile in one surface |
| `service-desk-server` | Jira Service Management Data Center |
| `assets` / `assets-server` | Assets, on Cloud and Data Center |
| `admin` | Organization administration |
| `teams` | Teams |
| `user-management` / `user-provisioning` | User management and SCIM provisioning |
| `webhooks` | Event and payload types, and the signature check that says a delivery came from Jira |

A surface you do not enable is not compiled: the whole crate is ten thousand types, and almost nobody needs all of
them.

## Other products

- [jira.js](https://github.com/MrRefactoring/jira.js) — the same APIs for Node.js and browsers
- [confluence.js](https://github.com/MrRefactoring/confluence.js)
- [trello.js](https://github.com/MrRefactoring/trello.js)

## License

MIT
