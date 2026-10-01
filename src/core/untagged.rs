#[cfg(feature = "audit")]
pub(crate) use super::audit::trial;

#[cfg(not(feature = "audit"))]
#[allow(dead_code)]
pub(crate) fn trial<T>(attempt: impl FnOnce() -> Option<T>) -> Option<T> {
    attempt()
}

#[allow(unused_macros)]
macro_rules! untagged {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
                let value = <::serde_json::Value as ::serde::Deserialize>::deserialize(deserializer)?;

                $(
                    if let ::std::option::Option::Some(parsed) = $crate::core::untagged::trial(|| {
                        ::serde::Deserialize::deserialize(&value).ok().map(Self::$variant)
                    }) {
                        return ::std::result::Result::Ok(parsed);
                    }
                )+

                ::std::result::Result::Err(<D::Error as ::serde::de::Error>::custom(::std::concat!(
                    "data did not match any variant of untagged enum ",
                    ::std::stringify!($name),
                )))
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use untagged;
