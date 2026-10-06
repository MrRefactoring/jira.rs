use regex::Regex;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::LazyLock;

#[cfg(test)]
use crate::generated::parse_service;
use crate::generated::{Operation, Signature, read_surface};

type Failure = Box<dyn std::error::Error>;

#[derive(Debug, serde::Deserialize)]
struct ManifestOperation {
    surface: String,
    method: String,
    path: String,
}

fn manifest_endpoint(operation: &ManifestOperation) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{[^}]+\}").unwrap());
    format!("{} {}", operation.method, RE.replace_all(&operation.path, "{…}"))
}

fn verify_manifest(operations: &[Operation], manifest: &[ManifestOperation], surfaces: &[&str]) -> Result<(), Failure> {
    let expected: BTreeMap<String, usize> = manifest
        .iter()
        .filter(|entry| surfaces.contains(&entry.surface.as_str()))
        .fold(BTreeMap::new(), |mut map, entry| {
            *map.entry(manifest_endpoint(entry)).or_default() += 1;
            map
        });
    let actual: BTreeMap<String, usize> = operations.iter().fold(BTreeMap::new(), |mut map, entry| {
        *map.entry(entry.endpoint.clone()).or_default() += 1;
        map
    });
    if expected != actual {
        let missing: Vec<_> = expected.iter().filter(|(key, count)| actual.get(*key) != Some(count)).collect();
        let extra: Vec<_> = actual.iter().filter(|(key, count)| expected.get(*key) != Some(count)).collect();
        return Err(format!("operations.json and generated {surfaces:?} differ; missing/mismatched: {missing:?}; extra/mismatched: {extra:?}").into());
    }
    Ok(())
}

fn values_named_on_one_side(cloud: &str, server: &str) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"impl Into<([^<>]+)>").unwrap());
    let cloud_values: Vec<_> = RE.captures_iter(cloud).collect();
    let server_values: Vec<_> = RE.captures_iter(server).collect();
    if cloud_values.len() != 1 || server_values.len() != 1 {
        return false;
    }
    let cloud_value = &cloud_values[0][1];
    let server_value = &server_values[0][1];
    if (cloud_value == "String") == (server_value == "String") {
        return false;
    }
    RE.replace(cloud, "impl Into<VALUE>") == RE.replace(server, "impl Into<VALUE>")
}

fn status(cloud: &Signature, server: &Signature) -> (Vec<&'static str>, Vec<String>) {
    let mut statuses = Vec::new();
    let mut notes = Vec::new();
    let mut parameters_differ = cloud.parameters.len() != server.parameters.len();
    let mut named_values = false;
    if cloud.parameters != server.parameters {
        for (cloud_parameter, server_parameter) in cloud.parameters.iter().zip(&server.parameters) {
            if cloud_parameter != server_parameter {
                let ((cloud_name, cloud_type), (server_name, server_type)) = (cloud_parameter, server_parameter);
                if cloud_name == server_name && values_named_on_one_side(cloud_type, server_type) {
                    named_values = true;
                } else {
                    parameters_differ = true;
                }
                if cloud_type == server_type {
                    notes.push(format!("`{cloud_name}` / `{server_name}`"));
                } else if cloud_name == server_name {
                    notes.push(format!("`{cloud_name}`: `{cloud_type}` / `{server_type}`"));
                } else {
                    notes.push(format!("`{cloud_name}: {cloud_type}` / `{server_name}: {server_type}`"));
                }
            }
        }
        if cloud.parameters.len() != server.parameters.len() {
            notes.push(format!(
                "{} Cloud positional parameters / {} Data Center",
                cloud.parameters.len(),
                server.parameters.len()
            ));
        }
    }
    for (name, cloud_type) in &cloud.setters {
        if let Some(server_type) = server.setters.get(name) {
            if cloud_type != server_type {
                if values_named_on_one_side(cloud_type, server_type) {
                    named_values = true;
                } else {
                    parameters_differ = true;
                }
                notes.push(format!("`{name}`: `{cloud_type}` / `{server_type}`"));
            }
        }
    }
    if parameters_differ {
        statuses.push("parameters differ");
    }
    if named_values {
        statuses.push("values named on one side");
    }
    let cloud_only: Vec<_> = cloud.setters.keys().filter(|key| !server.setters.contains_key(*key)).collect();
    let server_only: Vec<_> = server.setters.keys().filter(|key| !cloud.setters.contains_key(*key)).collect();
    if !cloud_only.is_empty() || !server_only.is_empty() {
        statuses.push("platform parameters");
        if !cloud_only.is_empty() {
            notes.push(format!(
                "Cloud only: {}",
                cloud_only.iter().map(|key| format!("`{key}`")).collect::<Vec<_>>().join(", ")
            ));
        }
        if !server_only.is_empty() {
            notes.push(format!(
                "Data Center only: {}",
                server_only.iter().map(|key| format!("`{key}`")).collect::<Vec<_>>().join(", ")
            ));
        }
    }
    if cloud.response != server.response {
        statuses.push("type differs");
        notes.push(format!("`{}` / `{}`", cloud.response, server.response));
    }
    if cloud.stream != server.stream {
        statuses.push("stream on one side");
        notes.push(format!("stream: {} only", if cloud.stream { "Cloud" } else { "Data Center" }));
    }
    if statuses.is_empty() {
        statuses.push("identical");
    }
    (statuses, notes)
}

