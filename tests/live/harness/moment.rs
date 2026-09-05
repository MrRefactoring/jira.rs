/// A `date-time` field as the crate is currently built: text, or an instant under the `chrono` feature.
#[cfg(feature = "chrono")]
pub type Moment = chrono::DateTime<chrono::Utc>;
#[cfg(not(feature = "chrono"))]
pub type Moment = String;

/// A timestamp as text, whichever of the two the crate was built for.
///
/// The suites assert about timestamps in ways that only text supports — that one is later than another, that the
/// value really is an instant rather than a string of the right shape — and the `chrono` feature changes the type
/// those fields have. Rendering both to text is what lets one assertion cover both builds.
///
/// Ordering survives the rendering. Within one response Jira writes every timestamp at the same offset, so
/// lexicographic order is chronological order; rendering every instant at UTC keeps that true.
///
/// The spelling is Jira's own rather than RFC 3339, so a value rendered here can be handed back to a filter that
/// takes a timestamp — `from` on the audit log does — and reach the API in the form it came in.
#[cfg(feature = "chrono")]
pub fn rendered(value: &Moment) -> String {
    value.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string()
}

#[cfg(not(feature = "chrono"))]
pub fn rendered(value: &Moment) -> String {
    value.clone()
}

/// The same for a field the specification allows to be absent.
pub fn rendered_option(value: &Option<Moment>) -> Option<String> {
    value.as_ref().map(rendered)
}
