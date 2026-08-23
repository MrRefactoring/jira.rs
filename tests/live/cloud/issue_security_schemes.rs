use crate::harness::{TEST_PROJECT_KEY, cloud};

/// The issue security schemes API and the neighbouring security level reads, read-only throughout.
///
/// There is no write half in this module at all — issue security schemes are created through a different, admin-only
/// API entirely.
///
/// Worth its own file because issue security is the one mechanism in Jira that can make an issue *invisible* rather
/// than merely read-only. Every other suite reads the issues it creates freely; that only holds because the test
/// project has no security scheme attached, and this file is what establishes it rather than assuming it.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_site_security_schemes_or_refuses_typed_without_admin_rights() {
    let schemes = match cloud().issue_security_schemes().get_issue_security_schemes().send().await {
        Ok(schemes) => schemes,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            return;
        }
    };

    let schemes = schemes.issue_security_schemes.expect("the listing carries a list, empty or not");

    for scheme in &schemes {
        assert!(scheme.id.is_some_and(|id| id > 0), "a security scheme carries an id");
        assert!(scheme.name.as_ref().is_some_and(|name| !name.is_empty()), "a security scheme carries a name");
        assert!(
            scheme.self_.as_deref().is_some_and(|url| url.starts_with("https://")),
            "a security scheme carries its own address: {:?}",
            scheme.self_,
        );
    }
}

/// The premise the rest of the live suites rest on: nothing hides the issues they create.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn confirms_the_test_project_has_no_security_scheme() {
    let error = cloud()
        .project_permission_schemes()
        .get_project_issue_security_scheme(TEST_PROJECT_KEY)
        .send()
        .await
        .expect_err("a project without a security scheme has none to report");

    assert!(error.is_not_found(), "an unattached project answers not-found rather than an empty scheme: {error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_no_security_levels_for_a_project_without_a_scheme() {
    let levels = cloud()
        .project_permission_schemes()
        .get_security_levels_for_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the security levels of the test project read back");

    assert!(levels.levels.is_empty(), "no scheme means no levels, got {}", levels.levels.len());
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_scheme_as_a_typed_error() {
    let error = cloud()
        .issue_security_schemes()
        .get_issue_security_scheme(99_999_999)
        .send()
        .await
        .expect_err("a security scheme that does not exist cannot be read");

    assert!(error.is_not_found() || error.is_forbidden(), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_security_level_as_a_typed_error() {
    let error = cloud()
        .issue_security_level()
        .get_issue_security_level("99999999")
        .send()
        .await
        .expect_err("a security level that does not exist cannot be read");

    assert!(error.is_not_found() || error.is_forbidden(), "{error}");
}