fn row(operation: &Operation) -> String {
    format!("| `{}().{}` | `{}` |\n", operation.accessor, operation.method, operation.endpoint)
}

fn render(
    cloud: Vec<Operation>,
    server: Vec<Operation>,
    cloud_total: usize,
    server_total: usize,
) -> Result<String, Failure> {
    let cloud: BTreeMap<_, _> = cloud.into_iter().map(|operation| (operation.key(), operation)).collect();
    let server: BTreeMap<_, _> = server.into_iter().map(|operation| (operation.key(), operation)).collect();
    if cloud.len() != cloud_total || server.len() != server_total {
        return Err("duplicate accessor/method or operation count mismatch".into());
    }
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut paired = BTreeMap::<String, Vec<String>>::new();
    let mut server_only = Vec::new();
    let mut cloud_only = Vec::new();
    for (key, dc) in &server {
        if let Some(hosted) = cloud.get(key) {
            let (statuses, notes) = status(&hosted.signature, &dc.signature);
            for label in &statuses {
                *counts.entry(label).or_default() += 1;
            }
            paired.entry(key.0.clone()).or_default().push(format!(
                "| `{}` | `{}` | `{}` | {} | {} |\n",
                key.1,
                dc.endpoint,
                hosted.endpoint,
                statuses.join(", "),
                if notes.is_empty() { "—".to_owned() } else { notes.join("; ") },
            ));
        } else {
            server_only.push(row(dc));
        }
    }
    for (key, hosted) in &cloud {
        if !server.contains_key(key) {
            cloud_only.push(row(hosted));
        }
    }
    let paired_count: usize = paired.values().map(Vec::len).sum();
    if paired_count + server_only.len() != server_total || paired_count + cloud_only.len() != cloud_total {
        return Err(format!("parity counts do not add up: paired {paired_count}, Data Center only {}, Cloud only {}; expected server {server_total}, Cloud {cloud_total}", server_only.len(), cloud_only.len()).into());
    }
    let mut output = String::from(
        "# Cloud and Data Center parity\n\nThis matrix compares generated operations in Cloud (`CloudClient` and `AgileClient`) and Data Center (`ServerClient`). A paired row uses the same accessor and method on both clients; its endpoints show where each request goes. Cloud names are canonical. Statuses can overlap, and notes give the differing signatures. `values named on one side` means both parameters accept string literals, while only one exposes documented enum variants. Generated by `cargo xtask parity`.\n\n## Summary\n\n| Status | Operations |\n|---|---:|\n",
    );
    for label in [
        "identical",
        "values named on one side",
        "parameters differ",
        "platform parameters",
        "type differs",
        "stream on one side",
    ] {
        output.push_str(&format!("| {label} | {} |\n", counts.get(label).copied().unwrap_or(0)));
    }
    output.push_str(&format!("| Paired operations | {paired_count} |\n| Data Center only | {} |\n| Cloud only | {} |\n| All Data Center operations | {server_total} |\n| All Cloud operations | {cloud_total} |\n", server_only.len(), cloud_only.len()));
    output.push_str("\n## Paired operations\n");
    for (accessor, rows) in paired {
        output.push_str(&format!("\n### `{accessor}()`\n\n| Method | Data Center endpoint | Cloud endpoint | Status | Note |\n|---|---|---|---|---|\n"));
        output.push_str(&rows.concat());
    }
    output.push_str("\n## Data Center only\n\n| Accessor and method | Endpoint |\n|---|---|\n");
    output.push_str(&server_only.concat());
    output.push_str("\n## Cloud only\n\n| Accessor and method | Endpoint |\n|---|---|\n");
    output.push_str(&cloud_only.concat());
    Ok(output)
}

