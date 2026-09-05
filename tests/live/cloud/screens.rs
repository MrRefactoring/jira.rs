use jira::cloud::GetScreensRequestScope;

use crate::harness::cloud;

/// The screens API, read-only.
///
/// A screen decides which fields appear when an issue is created, edited or transitioned. It is site-wide shared
/// configuration with no project-scoped variant to contain a mistake — adding a field to the default screen changes
/// the create dialog for every user of every project — so the write half is pinned only through its error channel.
///
/// The read half earns its place: `get_screens_for_field` answers "where would a user actually see this field",
/// which is the question behind most "why is my custom field missing" reports. A field can exist, be on the right
/// issue type, and still be invisible because no screen carries it.
///
/// Every read here needs *Administer Jira*, so each test first proves the token either has it or is refused typed.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_screen_listing_for_an_admin_or_fails_typed() {
    if !may_read_screens().await {
        return;
    }

    let page = cloud().screens().get_screens().max_results(5).send().await.expect("an admin lists the screens");

    assert_eq!(page.max_results, 5, "the page size asked for is the page size returned");
    assert_eq!(page.start_at, 0, "an unoffset request starts at the beginning");
    assert!(page.values.len() <= 5, "a page holds no more than it says it does");
    assert!(page.is_last || page.values.len() == 5, "a page that is not the last one is full");

    for screen in &page.values {
        assert!(screen.id.is_some_and(|id| id > 0), "a screen carries an id");
        assert!(screen.name.as_ref().is_some_and(|name| !name.is_empty()), "a screen carries a name");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn distinguishes_global_screens_from_project_scoped_ones() {
    if !may_read_screens().await {
        return;
    }

    let page = cloud().screens().get_screens().max_results(50).send().await.expect("an admin lists the screens");

    assert!(!page.values.is_empty(), "a site always carries at least the screens its default configuration needs");

    for screen in &page.values {
        let Some(kind) = screen.scope.as_ref().and_then(|scope| scope.r#type.as_ref()) else { continue };

        assert!(
            matches!(kind.as_str(), "GLOBAL" | "TEMPLATE" | "PROJECT"),
            "a scope names one of the three kinds, got {}",
            kind.as_str(),
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn filters_the_listing_by_scope() {
    if !may_read_screens().await {
        return;
    }

    let page = cloud()
        .screens()
        .get_screens()
        .scope([GetScreensRequestScope::Global])
        .max_results(50)
        .send()
        .await
        .expect("the scope filter is accepted");

    for screen in &page.values {
        let Some(kind) = screen.scope.as_ref().and_then(|scope| scope.r#type.as_ref()) else { continue };

        assert_eq!(kind.as_str(), "GLOBAL", "the scope filter returns only what it was asked for");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_where_a_custom_field_would_be_visible() {
    if !may_read_screens().await {
        return;
    }

    let fields = cloud().issue_fields().get_fields().send().await.expect("the site lists its fields");

    assert!(fields.iter().any(|field| field.id.as_deref() == Some("summary")), "every site carries the summary field");

    let Some(custom) = fields.iter().find(|field| field.custom == Some(true)).and_then(|field| field.id.clone()) else {
        return;
    };

    let page = cloud()
        .screens()
        .get_screens_for_field(custom.as_str())
        .max_results(10)
        .send()
        .await
        .expect("a custom field can be traced to the screens it is used in");

    for screen in &page.values {
        assert!(screen.id.is_some_and(|id| id > 0), "a screen a field is used in carries an id");
    }
}

/// A system field is on every screen and yet the index answers 404 for it, because the index only tracks custom
/// fields. That asymmetry is the trap the endpoint sets, so it is asserted rather than assumed.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_a_system_field_outright_despite_it_being_on_every_screen() {
    if !may_read_screens().await {
        return;
    }

    let error = cloud()
        .screens()
        .get_screens_for_field("summary")
        .max_results(10)
        .send()
        .await
        .expect_err("a system field is not carried by the field-to-screen index");

    assert!(error.is_not_found(), "the index answers for custom fields only: {error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_an_unknown_custom_field_the_same_way() {
    if !may_read_screens().await {
        return;
    }

    let error = cloud()
        .screens()
        .get_screens_for_field("customfield_99999999")
        .max_results(10)
        .send()
        .await
        .expect_err("a custom field that does not exist cannot be traced");

    assert!(error.is_not_found(), "an unknown field is refused the same way a system field is: {error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_fields_that_could_still_be_added_to_a_screen() {
    if !may_read_screens().await {
        return;
    }

    let page = cloud().screens().get_screens().max_results(1).send().await.expect("an admin lists the screens");
    let screen = page.values.first().expect("a site carries at least one screen");
    let screen_id = screen.id.expect("a screen carries an id");

    let available = match cloud().screens().get_available_screen_fields(screen_id).send().await {
        Ok(available) => available,
        Err(error) => {
            assert!(error.is_forbidden() || error.is_not_found(), "a refused availability listing is typed: {error}");

            return;
        }
    };

    for field in &available {
        assert!(field.id.as_ref().is_some_and(|id| !id.is_empty()), "an addable field carries an id");
        assert!(field.name.as_ref().is_some_and(|name| !name.is_empty()), "an addable field carries a name");
    }
}

/// The site-wide write, proven through its error channel and never aimed at a field that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_site_wide_write() {
    let error = cloud()
        .screens()
        .add_field_to_default_screen("customfield_99999999")
        .send()
        .await
        .expect_err("a field that does not exist cannot be added to the default screen");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// Whether the token may read the screen configuration at all.
///
/// Every screen read needs *Administer Jira*. A token without it must be refused in a way the caller can recognise,
/// so the refusal is asserted here rather than being silently swallowed by the tests that stand down on it.
async fn may_read_screens() -> bool {
    match cloud().screens().get_screens().max_results(1).send().await {
        Ok(_) => true,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            false
        }
    }
}
