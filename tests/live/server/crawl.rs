//! Calls every readable Data Center endpoint the crate generates and reports which ones nothing can reach.
//!
//! This is the breadth instrument for the `server` surface, and it exists because the Data Center document is
//! generated from Java annotations rather than written: it is far less accurate than the Cloud one, and the
//! inaccuracies are spread thin across four hundred operations rather than concentrated where a hand-written test
//! would look. Two hundred GETs answered by a real instance sweep further in one run than a month of hand-written
//! assertions.
//!
//! What counts as a failure is only a request that never reached Jira. An endpoint answering 404 or 403 is Jira
//! telling the truth about this instance — there is no cluster on a single node, no index snapshot on a fresh install
//! — and treating that as breakage would drown the signal the crawl exists to produce.
//!
//! The set of endpoints is read out of the generated sources rather than kept in a list here. Each operation declares
//! its method and its URL as literals inside `RequestConfig::new`, so the crate's own `src/server/api` is the
//! register, and the crawl cannot drift out of step with what the client actually ships.
//!
//! # What the TypeScript twin does that this cannot
//!
//! `crawl.test.ts` calls each *generated function*, reached by name off the API namespace at run time, and its real
//! subject is the schema check that call performs: every response is deserialized against the model the generator
//! wrote for it, and the run's verdict is the list of endpoints whose body did not match. Rust has no equivalent
//! handle. There is no reflection over functions and no way to dispatch to a generated deserializer from a string, so
//! a call made generically can only be a raw one — `send_raw`, which hands back the body unmodelled and therefore
//! skips the very check the crawl is for. Reproducing that half would mean writing out two hundred typed calls by
//! hand, which is precisely the list the TypeScript crawl refuses to keep.
//!
//! So what is here is the reachability half: the register is read the same way, the same table of where each path
//! parameter's value comes from feeds the same multi-pass sweep, and every URL is called. Schema drift stays the
//! business of the hand-written suites, where the typed call is what makes the check happen.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use jira::server::{CommentJson, Filter, IssueUpdate, ProjectInput, Version, WorkflowScheme};
use serde_json::{Value, json};

use super::fixtures::software_licensed;
use crate::harness::{ResourceTracker, run_id, server, server_client, test_name};

/// Where the generated operations live, as the crate compiles them.
const GENERATED_SOURCES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/server/api");

/// The scope of a value that is good on any URL.
const ANYWHERE: &str = "*";

/// Names the crawl has to be able to spell without being told what this run happened to create.
const PROPERTY_KEY: &str = "jirars";
const ATTRIBUTE_KEY: &str = "jirars.probe";

/// One generated operation, as its source declares it.
struct Endpoint {
    /// The URL with `{}` where the operation interpolates a path parameter.
    url: String,
    /// The path parameters it interpolates, in the order they appear.
    parameters: Vec<String>,
}

/// What reading the generated sources found.
struct Generated {
    /// Every `RequestConfig` the sources declare, whatever its method.
    declared: usize,
    /// The files holding a declaration whose method or URL could not be read as a literal.
    unreadable: BTreeSet<String>,
    /// The `GET` operations, which are the ones a crawl may call.
    readable: Vec<Endpoint>,
}

fn read_generated() -> Generated {
    let mut generated = Generated { declared: 0, unreadable: BTreeSet::new(), readable: Vec::new() };
    let entries =
        fs::read_dir(GENERATED_SOURCES).expect("the generated Data Center sources are where the crate keeps them");

    for entry in entries {
        let path = entry.expect("a directory entry reads").path();

        if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
            continue;
        }

        let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("?").to_owned();
        let source = fs::read_to_string(&path).expect("a generated source file reads");
        let mut rest = source.as_str();

        while let Some(at) = rest.find("RequestConfig::new(") {
            rest = &rest[at + "RequestConfig::new(".len()..];
            generated.declared += 1;

            match read_declaration(rest) {
                Some((method, url, parameters)) if method == "GET" => {
                    generated.readable.push(Endpoint { url, parameters });
                }
                Some(_) => {}
                None => {
                    generated.unreadable.insert(name.clone());
                }
            }
        }
    }

    generated
}

