use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// The shapes Jira writes a `date-time` in, in the order they are tried.
///
/// The first is the one Atlassian's own documentation gives and the one nearly every Cloud endpoint sends. It is not
/// RFC 3339 — that spelling wants `+00:00` and this one writes `+0000` — so a reader that only knows the standard
/// would reject the format the API actually uses. RFC 3339 is second because a handful of endpoints do write it, and
/// the offset-less form is third because the self-hosted products send it.
const LAYOUTS: &[&str] = &["%Y-%m-%dT%H:%M:%S%.f%z", "%Y-%m-%dT%H:%M:%S%.f%:z"];

/// A `date-time` as an instant, or nothing when it was written in a way this does not read.
///
/// Nothing rather than an error, for the same reason an open enum has an `Other` arm: the specification these types
/// come from falls behind the API it describes, and a format Atlassian starts sending should cost the one field it
/// is on rather than the whole response. Under the `audit` feature the miss is reported, so a new format is
/// something the suite can find rather than something a caller has to notice.
pub fn parse(value: &Value) -> Option<DateTime<Utc>> {
    match value {
        // The bulk queue answers `"created": 1787521555310` — epoch milliseconds, as a JSON integer, on a field the
        // document calls a string.
        Value::Number(number) => number.as_i64().and_then(|millis| Utc.timestamp_millis_opt(millis).single()),
        Value::String(text) => parse_text(text),
        _ => None,
    }
}

fn parse_text(text: &str) -> Option<DateTime<Utc>> {
    if let Ok(parsed) = DateTime::parse_from_rfc3339(text) {
        return Some(parsed.with_timezone(&Utc));
    }

    for layout in LAYOUTS {
        if let Ok(parsed) = DateTime::parse_from_str(text, layout) {
            return Some(parsed.with_timezone(&Utc));
        }
    }

    // An instant without an offset is read as UTC. Jira's self-hosted products write local time this way and say
    // nowhere which zone that is, so any other choice would be a guess dressed up as a conversion.
    if let Ok(naive) = NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M:%S%.f") {
        return Some(Utc.from_utc_datetime(&naive));
    }

    // A `date` is a `date-time` at midnight: `duedate` is declared one way and read alongside the others.
    NaiveDate::parse_from_str(text, "%Y-%m-%d")
        .ok()
        .and_then(|date| date.and_hms_opt(0, 0, 0))
        .map(|naive| Utc.from_utc_datetime(&naive))
}

/// Reads a `date-time` field, keeping a value it cannot read out of the way rather than failing the response.
pub fn deserialize_datetime<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error> {
    let Some(value) = Option::<Value>::deserialize(deserializer)? else {
        return Ok(None);
    };

    if value.is_null() {
        return Ok(None);
    }

    let parsed = parse(&value);

    #[cfg(feature = "audit")]
    if parsed.is_none() {
        crate::core::audit::record_unreadable_timestamp(&value);
    }

    Ok(parsed)
}

/// Writes a `date-time` back in the spelling Atlassian's documentation gives.
///
/// Not RFC 3339: an instant read from Jira and sent back unchanged has to reach it in the form it came in, and the
/// form it came in writes the offset without a colon.
pub fn serialize_datetime<S: Serializer>(value: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error> {
    match value {
        Some(instant) => instant.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string().serialize(serializer),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(text: &str) -> Option<DateTime<Utc>> {
        parse(&Value::String(text.to_owned()))
    }

    #[test]
    fn reads_the_format_atlassian_documents() {
        let parsed = at("2024-01-15T10:30:00.000+0000").expect("Jira's own spelling is readable");

        assert_eq!(parsed.to_rfc3339(), "2024-01-15T10:30:00+00:00");
    }

    #[test]
    fn reads_an_offset_that_is_not_utc() {
        let parsed = at("2024-01-15T10:30:00.000+0300").expect("an offset is readable");

        assert_eq!(parsed.to_rfc3339(), "2024-01-15T07:30:00+00:00");
    }

    #[test]
    fn reads_rfc_3339_as_well() {
        assert_eq!(at("2024-01-15T10:30:00Z").unwrap().to_rfc3339(), "2024-01-15T10:30:00+00:00");
        assert_eq!(at("2024-01-15T10:30:00.123456+03:00").unwrap().to_rfc3339(), "2024-01-15T07:30:00.123456+00:00");
    }

    #[test]
    fn reads_an_instant_without_an_offset_as_utc() {
        assert_eq!(at("2024-01-15T10:30:00").unwrap().to_rfc3339(), "2024-01-15T10:30:00+00:00");
    }

    #[test]
    fn reads_a_bare_date_as_midnight() {
        assert_eq!(at("2024-01-15").unwrap().to_rfc3339(), "2024-01-15T00:00:00+00:00");
    }

    #[test]
    fn reads_the_epoch_milliseconds_the_bulk_queue_sends() {
        let parsed = parse(&Value::Number(1_787_521_555_310_i64.into())).expect("a number is a timestamp too");

        assert_eq!(parsed.timestamp_millis(), 1_787_521_555_310);
    }

    #[test]
    fn keeps_a_shape_it_cannot_read_out_of_the_way() {
        assert!(at("sometime last tuesday").is_none());
        assert!(at("").is_none());
        assert!(parse(&Value::Bool(true)).is_none());
    }

    #[test]
    fn writes_back_the_spelling_it_read() {
        let parsed = at("2024-01-15T10:30:00.000+0000");
        let written = serde_json::to_value(Wrapper { at: parsed }).expect("an instant is serializable");

        assert_eq!(written["at"], "2024-01-15T10:30:00.000+0000");
    }

    #[derive(Serialize)]
    struct Wrapper {
        #[serde(serialize_with = "serialize_datetime")]
        at: Option<DateTime<Utc>>,
    }
}
