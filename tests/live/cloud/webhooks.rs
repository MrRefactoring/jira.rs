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
        Err(error) => {
            assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "the refusal is typed: {error}")
        }
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

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
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

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
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

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
}

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
