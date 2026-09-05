use jira::cloud::{
    AnalyseExpressionRequestCheck, IdOrKey, JiraExpressionEvaluateContext, JiraExpressionEvaluateRequest,
    JiraExpressionForAnalysis,
};

use crate::harness::{ResourceTracker, cloud, create_test_issue, test_name};

/// Jira expressions: a small sandboxed language the server evaluates against a context of issues, projects and users.
///
/// Nothing about them can be tested without a live site — the whole point is what the server computes, and there is no
/// client-side evaluation to check against. Read-only in effect: an expression can read the context it is given but
/// cannot mutate anything.
///
/// The distinction worth pinning is analysis versus evaluation. `analyse_expression` type-checks an expression without
/// running it, and its three `check` modes answer three different questions; a caller who only ever evaluates finds
/// out about a bad expression at the worst possible moment.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn evaluates_a_constant_expression_with_no_context_at_all() {
    let request = JiraExpressionEvaluateRequest { expression: "1 + 1".to_owned(), context: None };

    match cloud().jira_expressions().evaluate_jsis_jira_expression(request).send().await {
        // A float rather than an integer: Jira's expression engine answers `2.0`, so reading it as an integer
        // finds nothing at all.
        Ok(result) => assert_eq!(result.value.as_f64(), Some(2.0), "the server does the arithmetic, not the client"),
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "{error}"),
    }
}

/// `user` is in scope without ever being asked for, which is what "implicit context" means here.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reads_the_current_user_from_the_implicit_context() {
    let me = cloud().myself().get_current_user().send().await.expect("the site names the current user");
    let account_id = me.account_id.expect("the current user carries an account id");

    let request = JiraExpressionEvaluateRequest { expression: "user.accountId".to_owned(), context: None };

    match cloud().jira_expressions().evaluate_jsis_jira_expression(request).send().await {
        Ok(result) => assert_eq!(result.value.as_str(), Some(account_id.as_str()), "the expression sees the caller"),
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "{error}"),
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reads_an_issue_passed_explicitly_in_the_context() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("expression subject"))).await;

    let request = JiraExpressionEvaluateRequest {
        expression: "issue.key".to_owned(),
        context: Some(JiraExpressionEvaluateContext {
            issue: Some(IdOrKey { key: Some(issue.key.clone()), id: None }),
            ..JiraExpressionEvaluateContext::default()
        }),
    };

    match cloud().jira_expressions().evaluate_jsis_jira_expression(request).send().await {
        Ok(result) => {
            assert_eq!(result.value.as_str(), Some(issue.key.as_str()), "the issue given in context is the one read");
        }
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "{error}"),
    }

    tracker.cleanup().await;
}

/// The same expression as above, minus the context: what is not passed is not in scope.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_an_expression_that_references_something_not_in_context() {
    let request = JiraExpressionEvaluateRequest { expression: "issue.key".to_owned(), context: None };

    let error = cloud()
        .jira_expressions()
        .evaluate_jsis_jira_expression(request)
        .send()
        .await
        .expect_err("an issue that was never put in context cannot be read");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn analyses_an_expression_for_syntax_without_evaluating_it() {
    let analysis = cloud()
        .jira_expressions()
        .analyse_expression(JiraExpressionForAnalysis {
            expressions: vec!["1 + 1".to_owned()],
            ..JiraExpressionForAnalysis::default()
        })
        .check(AnalyseExpressionRequestCheck::Syntax)
        .send()
        .await
        .expect("a syntax check is accepted");

    let result = analysis.results.first().expect("one expression in, one result out");

    assert!(result.valid, "a well-formed expression parses");
    assert_eq!(result.expression, "1 + 1", "the result names the expression it belongs to");
}

/// A malformed expression is a *result*, not a failure: the analysis endpoint answers 200 and reports the errors.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_a_malformed_expression_as_invalid_rather_than_throwing() {
    let analysis = cloud()
        .jira_expressions()
        .analyse_expression(JiraExpressionForAnalysis {
            expressions: vec!["1 +".to_owned()],
            ..JiraExpressionForAnalysis::default()
        })
        .check(AnalyseExpressionRequestCheck::Syntax)
        .send()
        .await
        .expect("a malformed expression is analysed rather than refused");

    let result = analysis.results.first().expect("one expression in, one result out");

    assert!(!result.valid, "an expression that does not parse is not valid");
    assert!(
        result.errors.as_ref().is_some_and(|errors| !errors.is_empty()),
        "an invalid expression says what is wrong with it",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn estimates_complexity_when_asked_to() {
    let analysis = cloud()
        .jira_expressions()
        .analyse_expression(JiraExpressionForAnalysis {
            expressions: vec!["issues.map(i => i.key)".to_owned()],
            ..JiraExpressionForAnalysis::default()
        })
        .check(AnalyseExpressionRequestCheck::Complexity)
        .send()
        .await
        .expect("a complexity check is accepted");

    let result = analysis.results.first().expect("one expression in, one result out");
    let complexity = result.complexity.as_ref().expect("the complexity check is what fills the complexity in");

    assert!(!complexity.expensive_operations.is_empty(), "the estimate names how many expensive operations there are");
}
