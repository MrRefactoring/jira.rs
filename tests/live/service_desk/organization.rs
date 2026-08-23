use jira::service_desk::CustomerCreate;

use crate::harness::service_desk;

/// The Service Management `organization` and `customer` APIs.
///
/// Both are gated behind an agent licence a site may not hold — every service-desk endpoint then answers 403 with an
/// empty body, as the `info` suite establishes. Rather than skip into vacuity, these assert the refusal is *typed* on
/// each endpoint a caller would reach for, which is the part the library is responsible for and the part that stays
/// true whether or not a licence is ever added.
///
/// The write halves would not be exercised even with a licence: creating an organization or a customer creates a real
/// identity on the tenant, and revoking portal-only access removes someone's access to the portal. They are proven
/// through their error channel instead, aimed at input that cannot succeed.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_organizations_or_refuses_typed_without_an_agent_licence() {
    let page = match service_desk().organization().get_organizations().limit(5).send().await {
        Ok(page) => page,
        Err(error) => {
            assert!(error.is_forbidden(), "an account without an agent licence is refused by rights: {error}");

            return;
        }
    };

    assert!(page.is_last_page.is_some(), "a page says whether it is the last one");
    assert!(page.values.len() <= 5, "a page holds no more than the limit asked for: {}", page.values.len());

    for organization in &page.values {
        assert!(organization.id.as_ref().is_some_and(|id| !id.is_empty()), "an organization carries a generated id");
        assert!(organization.name.is_some(), "and the name it is displayed under");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_with_start_and_limit_as_the_rest_of_this_surface_does() {
    let page = match service_desk().organization().get_organizations().start(0).limit(1).send().await {
        Ok(page) => page,
        Err(error) => {
            assert!(error.is_forbidden(), "an account without an agent licence is refused by rights: {error}");

            return;
        }
    };

    assert_eq!(page.limit, Some(1), "the page size asked for is the page size returned");
    assert_eq!(page.start, Some(0), "an unoffset request starts at the beginning");
    assert!(page.values.len() <= 1, "a page holds no more than it says it does: {}", page.values.len());
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_a_single_organization_lookup_typed() {
    let error = service_desk()
        .organization()
        .get_organization(99_999_999)
        .send()
        .await
        .expect_err("an organization that does not exist cannot be read");

    assert!(
        error.is_forbidden() || error.status() == Some(404),
        "the lookup is refused by licence or reported missing, never untyped: {error}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_organization_property_reads_typed() {
    let error = service_desk()
        .organization()
        .get_properties_keys("99999999")
        .send()
        .await
        .expect_err("the properties of an organization that does not exist cannot be read");

    assert!(
        error.is_forbidden() || error.status() == Some(404),
        "the property read is refused by licence or reported missing, never untyped: {error}",
    );
}

/// The identity-creating write, proven through its error channel and never allowed to complete: a customer created
/// here would be a real account on the tenant.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_identity_creating_writes_without_ever_completing_one() {
    let error = service_desk()
        .customer()
        .create_customer(CustomerCreate {
            email: Some("not-an-email".to_owned()),
            display_name: Some(String::new()),
            ..CustomerCreate::default()
        })
        .send()
        .await
        .expect_err("an address that is not an address cannot become a customer");

    assert!(error.status().is_some_and(|status| status >= 400), "the refused write is typed: {error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_when_removing_an_organization_that_does_not_exist() {
    let error = service_desk()
        .organization()
        .delete_organization(99_999_999)
        .send()
        .await
        .expect_err("an organization that does not exist cannot be removed");

    assert!(
        error.is_forbidden() || error.status() == Some(404),
        "a refused delete is typed by licence or by absence: {error}",
    );
}
