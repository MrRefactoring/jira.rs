use std::collections::HashMap;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

/// A type that keeps the keys its schema does not describe, and reads and writes them as a type of the caller's.
///
/// Which custom fields an issue has is a property of the site rather than of the API, so the specification cannot
/// name them and the generated types carry them in an `additional` map, keyed as the site keys them. This is the
/// way to work with that map without touching `serde_json::Value` by hand: describe the fields as a struct of your
/// own, and read or write them through it.
///
/// ```
/// use jira::Extensible;
/// use jira::cloud::IssueFields;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Estimation {
///     #[serde(rename = "customfield_10016", skip_serializing_if = "Option::is_none")]
///     story_points: Option<f64>,
/// }
///
/// # fn example() -> Result<(), serde_json::Error> {
/// let fields = IssueFields { summary: Some("Rotate the signing key".into()), ..Default::default() }
///     .with_custom(Estimation { story_points: Some(5.0) })?;
///
/// let estimation: Estimation = fields.custom()?;
///
/// assert_eq!(estimation.story_points, Some(5.0));
/// assert_eq!(serde_json::to_value(&fields)?["customfield_10016"], 5.0);
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// A key the schema does describe is refused rather than written twice: `summary` is set on the struct, and
/// [`with`](Self::with) answers it with an error so the request cannot carry two of them.
pub trait Extensible: Sized {
    /// The keys the schema describes, spelt as they are on the wire.
    const FIELDS: &'static [&'static str];

    /// The keys the schema does not describe, as they arrived.
    fn additional(&self) -> &HashMap<String, Value>;

    /// The keys the schema does not describe, to change in place.
    fn additional_mut(&mut self) -> &mut HashMap<String, Value>;

    /// The undescribed keys, read as a type of the caller's.
    ///
    /// A field the type requires and the map does not hold is the error `serde_json` reports for a missing field,
    /// so a field that may be absent is best declared as an `Option`.
    fn custom<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        let entries = self.additional().iter().map(|(key, value)| (key.clone(), value.clone()));

        serde_json::from_value(Value::Object(Map::from_iter(entries)))
    }

    /// The fields of a type of the caller's, each added under its own key.
    ///
    /// The value has to serialize as a JSON object, and none of its keys may be one the schema describes.
    fn with_custom<T: Serialize>(mut self, custom: T) -> Result<Self, serde_json::Error> {
        let Value::Object(entries) = serde_json::to_value(custom)? else {
            return Err(serde::ser::Error::custom("custom fields have to serialize as a JSON object"));
        };

        for (key, value) in entries {
            self = self.with(key, value)?;
        }

        Ok(self)
    }

    /// One undescribed key, added or replaced.
    ///
    /// A key the schema describes is refused: it has a field of its own, and a body carrying the same key twice
    /// leaves Jira to pick one.
    fn with<T: Serialize>(mut self, key: impl Into<String>, value: T) -> Result<Self, serde_json::Error> {
        let key = key.into();

        if Self::FIELDS.contains(&key.as_str()) {
            return Err(serde::ser::Error::custom(format!(
                "`{key}` is a field the schema describes; set it on the struct rather than as a custom key"
            )));
        }

        self.additional_mut().insert(key, serde_json::to_value(value)?);

        Ok(self)
    }
}