/// The method, the URL and the path parameters one declaration spells out, read from just after its opening bracket.
fn read_declaration(source: &str) -> Option<(String, String, Vec<String>)> {
    let rest = source.trim_start().strip_prefix("crate::core::Method::")?;
    let method: String =
        rest.chars().take_while(|character| character.is_ascii_alphanumeric() || *character == '_').collect();
    let rest = rest[method.len()..].trim_start().strip_prefix(',')?.trim_start();

    // A URL with no path parameters is a plain literal; one with them is a `format!` over the same literal.
    if let Some(literal) = rest.strip_prefix('"') {
        let end = literal.find('"')?;

        return Some((method, literal[..end].to_owned(), Vec::new()));
    }

    let literal = rest.strip_prefix("format!(")?.trim_start().strip_prefix('"')?;
    let end = literal.find('"')?;

    Some((method, literal[..end].to_owned(), read_interpolations(&literal[end + 1..])))
}

/// The `self.…` fields a `format!` interpolates, in the order they appear.
fn read_interpolations(source: &str) -> Vec<String> {
    let bytes = source.as_bytes();
    let mut parameters = Vec::new();
    let mut depth = 1_usize;
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;

                if depth == 0 {
                    break;
                }
            }
            _ => {
                if let Some(field) = source[index..].strip_prefix("self.") {
                    // `r#type` is one of them: a parameter whose name collides with a keyword.
                    let field = field.strip_prefix("r#").unwrap_or(field);
                    let name: String = field
                        .chars()
                        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
                        .collect();

                    if !name.is_empty() {
                        parameters.push(name);
                    }
                }
            }
        }

        index += 1;
    }

    parameters
}

/// Where each path parameter's value comes from, endpoint by endpoint: the listing URL, the parameter it feeds, the
/// field to read it out of, and the URL prefix the value is good for.
///
/// A parameter name alone does not identify what it wants. Seventeen different resources spell their identifier `id`,
/// and `scheme_id` means a permission scheme under one path and an issue type scheme under another — feeding one
/// value to both reaches neither, it only earns two 404s that read like the endpoints are unsupported. The longest
/// matching scope wins, and [`ANYWHERE`] is the fallback.
const SOURCES: &[(&str, &str, &str, &str)] = &[
    ("/rest/agile/1.0/board", "board_id", "id", ANYWHERE),
    ("/rest/agile/1.0/board/{}/sprint", "sprint_id", "id", ANYWHERE),
    ("/rest/agile/1.0/board/{}/epic", "epic_id_or_key", "key", ANYWHERE),
    ("/rest/agile/1.0/board/{}/epic", "epic_id", "id", ANYWHERE),
    ("/rest/api/2/issue/{}/comment", "comment_id", "id", ANYWHERE),
    ("/rest/api/2/issue/{}/remotelink", "link_id", "id", "/rest/api/2/issue"),
    ("/rest/api/2/customFields", "custom_field_id", "id", ANYWHERE),
    ("/rest/api/2/dashboard", "dashboard_id", "id", ANYWHERE),
    ("/rest/api/2/dashboard", "id", "id", "/rest/api/2/dashboard"),
    ("/rest/api/2/issuetype", "issue_type_id", "id", ANYWHERE),
    ("/rest/api/2/issuetype", "issue_type", "id", ANYWHERE),
    ("/rest/api/2/issuetype", "id", "id", "/rest/api/2/issuetype"),
    ("/rest/api/2/issueLinkType", "issue_link_type_id", "id", ANYWHERE),
    ("/rest/api/2/permissionscheme", "permission_scheme_id", "id", ANYWHERE),
    ("/rest/api/2/permissionscheme", "scheme_id", "id", "/rest/api/2/permissionscheme"),
    ("/rest/api/2/permissionscheme/{}/permission", "permission_id", "id", "/rest/api/2/permissionscheme"),
    ("/rest/api/2/issuetypescheme", "scheme_id", "id", "/rest/api/2/issuetypescheme"),
    ("/rest/api/2/priorityschemes", "scheme_id", "id", "/rest/api/2/priorityschemes"),
    ("/rest/api/2/filter/{}/permission", "permission_id", "id", "/rest/api/2/filter"),
    ("/rest/api/2/screens", "screen_id", "id", ANYWHERE),
    ("/rest/api/2/screens/{}/tabs", "tab_id", "id", ANYWHERE),
    ("/rest/api/2/terminology/entries", "original_name", "originalName", ANYWHERE),
    ("/rest/api/2/statuscategory", "id_or_key", "id", ANYWHERE),
    ("/rest/api/2/status", "id_or_name", "id", ANYWHERE),
    ("/rest/api/2/component/page", "id", "id", "/rest/api/2/component"),
    ("/rest/api/2/issuesecurityschemes", "id", "id", "/rest/api/2/issuesecurityschemes"),
    ("/rest/api/2/notificationscheme", "id", "id", "/rest/api/2/notificationscheme"),
    ("/rest/api/2/priority", "id", "id", "/rest/api/2/priority"),
    ("/rest/api/2/projectCategory", "id", "id", "/rest/api/2/projectCategory"),
    ("/rest/api/2/resolution", "id", "id", "/rest/api/2/resolution"),
    ("/rest/api/2/role", "id", "id", "/rest/api/2/role"),
    ("/rest/api/2/applicationrole", "key", "key", "/rest/api/2/applicationrole"),
    ("/rest/jira-webhook/1.0/webhooks", "webhook_id", "id", ANYWHERE),
];

