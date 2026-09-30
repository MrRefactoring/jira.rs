#[cfg(feature = "chrono")]
pub type Moment = chrono::DateTime<chrono::Utc>;
#[cfg(not(feature = "chrono"))]
pub type Moment = String;

#[cfg(feature = "chrono")]
pub fn rendered(value: &Moment) -> String {
    value.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string()
}

#[cfg(not(feature = "chrono"))]
pub fn rendered(value: &Moment) -> String {
    value.clone()
}

pub fn rendered_option(value: &Option<Moment>) -> Option<String> {
    value.as_ref().map(rendered)
}
