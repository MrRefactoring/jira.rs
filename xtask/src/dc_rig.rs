use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use regex::Regex;

const STARTUP_TIMEOUT: Duration = Duration::from_secs(10 * 60);
const POLL_INTERVAL: Duration = Duration::from_secs(5);

pub struct Rig {
    pub product: &'static str,
    pub compose_dir: PathBuf,
    pub base_url: String,
    pub admin_username: &'static str,
    pub admin_password: &'static str,
    pub admin_email: &'static str,
    pub title: &'static str,
    pub license_name: &'static str,
}

type Failure = Box<dyn std::error::Error>;

fn compose(rig: &Rig, arguments: &[&str]) -> Result<(), Failure> {
    let compose_file = rig.compose_dir.join("compose.yaml");
    let status = Command::new("docker").arg("compose").arg("-f").arg(&compose_file).args(arguments).status()?;

    if !status.success() {
        return Err(format!("docker compose {} failed", arguments.join(" ")).into());
    }

    Ok(())
}

async fn read_state(http: &reqwest::Client, rig: &Rig) -> String {
    let Ok(response) = http.get(format!("{}/status", rig.base_url)).send().await else {
        return "UNREACHABLE".to_owned();
    };

    if !response.status().is_success() {
        return format!("HTTP {}", response.status().as_u16());
    }

    response
        .json::<serde_json::Value>()
        .await
        .ok()
        .and_then(|body| body.get("state").and_then(|state| state.as_str()).map(ToOwned::to_owned))
        .unwrap_or_else(|| "UNKNOWN".to_owned())
}

async fn wait_for_state(http: &reqwest::Client, rig: &Rig, accepted: &[&str]) -> Result<String, Failure> {
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    let mut last = String::new();

    while Instant::now() < deadline {
        let state = read_state(http, rig).await;

        if state != last {
            println!("  state: {state}");
            last = state.clone();
        }

        if accepted.contains(&state.as_str()) {
            return Ok(state);
        }

        tokio::time::sleep(POLL_INTERVAL).await;
    }

    Err(format!(
        "{} did not reach {} within {}s (last: {last}).",
        rig.product,
        accepted.join(" or "),
        STARTUP_TIMEOUT.as_secs()
    )
    .into())
}

pub async fn is_set_up(http: &reqwest::Client, rig: &Rig) -> bool {
    read_state(http, rig).await == "RUNNING"
}

struct Form {
    action: String,
    fields: BTreeMap<String, String>,
}