/// Values by parameter name and by the URL prefix they are good for.
#[derive(Default)]
struct Values {
    entries: Vec<(String, String, String)>,
}

impl Values {
    /// Keeps the first value offered for a parameter in a scope, so a seed always beats what a listing later hands up.
    fn remember(&mut self, parameter: &str, value: impl Into<String>, scope: &str) {
        let value = value.into();

        if value.is_empty() {
            return;
        }

        if self.entries.iter().any(|(known_scope, known, _)| known == parameter && known_scope == scope) {
            return;
        }

        self.entries.push((scope.to_owned(), parameter.to_owned(), value));
    }

    /// The most specific value for this parameter on this URL: a matching scope beats a shorter one beats the global.
    fn resolve(&self, url: &str, parameter: &str) -> Option<&str> {
        let mut best: Option<&(String, String, String)> = None;

        for entry in &self.entries {
            if entry.1 != parameter || (entry.0 != ANYWHERE && !url.starts_with(entry.0.as_str())) {
                continue;
            }

            let better = match best {
                None => true,
                Some(current) => entry.0 != ANYWHERE && (current.0 == ANYWHERE || entry.0.len() > current.0.len()),
            };

            if better {
                best = Some(entry);
            }
        }

        best.map(|entry| entry.2.as_str())
    }
}

/// The first list in a response, whatever the endpoint calls it.
///
/// Data Center wraps its collections under a dozen different names — `values`, `issues`, `comments`, `links`,
/// `permissionSchemes` — and naming each one here would be a table that has to be kept in step with the API for no
/// gain. What every one of them has in common is being the only array in the body.
fn first_list(body: &Value) -> Option<&Vec<Value>> {
    if let Some(items) = body.as_array() {
        return Some(items);
    }

    body.as_object()?.values().find_map(|value| value.as_array().filter(|items| !items.is_empty()))
}

