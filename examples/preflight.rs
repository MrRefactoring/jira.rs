//! What the live suite will and will not be able to exercise on this site.
//!
//! Several suites stand down where the site does not offer what they need — a scrum board, site administration, a
//! Service Management licence, a paid plan. Standing down is the right behaviour, but a run of green tests says
//! nothing about how many of them stood down. This reports that before the run rather than after it.
//!
//! ```sh
//! cargo run --example preflight --all-features
//! ```

use jira::agile::AgileClient;
use jira::cloud::CloudClient;
use jira::service_desk::ServiceDeskClient;
use jira::{Auth, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));

    let host = std::env::var("JIRA_BASE_URL").or_else(|_| std::env::var("HOST"))?;
    let email = std::env::var("JIRA_EMAIL").or_else(|_| std::env::var("EMAIL"))?;
    let token = std::env::var("JIRA_API_TOKEN").or_else(|_| std::env::var("API_TOKEN"))?;

    let client = Client::builder().host(&host).auth(Auth::api_token(email, token)).build()?;
    let cloud = CloudClient::new(client.clone());
    let agile = AgileClient::new(client.clone());
    let desk = ServiceDeskClient::new(client);

    println!("site: {host}\n");

    let myself = cloud.myself().get_current_user().send().await?;

    println!("signed in as {}", myself.display_name.unwrap_or_default());

    report(
        "the test project",
        cloud.projects().get_project("AUTOTEST").send().await.map(|project| {
            format!(
                "{} ({})",
                project.name.unwrap_or_default(),
                project.project_type_key.map_or_else(String::new, |kind| kind.as_str().to_owned())
            )
        }),
    );

    report(
        "site administration",
        cloud.screens().get_screens().max_results(1).send().await.map(|page| format!("{} screens", page.total)),
    );

    report(
        "agile boards",
        agile.board().get_all_boards().max_results(50).send().await.map(|page| {
            let scrum = page
                .values
                .iter()
                .filter(|board| board.r#type.as_ref().is_some_and(|kind| kind.as_str() == "scrum"))
                .count();

            format!("{} boards, {scrum} of them scrum", page.values.len())
        }),
    );

    report("service management", desk.info().get_info().send().await.map(|_| "reachable".to_owned()));

    report(
        "service desks",
        desk.servicedesk()
            .get_service_desks()
            .limit(1)
            .send()
            .await
            .map(|page| format!("{} listed", page.values.len())),
    );

    println!("\nA suite whose prerequisite is missing stands down rather than failing, so those lines are the ones");
    println!("to read before trusting a green run.");

    Ok(())
}

fn report<T: std::fmt::Display>(what: &str, outcome: Result<T, jira::Error>) {
    match outcome {
        Ok(value) => println!("  ✔ {what}: {value}"),
        Err(error) => {
            let why = error.status().map_or_else(|| "no answer".to_owned(), |status| format!("HTTP {status}"));

            println!("  ✖ {what}: {why} — the suites that need it will stand down");
        }
    }
}
