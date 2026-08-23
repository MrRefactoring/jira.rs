//! The site's issue types, read-only but for one write.
//!
//! Issue types are site-wide configuration shared by every project on the tenant: creating one adds an option
//! everywhere, and deleting one asks Jira to migrate every issue that used it. Neither belongs in a suite that runs
//! against a working site, so that half is pinned through its error channel and aimed only at ids that cannot exist.
//!
//! The avatar upload is the exception, and it is exercised because it takes image bytes — the shape the specification
//! describes as an object of arbitrary keys, which is what made it unusable. It adds an avatar to the type's list of
//! available avatars and deletes it again; nothing selects it, so what the type displays never changes.

use jira::cloud::{DeleteAvatarRequestType, GetAvatarsRequestType, IssueTypeDetails};

use crate::harness::{ResourceTracker, TEST_ISSUE_TYPE, TEST_PROJECT_KEY, cloud};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn stores_an_avatar_for_an_issue_type_from_image_bytes() {
    let mut tracker = ResourceTracker::new();
    let types = site_issue_types().await;

    let issue_type = types
        .iter()
        .find(|candidate| candidate.subtask == Some(false))
        .or_else(|| types.first())
        .expect("a site has issue types");

    let type_id = issue_type.id.clone().expect("an issue type carries an id");

    let avatar = cloud()
        .issue_types()
        .create_issue_type_avatar(&type_id, 48, png_bytes(48))
        // Jira reads the declared media type rather than sniffing the bytes, and refuses an upload without one as
        // "not a supported image format".
        .content_type("image/png")
        .x(0)
        .y(0)
        .send()
        .await
        .expect("an issue type takes an avatar built from image bytes");

    assert!(!avatar.id.is_empty(), "a stored avatar carries an id");
    assert_eq!(avatar.is_system_avatar, Some(false), "an uploaded avatar is not one of Jira's own");

    let avatar_id = avatar.id.parse::<i64>().expect("an avatar id is a number");
    let owner = type_id.clone();

    tracker.defer(move || {
        let owner = owner.clone();

        async move {
            cloud().avatars().delete_avatar(DeleteAvatarRequestType::Issuetype, owner, avatar_id).send().await
        }
    });

    let avatars = cloud()
        .avatars()
        .get_avatars(GetAvatarsRequestType::Issuetype, &type_id)
        .send()
        .await
        .expect("an issue type lists the avatars available to it");

    assert!(
        avatars.custom.unwrap_or_default().iter().any(|candidate| candidate.id == avatar.id),
        "the upload is offered to the type as a custom avatar",
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_site_issue_types_each_fully_typed() {
    let types = site_issue_types().await;

    assert!(!types.is_empty(), "a Jira site always has issue types");

    for issue_type in &types {
        let id = issue_type.id.as_deref().expect("an issue type carries an id");

        assert!(id.chars().all(|character| character.is_ascii_digit()), "an id is digits: {id}");
        assert!(issue_type.name.as_deref().is_some_and(|name| !name.is_empty()), "{issue_type:?}");
        assert!(issue_type.subtask.is_some(), "an issue type says whether it is a subtask: {issue_type:?}");
        assert!(issue_type.self_.as_deref().is_some_and(|url| url.starts_with("https://")), "{issue_type:?}");
        assert!(issue_type.hierarchy_level.is_some(), "an issue type sits somewhere in the hierarchy: {issue_type:?}");
    }
}

/// Two fields describe the same fact, and a caller that trusts one has to be able to trust the other.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn marks_subtask_types_consistently_across_both_fields() {
    for issue_type in &site_issue_types().await {
        if issue_type.subtask == Some(true) {
            assert_eq!(issue_type.hierarchy_level, Some(-1), "a subtask type sits below the base level");
        }

        if issue_type.hierarchy_level == Some(-1) {
            assert_eq!(issue_type.subtask, Some(true), "a type below the base level is a subtask type");
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn includes_the_types_the_test_project_actually_offers() {
    let project =
        cloud().projects().get_project(TEST_PROJECT_KEY).send().await.expect("the test project reads back by key");

    let site_ids: Vec<String> = site_issue_types().await.into_iter().filter_map(|issue_type| issue_type.id).collect();
    let project_types = project.issue_types.unwrap_or_default();

    assert!(!project_types.is_empty(), "the test project offers issue types");

    for issue_type in &project_types {
        let id = issue_type.id.as_deref().expect("a project's issue type carries an id");

        assert!(site_ids.iter().any(|candidate| candidate == id), "a project type is one of the site types: {id}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_single_type_by_id_identical_to_its_listing_entry() {
    let sample = the_test_issue_type().await;
    let id = sample.id.clone().expect("an issue type carries an id");

    let fetched =
        cloud().issue_types().get_issue_type(&id).send().await.expect("a type from the listing reads back by id");

    assert_eq!(fetched.id, sample.id);
    assert_eq!(fetched.name, sample.name, "reading one type gives the same record the listing did");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_types_an_issue_could_be_changed_to() {
    let task = the_test_issue_type().await;
    let id = task.id.clone().expect("an issue type carries an id");

    let alternatives = cloud()
        .issue_types()
        .get_alternative_issue_types(&id)
        .send()
        .await
        .expect("a type lists what it could be replaced by");

    for alternative in &alternatives {
        let alternative_id = alternative.id.as_deref().expect("an alternative carries an id");

        assert_ne!(alternative_id, id, "a type is never offered as an alternative to itself");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_type_as_not_found() {
    let error = cloud()
        .issue_types()
        .get_issue_type("99999999")
        .send()
        .await
        .expect_err("an issue type that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}

/// The destructive path, proven through its error channel and never aimed at a type that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .issue_types()
        .delete_issue_type("99999999")
        .send()
        .await
        .expect_err("an issue type that does not exist cannot be deleted");

    assert!(error.is_not_found() || error.is_forbidden(), "a refused delete is typed: {error}");
}

async fn site_issue_types() -> Vec<IssueTypeDetails> {
    cloud().issue_types().get_issue_all_types().send().await.expect("the site lists its issue types")
}

/// The type the suite's fixtures use, or whatever the site leads with — either is a real type to address.
async fn the_test_issue_type() -> IssueTypeDetails {
    let types = site_issue_types().await;

    types
        .iter()
        .find(|issue_type| issue_type.name.as_deref() == Some(TEST_ISSUE_TYPE))
        .or_else(|| types.first())
        .cloned()
        .expect("a site has issue types")
}

/// A valid PNG of a solid colour, built here rather than committed as a fixture.
///
/// The avatar endpoints read the image: Jira answers "not a supported image format" to anything it cannot decode, so
/// a placeholder of arbitrary bytes proves nothing. Generating one keeps the suite free of a binary fixture, and
/// keeps the size a parameter — Jira refuses an avatar smaller than the crop it is asked for.
fn png_bytes(side: u32) -> Vec<u8> {
    let side_at = usize::try_from(side).expect("a side fits an index");

    // Every row is the filter byte the format demands and then one solid-colour pixel per column.
    let mut row = vec![0u8];

    row.extend_from_slice(&[200, 60, 60].repeat(side_at));

    let scanlines = row.repeat(side_at);
    let mut header = Vec::with_capacity(13);

    header.extend_from_slice(&side.to_be_bytes());
    header.extend_from_slice(&side.to_be_bytes());
    header.extend_from_slice(&[8, 2, 0, 0, 0]);

    let mut png = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];

    png.extend(png_chunk(b"IHDR", &header));
    png.extend(png_chunk(b"IDAT", &deflate_stored(&scanlines)));
    png.extend(png_chunk(b"IEND", &[]));

    png
}

fn png_chunk(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let length = u32::try_from(data.len()).expect("a chunk is smaller than four gigabytes");
    let mut framed = Vec::with_capacity(12 + data.len());

    framed.extend_from_slice(&length.to_be_bytes());
    framed.extend_from_slice(kind);
    framed.extend_from_slice(data);

    let crc = crc32(&framed[4..]);

    framed.extend_from_slice(&crc.to_be_bytes());

    framed
}

/// A zlib stream of uncompressed blocks: a valid deflate encoding, and the only one expressible without a compressor.
fn deflate_stored(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x78, 0x01];
    let mut blocks = data.chunks(0xFFFF).peekable();

    while let Some(block) = blocks.next() {
        let length = u16::try_from(block.len()).expect("a block is at most sixty-five kilobytes");

        out.push(u8::from(blocks.peek().is_none()));
        out.extend_from_slice(&length.to_le_bytes());
        out.extend_from_slice(&(!length).to_le_bytes());
        out.extend_from_slice(block);
    }

    out.extend_from_slice(&adler32(data).to_be_bytes());

    out
}

fn adler32(data: &[u8]) -> u32 {
    let (mut low, mut high) = (1u32, 0u32);

    for byte in data {
        low = (low + u32::from(*byte)) % 65521;
        high = (high + low) % 65521;
    }

    (high << 16) | low
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;

    for byte in data {
        crc ^= u32::from(*byte);

        for _ in 0..8 {
            crc = if crc & 1 == 1 { 0xEDB8_8320 ^ (crc >> 1) } else { crc >> 1 };
        }
    }

    !crc
}
