//! Ported from jira.js/tests/live/cloud/userProperties.test.ts.
//!
//! The entity-property mechanism again, this time hung off a user. Exercised in full against the authenticating
//! account under a namespaced key, and never against anyone else — writing a property onto another person's account
//! is the one variant of this API with a privacy dimension.

use serde_json::json;

use crate::harness::{ResourceTracker, cloud};

const PROPERTY_KEY: &str = "jira.rs.livetest.user";

fn object_of(value: &serde_json::Value) -> std::collections::HashMap<String, serde_json::Value> {
    value.as_object().expect("a property value is an object").clone().into_iter().collect()
}

/// The account id of whoever the credentials belong to.
async fn current_account_id() -> String {
    cloud()
        .myself()
        .get_current_user()
        .send()
        .await
        .expect("the site names the calling user")
        .account_id
        .expect("the calling user carries an account id")
}

/// `account_id` is optional in the types and mandatory in practice.
///
/// The specification declares the query parameter optional, so the generated setter is optional too and omitting it
/// compiles cleanly. It looks like it should mean "the calling user". Jira refuses the request outright instead,
/// naming a parameter the types said was unnecessary — a call that can never succeed and never fails to compile.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_a_property_write_that_omits_the_account_id() {
    let error = cloud()
        .user_properties()
        .set_user_property(PROPERTY_KEY, object_of(&json!({ "written": "no accountId" })))
        .send()
        .await
        .expect_err("a user property write without an account id is refused");

    assert_eq!(error.status(), Some(400), "{error}");
}

/// The user property round trip, end to end.
///
/// Proves that a named account takes a property, that the listing links to what it names, that a second write
/// replaces rather than merges, and that a deleted property is unreadable.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_user_property_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let account_id = current_account_id().await;

    cloud()
        .user_properties()
        .set_user_property(PROPERTY_KEY, object_of(&json!({ "written": "with an accountId" })))
        .account_id(&account_id)
        .send()
        .await
        .expect("the account takes a property when it is named");

    let owner = account_id.clone();

    tracker.defer(move || {
        let owner = owner.clone();

        async move { cloud().user_properties().delete_user_property(PROPERTY_KEY).account_id(owner).send().await }
    });

    let property = cloud()
        .user_properties()
        .get_user_property(PROPERTY_KEY)
        .account_id(&account_id)
        .send()
        .await
        .expect("the property reads back");

    assert_eq!(property.key.as_deref(), Some(PROPERTY_KEY));
    assert_eq!(property.value, Some(json!({ "written": "with an accountId" })));

    let listed = cloud()
        .user_properties()
        .get_user_property_keys()
        .account_id(&account_id)
        .send()
        .await
        .expect("the listing sees the stored property");

    let entry = listed
        .keys
        .unwrap_or_default()
        .into_iter()
        .find(|key| key.key.as_deref() == Some(PROPERTY_KEY))
        .expect("the stored key is listed among the account properties");

    assert!(
        entry.self_.as_deref().is_some_and(|url| url.starts_with("https://")),
        "a listed key links to where it can be read: {:?}",
        entry.self_,
    );

    cloud()
        .user_properties()
        .set_user_property(PROPERTY_KEY, object_of(&json!({ "only": "this" })))
        .account_id(&account_id)
        .send()
        .await
        .expect("the property can be written a second time");

    let replaced = cloud()
        .user_properties()
        .get_user_property(PROPERTY_KEY)
        .account_id(&account_id)
        .send()
        .await
        .expect("the rewritten property reads back");

    assert_eq!(replaced.value, Some(json!({ "only": "this" })), "a second write replaces the value, it does not merge");

    cloud()
        .user_properties()
        .delete_user_property(PROPERTY_KEY)
        .account_id(&account_id)
        .send()
        .await
        .expect("the property can be deleted");

    let error = cloud()
        .user_properties()
        .get_user_property(PROPERTY_KEY)
        .account_id(&account_id)
        .send()
        .await
        .expect_err("a deleted property cannot be read");

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_a_property_on_an_unknown_account_as_a_typed_error() {
    let error = cloud()
        .user_properties()
        .get_user_property(PROPERTY_KEY)
        .account_id("no-such-account-id")
        .send()
        .await
        .expect_err("an account that does not exist has no properties");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}
