use crate::harness::cloud;

/// Screen schemes, tabs and tab fields — the three layers between a field and the form a user actually sees.
///
/// Read-only. The chain is issue type screen scheme → screen scheme → screen → tab → field, and every link is
/// shared configuration: one screen serves many projects, so moving a field on a tab moves it for all of them. There
/// is no project-scoped variant of any of it, which is why the destructive path is proven only through its error
/// channel.
///
/// Covering the three modules together is deliberate. Individually each is a thin listing; what is worth asserting
/// is that the chain resolves end to end, because a break anywhere in it is what makes a correctly configured field
/// invisible.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_screen_scheme_listing_for_an_admin_or_fails_typed() {
    if !may_read_screen_schemes().await {
        return;
    }

    let page = cloud()
        .screen_schemes()
        .get_screen_schemes()
        .max_results(5)
        .send()
        .await
        .expect("an admin lists the screen schemes");

    assert_eq!(page.max_results, 5, "the page size asked for is the page size returned");
    assert_eq!(page.start_at, 0, "an unoffset request starts at the beginning");
    assert!(page.values.len() <= 5, "a page holds no more than it says it does");

    for scheme in &page.values {
        assert!(scheme.id.is_some_and(|id| id > 0), "a screen scheme carries an id");
        assert!(scheme.name.as_ref().is_some_and(|name| !name.is_empty()), "a screen scheme carries a name");
        assert!(
            scheme.screens.as_ref().is_some_and(|screens| screens.default > 0),
            "every screen scheme names a default screen, whatever else it leaves unset",
        );
    }
}

/// The whole chain in one walk: a scheme names a screen that exists, that screen has tabs, and a tab has the fields
/// the user ends up looking at. Split across tests each step would refetch the step before it, and a break in the
/// middle would read as several unrelated failures instead of one broken chain.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_screen_scheme_down_to_the_fields_on_a_tab() {
    if !may_read_screen_schemes().await {
        return;
    }

    let schemes = cloud()
        .screen_schemes()
        .get_screen_schemes()
        .max_results(1)
        .send()
        .await
        .expect("an admin lists the screen schemes");

    let scheme = schemes.values.first().expect("a site carries at least one screen scheme");
    let screen_id = scheme.screens.as_ref().expect("a screen scheme names its screens").default;

    let screens = cloud()
        .screens()
        .get_screens()
        .id([screen_id])
        .max_results(1)
        .send()
        .await
        .expect("the screen a scheme points at can be looked up by id");

    assert_eq!(
        screens.values.first().and_then(|screen| screen.id),
        Some(screen_id),
        "the default screen of a scheme is a screen that exists",
    );

    let tabs = match cloud().screen_tabs().get_all_screen_tabs(screen_id).send().await {
        Ok(tabs) => tabs,
        Err(error) => {
            assert!(error.is_forbidden() || error.is_not_found(), "a refused tab listing is typed: {error}");

            return;
        }
    };

    for tab in &tabs {
        assert!(tab.id.is_some_and(|id| id > 0), "a tab carries an id");
        assert!(!tab.name.is_empty(), "a tab carries a name");
    }

    let Some(tab_id) = tabs.first().and_then(|tab| tab.id) else { return };

    let fields = match cloud().screen_tab_fields().get_all_screen_tab_fields(screen_id, tab_id).send().await {
        Ok(fields) => fields,
        Err(error) => {
            assert!(error.is_forbidden() || error.is_not_found(), "a refused field listing is typed: {error}");

            return;
        }
    };

    for field in &fields {
        assert!(field.id.as_ref().is_some_and(|id| !id.is_empty()), "a field on a tab carries an id");
        assert!(field.name.as_ref().is_some_and(|name| !name.is_empty()), "a field on a tab carries a name");
    }
}

/// The destructive path, proven through its error channel and never aimed at a scheme that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .screen_schemes()
        .delete_screen_scheme("99999999")
        .send()
        .await
        .expect_err("a screen scheme that does not exist cannot be deleted");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// Whether the token may read the screen scheme configuration at all.
///
/// Every read in this chain needs *Administer Jira*. A token without it must be refused in a way the caller can
/// recognise, so the refusal is asserted here rather than being silently swallowed by the tests that stand down.
async fn may_read_screen_schemes() -> bool {
    match cloud().screen_schemes().get_screen_schemes().max_results(1).send().await {
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
