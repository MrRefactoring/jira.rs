use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::generated::{Operation, read_surface};

type Failure = Box<dyn std::error::Error>;

const SURFACES: &[&str] = &[
    "admin",
    "agile",
    "assets",
    "assets_server",
    "cloud",
    "server",
    "service_desk",
    "service_desk_server",
    "teams",
    "user_management",
    "user_provisioning",
];
const SETTERS: &[&str] =
    &["start_at", "start", "offset", "page", "cursor", "next_page_token", "start_index", "since", "after"];
const MEMBERS: &[&str] =
    &["next_page_token", "next_cursor", "is_last", "is_last_page", "last_page", "next_page", "end_cursor"];

type Fields = BTreeMap<String, String>;

#[derive(Default)]
struct Models {
    structs: BTreeMap<String, Fields>,
    aliases: BTreeMap<String, String>,
}

impl Models {
    fn read(root: &Path, surface: &str) -> Result<Self, Failure> {
        static STRUCT: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?m)^pub struct (\w+)(?:<[^\n]+>)? \{\n").unwrap());
        static FIELD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^\s+pub (\w+): ([^,\n]+),").unwrap());
        static ALIAS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^pub type (\w+) = ([^;]+);").unwrap());
        let mut models = Self::default();
        for entry in std::fs::read_dir(root.join("src").join(surface).join("models"))? {
            let path = entry?.path();
            if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
                continue;
            }
            let source = std::fs::read_to_string(path)?;
            for capture in STRUCT.captures_iter(&source) {
                let start = capture.get(0).unwrap().end();
                let end = source[start..].find("\n}").ok_or("model struct has no closing brace")? + start;
                let fields = FIELD
                    .captures_iter(&source[start..end])
                    .map(|field| (field[1].to_owned(), field[2].trim().to_owned()))
                    .collect();
                models.structs.insert(capture[1].to_owned(), fields);
            }
            for capture in ALIAS.captures_iter(&source) {
                models.aliases.insert(capture[1].to_owned(), capture[2].trim().to_owned());
            }
        }
        Ok(models)
    }

    fn fields(&self, ty: &str) -> Option<&Fields> {
        let mut ty = ty.trim();
        for _ in 0..8 {
            if let Some(inner) = ty.strip_prefix("Option<").and_then(|value| value.strip_suffix('>')) {
                ty = inner;
                continue;
            }
            let name = ty.split('<').next().unwrap_or(ty).rsplit("::").next().unwrap_or(ty);
            if let Some(alias) = self.aliases.get(name) {
                ty = alias;
                continue;
            }
            return self.structs.get(name);
        }
        None
    }
}

fn page_members(fields: &Fields, models: &Models) -> bool {
    MEMBERS.iter().any(|member| fields.contains_key(*member))
        || (fields.contains_key("start_at") && fields.contains_key("total"))
        || (fields.contains_key("start_index") && fields.contains_key("total_results"))
        || fields.get("links").is_some_and(|ty| models.fields(ty).is_some_and(|links| links.contains_key("next")))
}

fn looks_paginated(operation: &Operation, models: &Models) -> bool {
    SETTERS.iter().any(|setter| operation.signature.setters.contains_key(*setter))
        || models.fields(&operation.signature.response).is_some_and(|fields| {
            page_members(fields, models)
                || fields.values().any(|ty| models.fields(ty).is_some_and(|nested| page_members(nested, models)))
        })
}

fn read_registry(path: &Path) -> Result<BTreeMap<String, String>, Failure> {
    let contents = std::fs::read_to_string(path)?;
    let mut entries = BTreeMap::new();
    for (index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let at = format!("{}:{}", path.display(), index + 1);
        let (key, reason) =
            line.split_once(" — ").ok_or_else(|| format!("{at}: expected `surface accessor method — reason`"))?;
        if key.split_whitespace().count() != 3
            || reason.trim().is_empty()
            || entries.insert(key.to_owned(), reason.to_owned()).is_some()
        {
            return Err(format!("{at}: invalid or duplicate registry entry").into());
        }
    }
    Ok(entries)
}

fn key(surface: &str, operation: &Operation) -> String {
    format!("{surface} {} {}", operation.accessor, operation.method)
}

