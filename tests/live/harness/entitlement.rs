pub fn is_not_entitled(error: &jira::Error) -> bool {
    let Some(body) = error.body() else { return false };
    let rendered = body.to_string().to_lowercase();

    rendered.contains("free plan") || rendered.contains("not entitled to")
}