pub fn run(root: &Path, arguments: &[String]) -> Result<(), Failure> {
    let check = match arguments {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => return Err("usage: cargo xtask parity [--check]".into()),
    };
    let manifest: Vec<ManifestOperation> =
        serde_json::from_str(&std::fs::read_to_string(root.join("operations.json"))?)?;
    let mut cloud = read_surface(root, "cloud")?;
    cloud.extend(read_surface(root, "agile")?);
    let server = read_surface(root, "server")?;
    verify_manifest(&cloud, &manifest, &["cloud", "agile"])?;
    verify_manifest(&server, &manifest, &["server"])?;
    let output = render(
        cloud,
        server,
        manifest.iter().filter(|entry| entry.surface == "cloud" || entry.surface == "agile").count(),
        manifest.iter().filter(|entry| entry.surface == "server").count(),
    )?;
    let path = root.join("docs/parity.md");
    if check {
        if std::fs::read_to_string(&path).ok().as_deref() != Some(&output) {
            return Err(format!("{} differs from `cargo xtask parity` output", path.display()).into());
        }
        println!("{} is current", path.display());
    } else {
        std::fs::write(&path, output)?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_and_multiline_signatures_and_builders() {
        let source = r#"impl<'a> ExampleService<'a> {
    pub fn one(&self, id: impl Into<String>) -> OneRequest<'a> { OneRequest::new(self.client, id) }
    pub fn two(
        &self,
        id: impl Into<String>,
        number: i64,
    ) -> TwoRequest<'a> { TwoRequest::new(self.client, id, number) }
}
impl<'a> OneRequest<'a> {
    pub fn field(mut self, value: impl Into<String>) -> Self { self }
    pub fn stream(self) -> Stream { todo!() }
    pub async fn send(self) -> crate::core::Result<Vec<Thing>> { todo!() }
    pub fn config(&self) -> Config { RequestConfig::new(crate::core::Method::GET, "/one".to_owned()) }
}
impl<'a> TwoRequest<'a> {
    pub async fn send(self) -> crate::core::Result<Thing> { todo!() }
    pub fn config(&self) -> Config { RequestConfig::new(crate::core::Method::POST, format!("/two/{}", self.id)) }
}"#;
        let operations = parse_service("example", source).unwrap();
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].signature.parameters, vec![("id".to_owned(), "impl Into<String>".to_owned())]);
        assert_eq!(operations[0].signature.setters["field"], "impl Into<String>");
        assert!(operations[0].signature.stream);
        assert_eq!(operations[0].signature.response, "Vec<Thing>");
        assert_eq!(operations[1].signature.parameters.len(), 2);
        assert!(!operations[1].signature.stream);
        assert_eq!(operations[1].endpoint, "POST /two/{…}");
    }

    #[test]
    fn reports_every_pair_status() {
        let base =
            Signature { parameters: vec![], setters: BTreeMap::new(), response: "Thing".to_owned(), stream: false };
        assert_eq!(status(&base, &base).0, ["identical"]);
        let mut changed = base.clone();
        changed.parameters.push(("id".to_owned(), "i64".to_owned()));
        assert_eq!(status(&base, &changed).0, ["parameters differ"]);
        changed = base.clone();
        changed.setters.insert("shared".to_owned(), "i64".to_owned());
        let mut cloud = base.clone();
        cloud.setters.insert("shared".to_owned(), "String".to_owned());
        assert_eq!(status(&cloud, &changed).0, ["parameters differ"]);
        changed = base.clone();
        changed.setters.insert("only".to_owned(), "i64".to_owned());
        assert_eq!(status(&base, &changed).0, ["platform parameters"]);
        changed = base.clone();
        changed.response = "Other".to_owned();
        assert_eq!(status(&base, &changed).0, ["type differs"]);
        changed = base.clone();
        changed.stream = true;
        assert_eq!(status(&base, &changed).0, ["stream on one side"]);
    }

    #[test]
    fn separates_named_values_from_parameter_shape_differences() {
        let base = Signature {
            parameters: vec![],
            setters: BTreeMap::from([("expand".to_owned(), "impl IntoIterator<Item = impl Into<String>>".to_owned())]),
            response: "Thing".to_owned(),
            stream: false,
        };
        let mut cloud = base.clone();
        cloud.setters.insert(
            "expand".to_owned(),
            "impl IntoIterator<Item = impl Into<SearchIssuesRequestExpandValue>>".to_owned(),
        );
        assert_eq!(status(&cloud, &base).0, ["values named on one side"]);
        assert_eq!(status(&base, &cloud).0, ["values named on one side"]);

        let mut different_shape = base.clone();
        different_shape.setters.insert("expand".to_owned(), "impl Into<String>".to_owned());
        assert_eq!(status(&cloud, &different_shape).0, ["parameters differ"]);

        cloud.setters.insert("order_by".to_owned(), "impl Into<GetCommentsRequestOrderBy>".to_owned());
        different_shape.setters.insert("order_by".to_owned(), "impl Into<String>".to_owned());
        assert_eq!(status(&cloud, &different_shape).0, ["parameters differ", "values named on one side"]);

        let mut positional = base.clone();
        positional.parameters.push(("type".to_owned(), "impl Into<GetAvatarsRequestType>".to_owned()));
        let mut plain = base;
        plain.parameters.push(("type".to_owned(), "impl Into<String>".to_owned()));
        assert_eq!(status(&positional, &plain).0, ["values named on one side"]);
    }
}
