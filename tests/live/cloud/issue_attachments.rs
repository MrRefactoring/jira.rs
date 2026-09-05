//! Ported from jira.js/tests/live/cloud/issueAttachments.test.ts.
//!
//! The most machinery-heavy path in the library: multipart encoding, the `X-Atlassian-Token` header, and a binary
//! response that must not be JSON-parsed. Unit tests can only prove the multipart bytes are well formed — whether
//! Jira accepts them is a question only a real site answers, which is the entire reason this file exists.
//!
//! Every content shape [`Attachment`] admits is uploaded here — borrowed text, owned bytes, and a declared media type
//! that overrides the one the filename implies — and each one's bytes are read back and compared to what went out.

use jira::core::Attachment;

use crate::harness::{
    ResourceTracker, await_readable, await_refused, cloud, create_test_issue, rendered_option, test_name,
};

/// Deliberately multibyte: a size measured in characters rather than bytes would not match.
const TEXT: &str = "attachment body — с кириллицей и эмодзи 🎯";

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_site_attachment_settings() {
    let settings = cloud().issue_attachments().get_attachment_meta().send().await.expect("the site reports settings");

    assert_eq!(settings.enabled, Some(true), "the test site has attachments enabled");
    assert!(
        settings.upload_limit.is_some_and(|limit| limit > 0),
        "an upload limit is a positive size: {:?}",
        settings.upload_limit,
    );
}

/// An attachment from upload to deletion.
///
/// Proves that each content shape survives the multipart round trip byte for byte, that a declared media type wins
/// over the one guessed from the filename, that several files go up in one request, that the issue itself lists what
/// was uploaded, and that a removed attachment is gone from the metadata endpoint too.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_an_attachment_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("attachments"))).await;

    let uploaded = cloud()
        .issue_attachments()
        .add_attachment(&issue.key, [Attachment::new("note.txt", TEXT)])
        .send()
        .await
        .expect("the issue takes a text attachment");

    let text_attachment = uploaded.into_iter().next().expect("one upload answers with one attachment");
    let attachment_id = text_attachment.id.clone().expect("a stored attachment carries an id");

    assert!(attachment_id.chars().all(|character| character.is_ascii_digit()), "an id is digits: {attachment_id}");
    assert_eq!(text_attachment.filename.as_deref(), Some("note.txt"));
    assert_eq!(
        text_attachment.size,
        Some(i64::try_from(TEXT.len()).expect("the fixture text fits an i64")),
        "the stored size is the UTF-8 byte length of what went up",
    );
    assert!(TEXT.len() > TEXT.chars().count(), "the fixture is multibyte, so bytes and characters cannot be confused");
    assert_eq!(text_attachment.mime_type.as_deref(), Some("text/plain"), "the type is guessed from the filename");

    let author = text_attachment.author.as_ref().and_then(|author| author.account_id.as_deref());

    assert!(author.is_some_and(|id| !id.is_empty()), "an upload is attributed to whoever made it: {author:?}");

    let content = cloud()
        .issue_attachments()
        .get_attachment_content(&attachment_id)
        .send()
        .await
        .expect("the attachment content reads back");

    assert_eq!(std::str::from_utf8(&content), Ok(TEXT), "the bytes come back unchanged");

    let raw = [0u8, 1, 2, 253, 254, 255];

    let binary = cloud()
        .issue_attachments()
        .add_attachment(&issue.key, [Attachment::new("bytes.bin", raw.to_vec())])
        .send()
        .await
        .expect("the issue takes a binary attachment");

    let binary = binary.into_iter().next().expect("one upload answers with one attachment");

    assert_eq!(binary.size, Some(i64::try_from(raw.len()).expect("six bytes fit an i64")));

    let returned = await_readable("the binary content reads back", || {
        cloud()
            .issue_attachments()
            .get_attachment_content(binary.id.clone().expect("a stored attachment carries an id"))
            .send()
    })
    .await;

    assert_eq!(returned.as_ref(), raw.as_slice(), "bytes outside the ASCII range survive intact");

    const PAYLOAD: &str = r#"{"from":"declared"}"#;

    let declared = cloud()
        .issue_attachments()
        .add_attachment(&issue.key, [Attachment::new("payload.json", PAYLOAD).with_content_type("application/json")])
        .send()
        .await
        .expect("the issue takes an attachment carrying its own media type");

    let declared = declared.into_iter().next().expect("one upload answers with one attachment");

    assert_eq!(declared.mime_type.as_deref(), Some("application/json"), "the declared media type reaches the site");
    assert_eq!(
        declared.size,
        Some(i64::try_from(PAYLOAD.len()).expect("the payload fits an i64")),
        "the declared type does not change the stored size",
    );

    let several = cloud()
        .issue_attachments()
        .add_attachment(&issue.key, [Attachment::new("one.txt", "first"), Attachment::new("two.txt", "second")])
        .send()
        .await
        .expect("several attachments go up in one request");

    let mut names = several.iter().filter_map(|attachment| attachment.filename.clone()).collect::<Vec<_>>();

    names.sort();

    assert_eq!(several.len(), 2, "one request, two attachments");
    assert_eq!(names, ["one.txt", "two.txt"]);

    let fetched = await_readable("the issue reads back with its attachments", || {
        cloud().issues().get_issue(&issue.key).fields(["attachment"]).send()
    })
    .await;

    let mut listed = fetched
        .fields
        .and_then(|fields| fields.attachment)
        .expect("the attachment field is a list")
        .into_iter()
        .filter_map(|attachment| attachment.filename)
        .collect::<Vec<_>>();

    listed.sort_unstable();

    assert_eq!(listed, ["bytes.bin", "note.txt", "one.txt", "payload.json", "two.txt"], "the issue lists every upload");

    let metadata = cloud()
        .issue_attachments()
        .get_attachment(&attachment_id)
        .send()
        .await
        .expect("a single attachment describes itself");

    assert_eq!(metadata.id, Some(attachment_id.parse().expect("an attachment id is a number")));
    assert_eq!(metadata.filename.as_deref(), Some("note.txt"));
    assert!(
        rendered_option(&metadata.created).is_some_and(|created| created.contains('T') && created.len() >= 20),
        "a stored attachment carries a timestamp: {:?}",
        metadata.created,
    );

    cloud().issue_attachments().remove_attachment(&attachment_id).send().await.expect("an attachment can be removed");

    let error = await_refused("a removed attachment cannot be described", || {
        cloud().issue_attachments().get_attachment(&attachment_id).send()
    })
    .await;

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_attachment_as_not_found() {
    let error = cloud()
        .issue_attachments()
        .get_attachment("99999999")
        .send()
        .await
        .expect_err("an attachment that does not exist cannot be described");

    assert!(error.is_not_found(), "{error}");
}