fn harvest(url: &str, body: &Value, values: &mut Values) {
    let Some(first) = first_list(body).and_then(|items| items.first()) else {
        return;
    };

    for (from, parameter, field, scope) in SOURCES {
        if *from != url {
            continue;
        }

        match first.get(field) {
            Some(Value::String(text)) => values.remember(parameter, text.clone(), scope),
            Some(Value::Number(number)) => values.remember(parameter, number.to_string(), scope),
            _ => {}
        }
    }
}

enum Outcome {
    Answered,
    Refused,
    Failed(String),
}

/// A project key of this file's own making.
///
/// `project_key` derives its key from the run id alone, so every call to it in a run answers with the same key — and
/// the other Data Center suites use it for the project they create. `X` is not a hexadecimal digit and the run id is
/// nothing else, so this key cannot collide with theirs however the run comes out.
fn crawl_project_key() -> String {
    format!("JRSX{}", run_id().to_uppercase()).chars().take(10).collect()
}

/// The world the crawl points at.
///
/// A bare instance has one administrator and nothing else, and the parameterised half of the surface has nothing to
/// name until something exists. What is created here is the cheapest set that opens the most paths: a project, an
/// issue inside it with a comment and a property, a version, a filter and a workflow scheme.
struct Fixture {
    project_key: String,
    project_id: String,
    issue_key: String,
    version_id: String,
    filter_id: String,
    workflow_scheme_id: i64,
}

async fn create_fixture(tracker: &mut ResourceTracker) -> Fixture {
    let (project_type, project_template) = if software_licensed().await {
        ("software", "com.pyxis.greenhopper.jira:gh-scrum-template")
    } else {
        ("business", "com.atlassian.jira-core-project-templates:jira-core-project-management")
    };

    let me = server().myself().get_current_user().send().await.expect("the instance knows the caller");
    let lead = me.name.expect("a Data Center user is addressed by name, not by id");
    let project_key = crawl_project_key();

    let project = server()
        .projects()
        .create_project(ProjectInput {
            key: Some(project_key.clone()),
            name: Some(test_name("crawl")),
            lead: Some(lead),
            project_type_key: Some(project_type.to_owned()),
            project_template_key: Some(project_template.to_owned()),
            ..ProjectInput::default()
        })
        .send()
        .await
        .expect("the instance accepts a new project");

    let project_id = project.id.expect("a created project carries an id").to_string();
    let for_cleanup = project_key.clone();

    tracker.defer(move || {
        let key = for_cleanup.clone();

        async move { server().projects().delete_project(key).send().await }
    });

    let issue = server()
        .issues()
        .create_issue()
        .issue_update(IssueUpdate {
            fields: Some(
                [
                    ("project".to_owned(), json!({ "key": project_key })),
                    ("issuetype".to_owned(), json!({ "name": "Task" })),
                    ("summary".to_owned(), json!(test_name("crawl issue"))),
                    // Rich text on a self-hosted instance is wiki markup as a plain string, not ADF.
                    ("description".to_owned(), json!("h2. Fixture\n\nCreated by the crawl.")),
                ]
                .into_iter()
                .collect(),
            ),
            ..IssueUpdate::default()
        })
        .send()
        .await
        .expect("the project accepts a new issue");

    let issue_key = issue.key.expect("a created issue carries a key");
    let for_cleanup = issue_key.clone();

    tracker.defer(move || {
        let key = for_cleanup.clone();

        async move { server().issues().delete_issue(key).send().await }
    });

    // The comment and the property are not registered separately: both belong to the issue and go when it does.
    server()
        .issues()
        .add_comment(&issue_key)
        .comment_json(CommentJson { body: Some("created by the crawl".to_owned()), ..CommentJson::default() })
        .send()
        .await
        .expect("the issue takes a comment");

    server()
        .issues()
        .set_issue_property(
            PROPERTY_KEY,
            &issue_key,
            [("createdBy".to_owned(), json!("the Data Center crawl"))].into_iter().collect(),
        )
        .send()
        .await
        .expect("the issue takes a property");

    // A version belongs to the project and goes with it, and the Data Center API offers no plain delete for one
    // anyway — only a delete that swaps every issue's fix version over to another.
    let version = server()
        .project_versions()
        .create_version(Version {
            name: Some(test_name("version")),
            project: Some(project_key.clone()),
            description: Some("created by the crawl".to_owned()),
            ..Version::default()
        })
        .send()
        .await
        .expect("the project accepts a new version");

    let version_id = version.id.expect("a created version carries an id");

    let filter = server()
        .filters()
        .create_filter()
        .filter(Filter {
            name: Some(test_name("filter")),
            jql: Some(format!("project = {project_key}")),
            description: Some("created by the crawl".to_owned()),
            ..Filter::default()
        })
        .send()
        .await
        .expect("the instance accepts a new filter");

    let filter_id = filter.id.expect("a created filter carries an id");
    let for_cleanup = filter_id.clone();

    tracker.defer(move || {
        let id = for_cleanup.clone();

        async move { server().filters().delete_filter(id).send().await }
    });

    // A fresh instance has no workflow scheme that is not the default, and eight read endpoints — the drafts among
    // them — take a scheme id.
    let scheme = server()
        .workflow_schemes()
        .create_scheme(WorkflowScheme {
            name: Some(test_name("crawl workflow scheme")),
            description: Some("created by the crawl".to_owned()),
            ..WorkflowScheme::default()
        })
        .send()
        .await
        .expect("the instance accepts a new workflow scheme");

    let workflow_scheme_id = scheme.id.expect("a created workflow scheme carries an id");

    tracker.defer(move || async move { server().workflow_schemes().delete_scheme(workflow_scheme_id).send().await });

    Fixture { project_key, project_id, issue_key, version_id, filter_id, workflow_scheme_id }
}

