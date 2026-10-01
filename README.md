# jira

[![crates.io](https://img.shields.io/crates/v/jira.svg?style=flat-square)](https://crates.io/crates/jira)
[![docs.rs](https://img.shields.io/docsrs/jira?style=flat-square)](https://docs.rs/jira)
[![build status](https://img.shields.io/github/actions/workflow/status/mrrefactoring/jira.rs/.github/workflows/ci.yaml?branch=master&style=flat-square)](https://github.com/MrRefactoring/jira.rs/actions/workflows/ci.yaml)
[![license](https://img.shields.io/crates/l/jira?style=flat-square)](https://github.com/MrRefactoring/jira.rs/blob/master/LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.91-blue?style=flat-square&logo=rust)](https://blog.rust-lang.org/)

> 🌐 **English** · [Русский](https://github.com/MrRefactoring/jira.rs/blob/master/README.ru.md)

A Rust client for the Atlassian Jira REST APIs, built as the Rust counterpart of
[jira.js](https://github.com/MrRefactoring/jira.js).

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

`host` is the bare site URL. The API path goes on each request.

Build the transport **once** and share it between surfaces. Under OAuth 2.0 each transport holds its own token, and
Atlassian rotates the refresh token on every refresh, so a second transport would invalidate the first.

```rust,no_run
use jira::Client;

fn surfaces(client: Client) {
    let jira = jira::cloud::CloudClient::new(client.clone());
    let agile = jira::agile::AgileClient::new(client);
}
```

Every operation is a builder: required parameters are arguments, optional ones are methods.

```rust,no_run
use jira::cloud::{CloudClient, SearchAndReconcileResults};

async fn latest_issues(jira: &CloudClient) -> jira::Result<SearchAndReconcileResults> {
    jira.issue_search()
        .search_issues()
        .jql("project = PROJ ORDER BY created DESC")
        .max_results(50)
        .fields(["summary", "status"])
        .send()
        .await
}
```

## Searching with JQL

`jira::jql` builds a query with every value quoted and escaped, so user input cannot change its structure.

```rust,no_run
use jira::cloud::{CloudClient, SearchAndReconcileResults};
use jira::jql::{field, func};

async fn my_open_issues(jira: &CloudClient, typed: &str) -> jira::Result<SearchAndReconcileResults> {
    let query = field("project").eq("PROJ")
        .and(field("summary").contains(typed))
        .and(field("status").not_in(["Done", "Closed"]))
        .and(field("assignee").eq(func("currentUser")))
        .order_by_desc("created");

    jira.issue_search().search_issues().jql(query).fields(["summary", "status"]).send().await
}
```

Without `fields` the search returns only identifiers. System fields are typed; custom fields arrive under their
site-specific keys, and `Extensible::custom` reads them into a type of your own:

```rust,no_run
use jira::Extensible;
use jira::cloud::SearchAndReconcileResults;
use serde::Deserialize;

#[derive(Deserialize)]
struct Estimation {
    #[serde(rename = "customfield_10016")]
    story_points: Option<f64>,
}

fn print_estimates(page: SearchAndReconcileResults) -> Result<(), serde_json::Error> {
    for issue in page.issues.unwrap_or_default() {
        let fields = issue.fields.unwrap_or_default();
        let estimation: Estimation = fields.custom()?;

        println!("{} — {} ({:?})", issue.key.unwrap_or_default(), fields.summary.unwrap_or_default(), estimation.story_points);
    }

    Ok(())
}
```

Writes take the same struct; `with_custom` adds your fields to it:

```rust,no_run
use jira::Extensible;
use jira::cloud::{CloudClient, IssueFields, IssueTypeDetails, IssueUpdateDetails, Project};
use serde::Serialize;

#[derive(Serialize)]
struct Estimation {
    #[serde(rename = "customfield_10016")]
    story_points: Option<f64>,
}

async fn create_estimated_task(jira: &CloudClient) -> jira::Result<()> {
    let fields = IssueFields {
        project: Some(Project { key: Some("PROJ".into()), ..Default::default() }),
        issuetype: Some(IssueTypeDetails { name: Some("Task".into()), ..Default::default() }),
        summary: Some("Rotate the signing key".into()),
        ..Default::default()
    }
    .with_custom(Estimation { story_points: Some(5.0) })?;

    jira.issues()
        .create_issue(IssueUpdateDetails { fields: Some(fields), ..Default::default() })
        .send()
        .await?;

    Ok(())
}
```

A key the struct already has, such as `summary`, is refused. `with` sets a single key.

`stream` follows the search's page token to the last page:

```rust,no_run
use jira::cloud::CloudClient;
use jira::futures_util::TryStreamExt;
use jira::jql::field;

async fn print_issue_keys(jira: &CloudClient) -> jira::Result<()> {
    let mut issues = jira
        .issue_search()
        .search_issues()
        .jql(field("project").eq("PROJ").order_by_desc("created"))
        .fields(["summary"])
        .stream();

    while let Some(issue) = issues.try_next().await? {
        println!("{}", issue.key.unwrap_or_default());
    }

    Ok(())
}
```

Every paginated listing has `stream` too, including projects, users, filters, dashboards, boards, sprints and queues:

```rust,no_run
use jira::cloud::CloudClient;
use jira::futures_util::TryStreamExt;

async fn print_project_keys(jira: &CloudClient) -> jira::Result<()> {
    let mut projects = jira.projects().search_projects().stream();

    while let Some(project) = projects.try_next().await? {
        println!("{}", project.key.unwrap_or_default());
    }

    Ok(())
}
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

Every failure is a `jira::Error`. Its predicates read the HTTP status and the OAuth error code, so match on them:

```rust,no_run
use jira::Client;

async fn read_issue(client: &Client) {
    match client.get("/rest/api/3/issue/PROJ-1").send::<serde_json::Value>().await {
        Ok(issue) => println!("{issue}"),
        Err(error) if error.is_not_found() => println!("no such issue, or no permission to know"),
        Err(error) if error.is_rate_limit() => println!("wait {:?}", error.retry_after()),
        Err(error) if error.is_reauthorization_required() => println!("the grant is gone; authorize again"),
        Err(error) => eprintln!("{error}"),
    }
}
```

| Predicate | Means |
|---|---|
| `is_auth` | 401: credentials missing, expired or rejected |
| `is_scope` | 401 with a scope the token never asked for; refreshing cannot help |
| `is_forbidden` | 403: authenticated but not permitted |
| `is_not_found` | 404: absent, or not visible to you |
| `is_rate_limit` | 429: read `retry_after()` |
| `is_server` | 5xx |
| `is_network` | no HTTP answer at all |
| `is_oauth` | the token endpoint refused, or the cloud id would not resolve |
| `is_config` | the client cannot work as configured |
| `is_schema_mismatch` | a 2xx whose body is not what the type describes |

A credential refused through `X-Seraph-LoginReason` on a `200` response is reported as an auth error too.

## Retry

Retry is off by default. When enabled, it retries transient transport failures and 502, 503 and 504 responses. It
never retries a 4xx, a 429 or a 500:

```rust,no_run
use jira::{Client, RetryConfig};
use std::time::Duration;

fn retrying_client() -> jira::Result<Client> {
    Client::builder()
        .host("https://your-domain.atlassian.net")
        .retry(RetryConfig { max_attempts: 3, initial_delay: Duration::from_millis(500), backoff_factor: 2.0 })
        .build()
}
```

`jira::with_retry` applies the same policy around a call you already have.

## Cancellation, proxies and timeouts

To cancel a request, drop its future or wrap it in `tokio::time::timeout`. Proxies, timeouts and other transport
settings go through your own `reqwest::Client`:

```rust,no_run
use jira::Client;

fn proxied_client() -> Result<Client, Box<dyn std::error::Error>> {
    let http = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://proxy.internal:8080")?)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    Ok(Client::builder().host("https://your-domain.atlassian.net").http_client(http).build()?)
}
```

## Feature flags

One feature per API surface; a surface you do not enable is not compiled.

| Feature | Surface |
|---|---|
| `cloud` (default) | Jira Cloud platform: issues, projects, fields, workflows |
| `agile` | Jira Agile: boards, sprints, backlog |
| `service-desk` | Jira Service Management |
| `server` | Jira Data Center, platform and Agile in one surface |
| `service-desk-server` | Jira Service Management Data Center |
| `assets` / `assets-server` | Assets, on Cloud and Data Center |
| `admin` | Organization administration |
| `teams` | Teams |
| `user-management` / `user-provisioning` | User management and SCIM provisioning |
| `webhooks` | Event and payload types, and signature verification for deliveries |

Other features, all off by default:

| Feature | What it adds |
|---|---|
| `chrono` | Every `date-time` field becomes `Option<chrono::DateTime<Utc>>`; a value that does not parse becomes `None` |
| `tracing` | A `DEBUG` span per request with an event per attempt; no credentials or bodies are recorded |
| `audit` | Collects the response fields the generated types do not describe |

`chrono` changes field types, and cargo unifies features across a build, so enable it in applications, not libraries.

## Other products

- [jira.js](https://github.com/MrRefactoring/jira.js): the same APIs for Node.js and browsers
- [confluence.js](https://github.com/MrRefactoring/confluence.js)
- [trello.js](https://github.com/MrRefactoring/trello.js)

## License

MIT
