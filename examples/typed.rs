//! The typed API: a client, a surface, and operations that answer with models.
//!
//! ```sh
//! JIRA_BASE_URL=https://your-domain.atlassian.net \
//! JIRA_EMAIL=you@example.com \
//! JIRA_API_TOKEN=... \
//!   cargo run --example typed --features cloud,agile
//! ```

use jira::agile::AgileClient;
use jira::cloud::CloudClient;
use jira::{Auth, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .host(std::env::var("JIRA_BASE_URL")?)
        .auth(Auth::api_token(std::env::var("JIRA_EMAIL")?, std::env::var("JIRA_API_TOKEN")?))
        .build()?;

    // One transport, every surface: two clients would mean two OAuth token states.
    let jira = CloudClient::new(client.clone());
    let agile = AgileClient::new(client);

    let myself = jira.myself().get_current_user().send().await?;

    println!("signed in as {}", myself.display_name.unwrap_or_default());

    let projects = jira.projects().search_projects().max_results(5).order_by("name").send().await?;

    for project in projects.values {
        println!("{} — {}", project.key.unwrap_or_default(), project.name.unwrap_or_default());
    }

    let boards = agile.board().get_all_boards().max_results(5).send().await?;

    for board in boards.values {
        println!("board {} ({})", board.name.unwrap_or_default(), board.id.unwrap_or_default());
    }

    Ok(())
}