/// The seeds nothing on the instance can be asked for, plus the ones the fixture just made.
fn seed(fixture: &Fixture) -> Values {
    let mut values = Values::default();

    for name in ["project_id_or_key", "project_key_or_id", "project_key", "proj_id_or_key"] {
        values.remember(name, fixture.project_key.clone(), ANYWHERE);
    }

    for name in ["issue_id_or_key", "issue_key", "issue_id"] {
        values.remember(name, fixture.issue_key.clone(), ANYWHERE);
    }

    values.remember("id", fixture.issue_key.clone(), "/rest/api/2/issue");
    values.remember("id", fixture.project_id.clone(), "/rest/api/2/project");
    values.remember("id", fixture.version_id.clone(), "/rest/api/2/version");
    values.remember("id", fixture.filter_id.clone(), "/rest/api/2/filter");
    values.remember("id", fixture.workflow_scheme_id.to_string(), "/rest/api/2/workflowscheme");
    values.remember("version_id", fixture.version_id.clone(), ANYWHERE);
    values.remember("owning_object_id", fixture.project_id.clone(), ANYWHERE);

    // Chosen rather than discovered: the crawl has to be able to spell these without being told what this particular
    // run created.
    values.remember("property_key", PROPERTY_KEY, ANYWHERE);
    values.remember("attribute_key", ATTRIBUTE_KEY, ANYWHERE);
    values.remember("global_id", "jirars-remote-version-link", "/rest/api/2/version");
    values.remember("type", "project", ANYWHERE);
    values.remember("project_type_key", "software", ANYWHERE);

    values
}

