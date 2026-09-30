use jira::core::{Attachment, MultipartBody, mime_type_for};

#[test]
fn names_the_content_type_people_actually_attach() {
    assert_eq!(mime_type_for("screenshot.png"), "image/png");
    assert_eq!(mime_type_for("report.PDF"), "application/pdf");
    assert_eq!(mime_type_for("notes.md"), "text/markdown");
    assert_eq!(mime_type_for("archive.7z"), "application/x-7z-compressed");
}

#[test]
fn falls_back_to_octet_stream_for_an_extension_it_does_not_know() {
    assert_eq!(mime_type_for("firmware.blob"), "application/octet-stream");
    assert_eq!(mime_type_for("no-extension"), "application/octet-stream");
}

#[test]
fn a_leading_dot_is_not_an_extension() {
    assert_eq!(mime_type_for(".gitignore"), "application/octet-stream");
}

#[test]
fn an_attachment_guesses_its_content_type_from_its_filename() {
    let attachment = Attachment::new("screenshot.png", vec![1u8, 2, 3]);

    assert_eq!(attachment.content_type, None);
    assert_eq!(mime_type_for(&attachment.filename), "image/png");
}

#[test]
fn a_declared_content_type_wins_over_the_guess() {
    let attachment = Attachment::new("data.bin", vec![1u8]).with_content_type("application/x-custom");

    assert_eq!(attachment.content_type.as_deref(), Some("application/x-custom"));
}

#[test]
fn a_body_holds_several_attachments_under_one_field() {
    let body = MultipartBody::files(vec![Attachment::new("one.txt", "one"), Attachment::new("two.txt", "two")]);

    assert_eq!(body.field_name, "file");
    assert_eq!(body.attachments.len(), 2);
}

#[tokio::test]
async fn reads_a_file_and_names_the_attachment_after_it() {
    let path = std::env::temp_dir().join("jira-rs-attachment-test.txt");

    tokio::fs::write(&path, b"contents").await.unwrap();

    let attachment = Attachment::from_path(&path).await.unwrap();

    assert_eq!(attachment.filename, "jira-rs-attachment-test.txt");
    assert_eq!(attachment.content.as_ref(), b"contents");

    tokio::fs::remove_file(&path).await.unwrap();
}

#[test]
fn a_body_can_name_the_field_an_endpoint_reads_from() {
    let body = MultipartBody::new("attachment", vec![Attachment::new("a.txt", "a")]);

    assert_eq!(body.field_name, "attachment");
}
