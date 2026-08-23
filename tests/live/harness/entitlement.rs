/// Whether the site refused because of the plan it is on, not because of the request.
///
/// Project role actor writes and audit logs are paid-plan features. A site on a Free plan answers each with a refusal
/// that names the plan rather than anything the caller did — a 400 saying role actors cannot be updated "as it's on
/// the Jira Software Free plan", a 403 saying audit logs "aren't available for this site".
///
/// None of that is drift and none of it is breakage: the endpoints are correct and the library reaches them. Treated
/// as a failure, a lapsed trial turns the nightly run red for days and buries the signal it exists to carry. The
/// suites that need those features check this and stand down instead, visibly.
pub fn is_not_entitled(error: &jira::Error) -> bool {
    let Some(body) = error.body() else { return false };
    let rendered = body.to_string().to_lowercase();

    rendered.contains("free plan") || rendered.contains("not entitled to")
}