/// One URL with every `{}` filled in, or nothing if some parameter has no value yet.
fn fill(endpoint: &Endpoint, values: &Values) -> Option<String> {
    let mut parts = endpoint.url.split("{}");
    let mut filled = parts.next()?.to_owned();

    for (part, parameter) in parts.zip(&endpoint.parameters) {
        filled.push_str(values.resolve(&endpoint.url, parameter)?);
        filled.push_str(part);
    }

    Some(filled)
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_every_endpoint_whose_path_parameters_it_can_supply() {
    let generated = read_generated();

    // Every generated operation writes both as literals. A miss means the generator started building its URLs some
    // other way, and a crawl that silently skipped those endpoints would look like a clean run.
    assert!(
        generated.unreadable.is_empty(),
        "every generated operation declares its method and its url as literals; these files hold one that does not: \
{:?}",
        generated.unreadable,
    );
    assert!(generated.declared > 400, "the surface ships hundreds of operations, found {}", generated.declared);
    assert!(
        generated.readable.len() > 200,
        "the surface ships more than two hundred readable operations, found {}",
        generated.readable.len(),
    );

    let mut tracker = ResourceTracker::new();
    let fixture = create_fixture(&mut tracker).await;
    let mut values = seed(&fixture);
    let mut outcomes: BTreeMap<&str, Outcome> = BTreeMap::new();

    // Passes rather than one sweep, because reaching an endpoint can be what supplies the next one's parameter: a
    // sprint id is listed only by a board endpoint, and the board id only by the endpoint that lists boards. Looping
    // until a pass adds nothing turns that chain into coverage without hard-coding an order.
    for _ in 0..6 {
        let reachable: Vec<&Endpoint> = generated
            .readable
            .iter()
            .filter(|endpoint| !outcomes.contains_key(endpoint.url.as_str()))
            .filter(|endpoint| {
                endpoint.parameters.iter().all(|parameter| values.resolve(&endpoint.url, parameter).is_some())
            })
            .collect();

        if reachable.is_empty() {
            break;
        }

        for endpoint in reachable {
            let Some(url) = fill(endpoint, &values) else {
                continue;
            };

            // Raw on purpose, and the one thing this crawl cannot do: a call made from a URL rather than from a
            // generated operation has no model to deserialize into, so nothing here checks a response against its
            // schema. See the note at the top of the file.
            let outcome = match server_client().get(&url).send_raw().await {
                Ok(body) => {
                    harvest(&endpoint.url, &body, &mut values);

                    Outcome::Answered
                }
                // Jira answering "no" is an answer. Only a request that never reached it is this crawl's business.
                Err(error) if error.status().is_some() => Outcome::Refused,
                Err(error) => Outcome::Failed(error.to_string()),
            };

            outcomes.insert(endpoint.url.as_str(), outcome);
        }
    }

    let answered = outcomes.values().filter(|outcome| matches!(outcome, Outcome::Answered)).count();
    let refused = outcomes.values().filter(|outcome| matches!(outcome, Outcome::Refused)).count();
    let failed: Vec<String> = outcomes
        .iter()
        .filter_map(|(url, outcome)| match outcome {
            Outcome::Failed(detail) => Some(format!("{url}: {detail}")),
            _ => None,
        })
        .collect();
    let unreached: Vec<&Endpoint> =
        generated.readable.iter().filter(|endpoint| !outcomes.contains_key(endpoint.url.as_str())).collect();

    println!(
        "  {}/{} called — {answered} answered, {refused} refused, {} failed; {} never reachable",
        outcomes.len(),
        generated.readable.len(),
        failed.len(),
        unreached.len(),
    );

    if !unreached.is_empty() {
        let missing: BTreeSet<&str> = unreached
            .iter()
            .flat_map(|endpoint| {
                endpoint
                    .parameters
                    .iter()
                    .filter(|parameter| values.resolve(&endpoint.url, parameter).is_none())
                    .map(String::as_str)
            })
            .collect();

        println!("  nothing supplies: {}", missing.into_iter().collect::<Vec<_>>().join(", "));
        println!(
            "  out of reach: {}",
            unreached.iter().map(|endpoint| endpoint.url.as_str()).collect::<Vec<_>>().join(", "),
        );
    }

    tracker.cleanup().await;

    assert!(failed.is_empty(), "{} endpoints never reached Jira:\n{}", failed.len(), failed.join("\n"));
    assert!(
        answered >= 80,
        "a bare instance answers most of what has no path parameter at all; only {answered} of {} calls answered",
        outcomes.len(),
    );
}