fn read_form(html: &str, page_url: &str) -> Result<Form, Failure> {
    let form = Regex::new(r#"(?is)<form\b[^>]*\baction="([^"]+)"[^>]*>(.*?)</form>"#)?;
    let input = Regex::new(r"(?is)<input\b[^>]*>")?;
    let button = Regex::new(r"(?is)<button\b[^>]*>")?;
    let name = Regex::new(r#"(?i)\bname="([^"]+)""#)?;
    let value = Regex::new(r#"(?i)\bvalue="([^"]*)""#)?;
    let kind = Regex::new(r#"(?i)\btype="([^"]+)""#)?;
    let checked = Regex::new(r"(?i)\bchecked\b")?;
    let submit = Regex::new(r#"(?i)type="submit""#)?;

    let matched = form.captures(html).ok_or_else(|| -> Failure { format!("No form found on {page_url}.").into() })?;

    let body = matched.get(2).map_or("", |group| group.as_str());
    let mut fields = BTreeMap::new();

    for tag in input.find_iter(body).map(|found| found.as_str()) {
        let Some(field) = name.captures(tag).map(|captured| captured[1].to_owned()) else {
            continue;
        };

        let field_kind = kind.captures(tag).map_or_else(|| "text".to_owned(), |captured| captured[1].to_lowercase());

        if matches!(field_kind.as_str(), "radio" | "checkbox") && !checked.is_match(tag) {
            continue;
        }

        fields.insert(field, value.captures(tag).map_or_else(String::new, |captured| captured[1].to_owned()));
    }

    for tag in button.find_iter(body).map(|found| found.as_str()) {
        let Some(field) = name.captures(tag).map(|captured| captured[1].to_owned()) else {
            continue;
        };

        if !submit.is_match(tag) {
            continue;
        }

        fields.insert(field, value.captures(tag).map_or_else(String::new, |captured| captured[1].to_owned()));
    }

    let action = url::Url::parse(page_url)?.join(&matched[1])?.to_string();

    Ok(Form { action, fields })
}

fn answers(rig: &Rig, license: &str) -> Vec<(&'static str, Vec<(&'static str, String)>)> {
    vec![
        (
            "SetupApplicationProperties",
            vec![("title", rig.title.to_owned()), ("mode", "private".to_owned()), ("baseURL", rig.base_url.clone())],
        ),
        ("SetupLicense", vec![("setupLicenseKey", license.to_owned())]),
        (
            "SetupAdminAccount",
            vec![
                ("username", rig.admin_username.to_owned()),
                ("fullname", "jira live suite".to_owned()),
                ("email", rig.admin_email.to_owned()),
                ("password", rig.admin_password.to_owned()),
                ("confirm", rig.admin_password.to_owned()),
            ],
        ),
        ("SetupMailNotifications", vec![("noemail", "true".to_owned())]),
    ]
}

struct Page {
    url: String,
    html: String,
}

async fn get(http: &reqwest::Client, target: &str) -> Result<Page, Failure> {
    let response = http.get(target).send().await?;
    let url = response.url().to_string();

    Ok(Page { url, html: response.text().await? })
}

async fn post_form(http: &reqwest::Client, target: &str, fields: &BTreeMap<String, String>) -> Result<Page, Failure> {
    let response = http.post(target).form(fields).send().await?;
    let url = response.url().to_string();

    Ok(Page { url, html: response.text().await? })
}

async fn wait_for_first_step(http: &reqwest::Client, rig: &Rig, steps: &[&str]) -> Result<Page, Failure> {
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    let mut last = String::new();

    while Instant::now() < deadline {
        let page = get(http, &format!("{}/", rig.base_url)).await?;

        if steps.iter().any(|step| page.url.contains(step)) {
            return Ok(page);
        }

        if page.url != last {
            println!("  not asking yet: {}", page.url.replace(&rig.base_url, ""));
            last = page.url.clone();
        }

        tokio::time::sleep(POLL_INTERVAL).await;
    }

    Err(format!("The wizard never served a step within {}s (last: {last}).", STARTUP_TIMEOUT.as_secs()).into())
}

fn read_license(rig: &Rig) -> Result<String, Failure> {
    let path = rig.compose_dir.join("timebomb-license.txt");

    match std::fs::read_to_string(&path) {
        Ok(license) => Ok(license.trim().to_owned()),
        Err(_) => Err(format!(
            "No licence at {}. Atlassian publishes three-hour timebomb keys for exactly this at \
https://developer.atlassian.com/platform/marketplace/timebomb-licenses-for-testing-server-apps/ — take the {} one and \
write it to that path. In CI it comes from the repository secret instead. What the choice costs this rig is written \
down in {}/README.md.",
            path.display(),
            rig.license_name,
            rig.compose_dir.display()
        )
        .into()),
    }
}

async fn run_wizard(http: &reqwest::Client, rig: &Rig) -> Result<(), Failure> {
    let license = read_license(rig)?;
    let answers = answers(rig, &license);
    let steps: Vec<&str> = answers.iter().map(|(step, _)| *step).collect();

    let mut page = wait_for_first_step(http, rig, &steps).await?;

    let mut repeated = 0;
    let mut previous = String::new();

    for _ in 0..12 {
        if is_set_up(http, rig).await {
            return Ok(());
        }

        let form = read_form(&page.html, &page.url)?;
        let step = form.action.replace(&rig.base_url, "");

        if step == previous {
            repeated += 1;

            if repeated == 2 {
                return Err(format!(
                    "The wizard keeps serving {step} rather than moving on, so it has refused what was sent. Open \
                     {}{step} in a browser and submit it by hand; that is where the reason is shown. An expired \
                     timebomb in {} reads exactly like this.",
                    rig.base_url,
                    rig.compose_dir.join("timebomb-license.txt").display()
                )
                .into());
            }
        } else {
            repeated = 0;
            previous = step.clone();
        }

        println!("  step: {step}");

        let mut fields = form.fields.clone();

        if let Some((_, values)) = answers.iter().find(|(step, _)| form.action.contains(step)) {
            for (field, value) in values {
                fields.insert((*field).to_owned(), value.clone());
            }
        }

        page = post_form(http, &form.action, &fields).await?;
    }

    Err("The setup wizard did not finish within twelve steps. Open the instance and look at what it asks.".into())
}

pub async fn run(rig: &Rig, command: &str) -> Result<(), Failure> {
    let http = reqwest::Client::builder().cookie_store(true).build()?;

    match command {
        "down" => {
            compose(rig, &["down", "--volumes"])?;
            println!("✔ stopped, volumes removed");
        }
        "status" => {
            println!("{} → {}", rig.base_url, read_state(&http, rig).await);
        }
        "up" => {
            println!("▸ starting containers");
            compose(rig, &["up", "-d"])?;

            println!("▸ waiting for {} (a cold start takes minutes)", rig.product);

            if wait_for_state(&http, rig, &["FIRST_RUN", "RUNNING"]).await? == "FIRST_RUN" {
                println!("▸ running the setup wizard");
                run_wizard(&http, rig).await?;
                wait_for_state(&http, rig, &["RUNNING"]).await?;
            } else {
                println!("▸ already set up");
            }

            println!("✔ ready at {} — sign in as {}", rig.base_url, rig.admin_username);
        }
        other => return Err(format!("Unknown command \"{other}\". Use up, status or down.").into()),
    }

    Ok(())
}
