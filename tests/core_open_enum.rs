jira::open_enum! {
    /// How a project is administered.
    pub enum ProjectTypeKey {
        Software => "software",
        ServiceDesk => "service_desk",
        Business => "business",
    }
}

#[test]
fn accepts_a_documented_value() {
    assert_eq!(ProjectTypeKey::from("software"), ProjectTypeKey::Software);
    assert_eq!(ProjectTypeKey::Software.as_str(), "software");
}

#[test]
fn accepts_a_value_the_specification_never_mentioned() {
    let grown = ProjectTypeKey::from("product_discovery");

    assert_eq!(grown, ProjectTypeKey::Other("product_discovery".to_owned()));
    assert_eq!(grown.as_str(), "product_discovery");
}

#[test]
fn says_which_values_are_documented() {
    assert_eq!(ProjectTypeKey::documented(), &["software", "service_desk", "business"]);
    assert!(ProjectTypeKey::Software.is_documented());
    assert!(!ProjectTypeKey::from("whatever_comes_next").is_documented());
}

#[test]
fn deserializes_a_documented_value() {
    let parsed: ProjectTypeKey = serde_json::from_str("\"service_desk\"").unwrap();

    assert_eq!(parsed, ProjectTypeKey::ServiceDesk);
}

#[test]
fn deserializes_a_value_atlassian_added_since_rather_than_failing() {
    let parsed: ProjectTypeKey = serde_json::from_str("\"customer_service\"").unwrap();

    assert_eq!(parsed.as_str(), "customer_service");
}

#[test]
fn still_rejects_something_that_is_not_a_string() {
    let parsed = serde_json::from_str::<ProjectTypeKey>("42");

    assert!(parsed.is_err());
}

#[test]
fn round_trips_through_serialization() {
    for value in ["software", "service_desk", "business", "product_discovery"] {
        let parsed: ProjectTypeKey = value.into();

        assert_eq!(serde_json::to_string(&parsed).unwrap(), format!("\"{value}\""));
    }
}

#[test]
fn survives_the_wrappers_the_generated_types_put_around_it() {
    let optional: Option<ProjectTypeKey> = serde_json::from_str("null").unwrap();
    let list: Vec<ProjectTypeKey> = serde_json::from_str("[\"software\", \"grown\"]").unwrap();

    assert!(optional.is_none());
    assert_eq!(
        list,
        vec![ProjectTypeKey::Software, ProjectTypeKey::Other("grown".to_owned())]
    );
}

#[test]
fn displays_as_the_api_spells_it() {
    assert_eq!(ProjectTypeKey::ServiceDesk.to_string(), "service_desk");
    assert_eq!(ProjectTypeKey::from("grown".to_owned()).to_string(), "grown");
}

#[test]
fn goes_into_a_query_string_as_its_wire_value() {
    let value: jira::QueryValue = ProjectTypeKey::Business.into();

    assert_eq!(value, jira::QueryValue::Scalar("business".to_owned()));
}