fn problems(operations: &[(String, Operation, bool)], registry: &BTreeMap<String, String>) -> Vec<String> {
    let mut failures = Vec::new();
    let known: BTreeMap<_, _> =
        operations.iter().map(|(surface, operation, looks)| (key(surface, operation), (operation, looks))).collect();
    for (name, (operation, looks)) in &known {
        if **looks && !operation.signature.stream && !registry.contains_key(name) {
            failures.push(format!("missing stream or reason: {name}"));
        }
    }
    for name in registry.keys() {
        match known.get(name) {
            None => failures.push(format!("stale registry entry (request removed): {name}")),
            Some((operation, _)) if operation.signature.stream => {
                failures.push(format!("stale registry entry (has stream): {name}"))
            }
            Some((_, false)) => failures.push(format!("stale registry entry (no longer looks paginated): {name}")),
            _ => {}
        }
    }
    failures
}

pub fn run(root: &Path, arguments: &[String]) -> Result<(), Failure> {
    let check = match arguments {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => return Err("usage: cargo xtask streams [--check]".into()),
    };
    let registry = read_registry(&root.join("xtask/streams.txt"))?;
    let mut operations = Vec::new();
    let mut counts = Vec::new();
    for surface in SURFACES {
        let models = Models::read(root, surface)?;
        let mut paginated = 0;
        let mut streamed = 0;
        let mut excused = 0;
        for operation in read_surface(root, surface)? {
            let looks = looks_paginated(&operation, &models);
            if looks {
                paginated += 1;
                streamed += usize::from(operation.signature.stream);
                excused += usize::from(registry.contains_key(&key(surface, &operation)) && !operation.signature.stream);
            }
            operations.push(((*surface).to_owned(), operation, looks));
        }
        counts.push((surface, paginated, streamed, excused));
    }
    println!("| Surface | Looks paginated | Stream | Excused |\n|---|---:|---:|---:|");
    for (surface, paginated, streamed, excused) in counts {
        println!("| {surface} | {paginated} | {streamed} | {excused} |");
    }
    let failures = problems(&operations, &registry);
    if check && !failures.is_empty() {
        return Err(failures.join("\n").into());
    }
    if !check {
        let missing: BTreeSet<_> = failures.iter().filter(|failure| failure.starts_with("missing ")).collect();
        for failure in missing {
            println!("{failure}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::Signature;

    fn operation() -> Operation {
        Operation {
            accessor: "example".to_owned(),
            method: "get".to_owned(),
            endpoint: "GET /example".to_owned(),
            signature: Signature {
                parameters: vec![],
                setters: BTreeMap::new(),
                response: "Answer".to_owned(),
                stream: false,
            },
        }
    }

    #[test]
    fn recognizes_each_setter() {
        for setter in SETTERS {
            let mut request = operation();
            request.signature.setters.insert((*setter).to_owned(), "i64".to_owned());
            assert!(looks_paginated(&request, &Models::default()), "{setter}");
        }
    }

    #[test]
    fn recognizes_each_answer_shape_and_negative() {
        let mut models = Models::default();
        let request = operation();
        assert!(!looks_paginated(&request, &models));
        for member in MEMBERS {
            models.structs.insert("Answer".to_owned(), BTreeMap::from([((*member).to_owned(), "bool".to_owned())]));
            assert!(looks_paginated(&request, &models), "{member}");
        }
        for pair in [["start_at", "total"], ["start_index", "total_results"]] {
            models.structs.insert(
                "Answer".to_owned(),
                pair.into_iter().map(|field| (field.to_owned(), "i64".to_owned())).collect(),
            );
            assert!(looks_paginated(&request, &models));
        }
        models.structs.insert("Links".to_owned(), BTreeMap::from([("next".to_owned(), "Option<String>".to_owned())]));
        models.structs.insert("Answer".to_owned(), BTreeMap::from([("links".to_owned(), "Option<Links>".to_owned())]));
        assert!(looks_paginated(&request, &models));
        models.structs.insert("Wrapper".to_owned(), BTreeMap::from([("page".to_owned(), "Option<Answer>".to_owned())]));
        let mut nested = request;
        nested.signature.response = "Option<Wrapper>".to_owned();
        assert!(looks_paginated(&nested, &models));
    }

    #[test]
    fn stale_entries_are_errors() {
        let mut request = operation();
        let name = key("cloud", &request);
        let registry = BTreeMap::from([(name.clone(), "A finite picker.".to_owned())]);
        let mut operations = vec![("cloud".to_owned(), request.clone(), true)];
        assert!(problems(&operations, &registry).is_empty());
        request.signature.stream = true;
        operations[0].1 = request;
        assert!(problems(&operations, &registry)[0].contains("has stream"));
        operations.clear();
        assert!(problems(&operations, &registry)[0].contains("removed"));
        operations.push(("cloud".to_owned(), operation(), false));
        assert!(problems(&operations, &registry)[0].contains("no longer looks paginated"));
    }
}
