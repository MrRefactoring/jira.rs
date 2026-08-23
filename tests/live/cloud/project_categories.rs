use jira::cloud::ProjectCategory;

use crate::harness::{ResourceTracker, cloud, test_name};

/// A project category, from creation to removal.
///
/// A category is site-wide but inert: it is a label projects can be grouped by, it affects no permission and no
/// behaviour, and removing one leaves the projects that referenced it untouched. That combination makes it one of the
/// few pieces of site configuration a test can safely create. It is deliberately never attached to a project — that
/// would be a write against the project every other suite depends on.
///
/// Creation needs *Administer Jira*, so a token without it is expected to be refused typed rather than to fail the
/// run.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_project_category_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let name = test_name("category").replace(['[', ']'], "");

    let created = cloud()
        .project_categories()
        .create_project_category(ProjectCategory {
            name: Some(name.clone()),
            description: Some("created by the live suite".to_owned()),
            ..ProjectCategory::default()
        })
        .send()
        .await;

    let created = match created {
        Ok(category) => category,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            return;
        }
    };

    let category_id: i64 =
        created.id.expect("a created category carries an id").parse().expect("a category id is a number");

    tracker.defer(move || async move {
        cloud().project_categories().remove_project_category(category_id).send().await
    });

    assert!(category_id > 0, "an id identifies the category: {category_id}");

    let read = cloud()
        .project_categories()
        .get_project_category_by_id(category_id)
        .send()
        .await
        .expect("the category reads back by id");

    assert_eq!(read.name.as_deref(), Some(name.as_str()));
    assert_eq!(read.description.as_deref(), Some("created by the live suite"));
    assert!(read.self_.is_some_and(|link| link.starts_with("https://")), "a category carries an absolute self link");

    let all = cloud()
        .project_categories()
        .get_all_project_categories()
        .send()
        .await
        .expect("the site lists its categories");

    assert!(
        all.iter().any(|category| category.id.as_deref() == Some(category_id.to_string().as_str())),
        "the new category is among the site categories",
    );

    cloud()
        .project_categories()
        .update_project_category(
            category_id,
            ProjectCategory { description: Some("edited".to_owned()), ..ProjectCategory::default() },
        )
        .send()
        .await
        .expect("the description can be edited");

    let after_edit = cloud()
        .project_categories()
        .get_project_category_by_id(category_id)
        .send()
        .await
        .expect("the edited category reads back");

    assert_eq!(after_edit.description.as_deref(), Some("edited"), "the edit is observable on the next read");
    assert_eq!(after_edit.name.as_deref(), Some(name.as_str()), "editing the description leaves the name alone");

    let collision = cloud()
        .project_categories()
        .create_project_category(ProjectCategory { name: Some(name.clone()), ..ProjectCategory::default() })
        .send()
        .await
        .expect_err("two categories cannot share a name");

    assert!(collision.status().is_some_and(|status| status >= 400), "{collision}");

    let throwaway = cloud()
        .project_categories()
        .create_project_category(ProjectCategory {
            name: Some(format!("{name}-throwaway")),
            ..ProjectCategory::default()
        })
        .send()
        .await
        .expect("a second category can be created");

    let throwaway_id: i64 =
        throwaway.id.expect("a created category carries an id").parse().expect("a category id is a number");

    tracker.defer(move || async move {
        cloud().project_categories().remove_project_category(throwaway_id).send().await
    });

    cloud()
        .project_categories()
        .remove_project_category(throwaway_id)
        .send()
        .await
        .expect("the category can be removed");

    let gone = cloud()
        .project_categories()
        .get_project_category_by_id(throwaway_id)
        .send()
        .await
        .expect_err("a removed category cannot be read");

    assert!(gone.is_not_found(), "{gone}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_category_as_not_found() {
    let error = cloud()
        .project_categories()
        .get_project_category_by_id(99_999_999)
        .send()
        .await
        .expect_err("a category that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}
