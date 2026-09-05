//! Ported from jira.js/tests/live/cloud/webhooks.test.ts.
//!
//! These are app-only endpoints: they operate on the webhooks registered by a Connect or Forge app, identified by the
//! app's own credentials. A user token has no app to speak for, so every one of them refuses — and refuses with a
//! status that says almost nothing about why.
//!
//! That refusal is the whole suite. It is worth pinning because "webhooks" is a feature people reach for early, and
//! the failure gives no hint that the problem is the *kind* of credential rather than its permissions. Registering a
//! webhook would in any case be a standing configuration change that outlives the run — so the one call that could
//! create something registers its removal before asserting anything.

use jira::cloud::{ContainerForWebhookIDs, WebhookDetails, WebhookDetailsEvents, WebhookRegistrationDetails};

use crate::harness::{ResourceTracker, cloud};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_to_list_webhooks_for_user_credentials() {
    let error = cloud()
        .webhooks()
        .get_dynamic_webhooks_for_app()
        .max_results(5)
        .send()
        .await
        .expect_err("a user token has no app whose webhooks could be listed");

    let status = error.status().expect("an app-only refusal carries a status");

    assert!((400..500).contains(&status), "the refusal is the caller's, not the site's: {error}");
}

/// The payload names a project that does not exist, and Jira never gets far enough to notice.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_registration_before_validating_the_payload() {
    let mut tracker = ResourceTracker::new();

    let registered = cloud()
        .webhooks()
        .register_dynamic_webhooks(WebhookRegistrationDetails {
            url: "https://example.com/hook".to_owned(),
            webhooks: vec![WebhookDetails {
                events: vec![WebhookDetailsEvents::JiraIssueCreated],
                jql_filter: "project = NOSUCHPROJECT".to_owned(),
                ..WebhookDetails::default()
            }],
        })
        .send()
        .await;

    match registered {
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "the refusal is typed: {error}"),
        // Not expected from a user token, but a webhook that exists fires at a URL, so it is removed before the
        // failure is reported rather than left behind by the panic.
        Ok(container) => {
            let ids: Vec<i64> = container
                .webhook_registration_result
                .unwrap_or_default()
                .iter()
                .filter_map(|registered| registered.created_webhook_id)
                .collect();

            tracker.defer(move || {
                let ids = ids.clone();

                async move {
                    cloud().webhooks().delete_webhook_by_id(ContainerForWebhookIDs { webhook_ids: ids }).send().await
                }
            });

            tracker.cleanup().await;

            panic!("a user token registered a webhook, which an app-only endpoint should have refused");
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_deletion_without_an_app_context() {
    let error = cloud()
        .webhooks()
        .delete_webhook_by_id(ContainerForWebhookIDs { webhook_ids: vec![99_999_999] })
        .send()
        .await
        .expect_err("a user token has no app whose webhooks it could delete");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_expiry_refresh_without_an_app_context() {
    let error = cloud()
        .webhooks()
        .refresh_webhooks(ContainerForWebhookIDs { webhook_ids: vec![99_999_999] })
        .send()
        .await
        .expect_err("a user token has no app whose webhooks could be kept alive");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_dynamic_module_reads_too() {
    let error = cloud()
        .dynamic_modules()
        .get_modules()
        .send()
        .await
        .expect_err("a user token has no app whose modules could be listed");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// Failing typed rather than hanging is the part the library owns; what the status means is Atlassian's business.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_module_removal_rather_than_hanging() {
    let error = cloud()
        .dynamic_modules()
        .remove_modules()
        .send()
        .await
        .expect_err("a user token has no app whose modules could be removed");

    assert!(error.status().is_some(), "the failure came back from the site rather than from the transport: {error}");
}
