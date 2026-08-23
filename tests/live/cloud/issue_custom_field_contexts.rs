use crate::harness::cloud;

/// Custom field contexts and the options that hang off them, read-only.
///
/// A context decides which projects and issue types a custom field applies to, and its options are the values a
/// select field offers. Both are shared configuration: adding an option makes it selectable everywhere the context
/// applies, and deleting one leaves issues holding a value that no longer exists. So nothing here writes, and the
/// destructive path is aimed only at a field that cannot exist.
///
/// The concept worth pinning is that a custom field is not a single thing. It has contexts, each with its own options
/// and its own default, and "the field's value list" is meaningless without naming which context. That indirection is
/// behind a lot of confusion about why an option appears in one project and not another.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_contexts_of_a_custom_field_or_refuses_typed() {
    let Some(field_id) = readable_custom_field().await else {
        return;
    };

    let page = cloud()
        .issue_custom_field_contexts()
        .get_contexts_for_field(&field_id)
        .max_results(10)
        .send()
        .await
        .expect("the contexts of a custom field read back");

    assert_eq!(page.max_results, 10, "the page size asked for is the page size returned");
    assert!(page.values.len() <= 10, "a page holds no more than it says it does");

    for context in &page.values {
        assert!(context.id.chars().all(|c| c.is_ascii_digit()), "a context id is digits: {}", context.id);
        assert!(!context.name.is_empty(), "a context carries a name");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn filters_contexts_by_scope() {
    let Some(field_id) = readable_custom_field().await else {
        return;
    };

    let page = cloud()
        .issue_custom_field_contexts()
        .get_contexts_for_field(&field_id)
        .is_global_context(true)
        .max_results(10)
        .send()
        .await
        .expect("the scope filter is accepted");

    assert_eq!(page.max_results, 10, "the page size asked for is the page size returned");

    for context in &page.values {
        assert!(context.is_global_context, "the scope filter returns only the contexts that apply to every project");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_projects_a_context_applies_to() {
    let Some((field_id, context_id)) = a_context().await else {
        return;
    };

    let page = match cloud()
        .issue_custom_field_contexts()
        .get_project_context_mapping(&field_id)
        .context_id([context_id])
        .send()
        .await
    {
        Ok(page) => page,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.is_not_found() || error.status() == Some(400),
                "a refused project mapping is typed: {error}",
            );

            return;
        }
    };

    assert!(page.max_results > 0, "a page declares the size it was capped at");

    for mapping in &page.values {
        assert_eq!(
            mapping.context_id,
            context_id.to_string(),
            "filtering by context returns only that context's mappings",
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_issue_types_a_context_applies_to() {
    let Some((field_id, context_id)) = a_context().await else {
        return;
    };

    let page = match cloud()
        .issue_custom_field_contexts()
        .get_issue_type_mappings_for_contexts(&field_id)
        .context_id([context_id])
        .send()
        .await
    {
        Ok(page) => page,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.is_not_found() || error.status() == Some(400),
                "a refused issue type mapping is typed: {error}",
            );

            return;
        }
    };

    assert!(page.max_results > 0, "a page declares the size it was capped at");

    for mapping in &page.values {
        assert_eq!(
            mapping.context_id,
            context_id.to_string(),
            "filtering by context returns only that context's mappings",
        );
    }
}

/// The options of a context are what a select field offers. A field of any other type has none, and the endpoint says
/// so with a 400 rather than an empty page, so both outcomes are accepted and only their shape is pinned.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_options_of_a_context() {
    let Some((field_id, context_id)) = a_context().await else {
        return;
    };

    let page = match cloud()
        .issue_custom_field_options()
        .get_options_for_context(&field_id, context_id)
        .max_results(20)
        .send()
        .await
    {
        Ok(page) => page,
        Err(error) => {
            assert!(error.status().is_some_and(|status| status >= 400), "{error}");

            return;
        }
    };

    assert_eq!(page.max_results, 20, "the page size asked for is the page size returned");

    for option in &page.values {
        assert!(option.id.chars().all(|c| c.is_ascii_digit()), "an option id is digits: {}", option.id);
        assert!(!option.value.is_empty(), "an option carries the value a user would pick");
    }
}

/// A default belongs to a context, not to the field — which is why the same field can arrive pre-filled in one
/// project and empty in another.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_default_values_per_context_rather_than_per_field() {
    let Some(field_id) = readable_custom_field().await else {
        return;
    };

    let page = match cloud()
        .issue_custom_field_contexts()
        .get_context_default_values(&field_id)
        .max_results(10)
        .send()
        .await
    {
        Ok(page) => page,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.is_not_found() || error.status() == Some(400),
                "a refused default value listing is typed: {error}",
            );

            return;
        }
    };

    assert_eq!(page.max_results, 10, "the page size asked for is the page size returned");

    for value in &page.values {
        assert!(value.context_id > 0, "a default value names the context it belongs to");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_field_as_a_typed_error() {
    let error = cloud()
        .issue_custom_field_contexts()
        .get_contexts_for_field("customfield_99999999")
        .send()
        .await
        .expect_err("a field that does not exist has no contexts to list");

    assert!(error.is_not_found() || error.is_forbidden() || error.status() == Some(400), "{error}");
}

/// The destructive path, proven through its error channel and never aimed at a context that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path_without_ever_aiming_it_at_a_real_context() {
    let error = cloud()
        .issue_custom_field_contexts()
        .delete_custom_field_context("customfield_99999999", 99_999_999)
        .send()
        .await
        .expect_err("a context that does not exist cannot be deleted");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// A custom field on the site whose contexts the token may read.
///
/// Reading a context needs *Administer Jira*, so a token without it must be refused in a way the caller can
/// recognise; that refusal is asserted here rather than being silently swallowed by the tests that stand down on it.
/// A site with no custom field at all is left to the tests to skip, which is why the field listing itself is pinned.
async fn readable_custom_field() -> Option<String> {
    let fields = cloud().issue_fields().get_fields().send().await.expect("the site lists its fields");

    assert!(fields.iter().any(|field| field.id.as_deref() == Some("summary")), "every site carries the summary field");

    let field_id = fields.iter().find(|field| field.custom == Some(true)).and_then(|field| field.id.clone())?;

    let readable = cloud().issue_custom_field_contexts().get_contexts_for_field(&field_id).max_results(1).send().await;

    match readable {
        Ok(_) => Some(field_id),
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            None
        }
    }
}

/// A readable custom field paired with the first of its contexts, where it has one.
async fn a_context() -> Option<(String, i64)> {
    let field_id = readable_custom_field().await?;

    let page = cloud()
        .issue_custom_field_contexts()
        .get_contexts_for_field(&field_id)
        .max_results(1)
        .send()
        .await
        .expect("the contexts of a custom field read back");

    let context = page.values.first()?;
    let context_id: i64 = context.id.parse().expect("a context id is a number");

    Some((field_id, context_id))
}
