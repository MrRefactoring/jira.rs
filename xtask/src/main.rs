//! Repository chores that need more than a shell line.
//!
//! ```sh
//! cargo xtask jira-dc up      # start the Data Center instance the `server` suites need
//! cargo xtask jira-dc status
//! cargo xtask jira-dc down
//! cargo xtask jsm-dc up       # the same for the Service Management suites
//! cargo xtask audit           # run the live suite and report what the types do not describe
//! ```

mod audit;
mod dc_rig;

use std::path::PathBuf;

use dc_rig::Rig;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask sits inside the workspace")
        .to_owned()
}

fn jira_dc() -> Rig {
    Rig {
        product: "Jira Data Center",
        compose_dir: workspace_root().join("docker/jira-dc"),
        base_url: std::env::var("JIRA_SERVER_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_owned()),
        admin_username: "admin",
        admin_password: "admin123",
        admin_email: "admin@example.invalid",
        title: "jira-rs live suite",
    }
}

fn jsm_dc() -> Rig {
    Rig {
        product: "Jira Service Management Data Center",
        compose_dir: workspace_root().join("docker/jsm-dc"),
        base_url: std::env::var("JSM_SERVER_BASE_URL").unwrap_or_else(|_| "http://localhost:8081".to_owned()),
        admin_username: "admin",
        admin_password: "admin123",
        admin_email: "admin@example.invalid",
        title: "jira-rs live suite (JSM)",
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let rig = arguments.next().unwrap_or_default();
    let command = arguments.next().unwrap_or_else(|| "status".to_owned());

    match rig.as_str() {
        "jira-dc" => dc_rig::run(&jira_dc(), &command).await,
        "jsm-dc" => dc_rig::run(&jsm_dc(), &command).await,
        "audit" => audit::run(&workspace_root(), &std::env::args().skip(2).collect::<Vec<_>>()).await,
        other => Err(format!(
            "Unknown task \"{other}\". Use `cargo xtask jira-dc <up|status|down>` or `cargo xtask jsm-dc <…>`."
        )
        .into()),
    }
}
