//! Reads the account the credentials belong to.
//!
//! ```sh
//! JIRA_BASE_URL=https://your-domain.atlassian.net \
//! JIRA_EMAIL=you@example.com \
//! JIRA_API_TOKEN=... \
//!   cargo run --example basic
//! ```

use jira::{Auth, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::var("JIRA_BASE_URL")?;
    let email = std::env::var("JIRA_EMAIL")?;
    let api_token = std::env::var("JIRA_API_TOKEN")?;

    let client = Client::builder().host(host).auth(Auth::api_token(email, api_token)).build()?;

    let myself: serde_json::Value = client.get("/rest/api/3/myself").send().await?;

    println!("{} <{}>", myself["displayName"], myself["emailAddress"]);

    let projects: serde_json::Value =
        client.get("/rest/api/3/project/search").query("maxResults", 5).query("orderBy", "name").send().await?;

    for project in projects["values"].as_array().into_iter().flatten() {
        println!("{} — {}", project["key"], project["name"]);
    }

    Ok(())
}
