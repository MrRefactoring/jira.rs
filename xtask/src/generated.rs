use std::collections::BTreeMap;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

type Failure = Box<dyn std::error::Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Signature {
    pub(crate) parameters: Vec<(String, String)>,
    pub(crate) setters: BTreeMap<String, String>,
    pub(crate) response: String,
    pub(crate) stream: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct Operation {
    pub(crate) accessor: String,
    pub(crate) method: String,
    pub(crate) endpoint: String,
    pub(crate) signature: Signature,
}

impl Operation {
    pub(crate) fn key(&self) -> (String, String) {
        (self.accessor.clone(), self.method.clone())
    }
}

fn normalize(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn split_parameters(value: &str) -> Vec<(String, String)> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth = 0_i32;
    for (index, ch) in value.char_indices() {
        match ch {
            '<' | '(' | '[' => depth += 1,
            '>' | ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(&value[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }
    parts.push(&value[start..]);
    parts
        .into_iter()
        .filter_map(|part| {
            let (name, ty) = part.trim().split_once(':')?;
            Some((name.trim().to_owned(), normalize(ty)))
        })
        .collect()
}

fn methods(body: &str) -> Vec<(String, Vec<(String, String)>, String)> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?s)pub (?:async )?fn (\w+)\s*\((.*?)\)\s*->\s*([^\{]+)\{").unwrap());
    RE.captures_iter(body)
        .map(|capture| (capture[1].to_owned(), split_parameters(&capture[2]), normalize(&capture[3])))
        .collect()
}

fn implementations(source: &str) -> Vec<(String, String)> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^impl(?:<'a>)? (\w+)(?:<'a>)? \{\n").unwrap());
    let mut result = Vec::new();
    for capture in RE.captures_iter(source) {
        let start = capture.get(0).unwrap().end();
        let end = source[start..].find("\n}").expect("generated impl has a closing brace") + start;
        result.push((capture[1].to_owned(), source[start..end].to_owned()));
    }
    result
}

fn endpoint(body: &str) -> Result<String, Failure> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"RequestConfig::new\(\s*crate::core::Method::(\w+),\s*(?:format!\()?\s*"([^"]+)""#).unwrap()
    });
    let capture = RE.captures(body).ok_or("request builder has no endpoint")?;
    let path = capture[2].replace("{}", "{…}");
    Ok(format!("{} {path}", &capture[1]))
}

pub(crate) fn parse_service(accessor: &str, source: &str) -> Result<Vec<Operation>, Failure> {
    let implementations = implementations(source);
    let service = implementations.iter().find(|(name, _)| name.ends_with("Service")).ok_or("missing service impl")?;
    let builders: BTreeMap<_, _> =
        implementations.iter().filter(|(name, _)| name.ends_with("Request")).cloned().collect();
    let mut operations = Vec::new();
    for (name, parameters, response) in methods(&service.1) {
        if name == "new" {
            continue;
        }
        let builder_name = response.trim_end_matches("<'a>");
        let body = builders.get(builder_name).ok_or_else(|| format!("missing builder {builder_name}"))?;
        let builder_methods = methods(body);
        let setters = builder_methods
            .iter()
            .filter(|(name, _, response)| response == "Self" && name != "new")
            .filter_map(|(name, parameters, _)| parameters.first().map(|(_, ty)| (name.clone(), ty.clone())))
            .collect();
        let response = builder_methods
            .iter()
            .find(|(name, _, _)| name == "send")
            .ok_or_else(|| format!("missing send in {builder_name}"))?
            .2
            .trim_start_matches("crate::core::Result<")
            .strip_suffix('>')
            .ok_or("invalid send result")?
            .to_owned();
        let stream = builder_methods.iter().any(|(name, _, _)| name == "stream");
        operations.push(Operation {
            accessor: accessor.to_owned(),
            method: name,
            endpoint: endpoint(body)?,
            signature: Signature { parameters, setters, response, stream },
        });
    }
    Ok(operations)
}

pub(crate) fn read_surface(root: &Path, surface: &str) -> Result<Vec<Operation>, Failure> {
    let api = root.join("src").join(surface).join("api");
    let mut files =
        std::fs::read_dir(api)?.map(|entry| entry.map(|entry| entry.path())).collect::<Result<Vec<_>, _>>()?;
    files.sort();
    let mut operations = Vec::new();
    for file in files {
        if file.extension().and_then(|extension| extension.to_str()) != Some("rs")
            || file.file_name().unwrap() == "mod.rs"
        {
            continue;
        }
        let accessor = file.file_stem().unwrap().to_str().ok_or("invalid accessor filename")?;
        operations.extend(parse_service(accessor, &std::fs::read_to_string(&file)?)?);
    }
    Ok(operations)
}
