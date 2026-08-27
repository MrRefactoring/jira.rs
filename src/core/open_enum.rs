/// Declares an enum whose documented values are a vendor's list rather than a real constraint.
///
/// Open by default: any string round-trips through the extra `Other` variant, because the specification these types
/// are generated from routinely falls behind the API it describes. A value Atlassian added and did not write down is
/// not a bug in the caller's code, and it must not turn into a deserialization failure.
///
/// The documented values survive where they are useful — as real variants an editor suggests and a `match` covers.
///
/// ```
/// jira::open_enum! {
///     /// How a project is administered.
///     pub enum ProjectTypeKey {
///         Software => "software",
///         ServiceDesk => "service_desk",
///         Business => "business",
///     }
/// }
///
/// let known: ProjectTypeKey = "software".into();
/// let grown: ProjectTypeKey = "product_discovery".into();
///
/// assert_eq!(known, ProjectTypeKey::Software);
/// assert_eq!(grown.as_str(), "product_discovery");
/// ```
#[macro_export]
macro_rules! open_enum {
    (
        $(#[$meta:meta])*
        $visibility:vis enum $name:ident {
            $( $(#[$variant_meta:meta])* $variant:ident => $wire:literal ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[non_exhaustive]
        $visibility enum $name {
            $( $(#[$variant_meta])* $variant, )*
            /// A value the specification does not list. Atlassian grew the set; this is what it grew by.
            Other(::std::string::String),
        }

        impl $name {
            /// The value as the API spells it.
            pub fn as_str(&self) -> &str {
                match self {
                    $( $name::$variant => $wire, )*
                    $name::Other(value) => value.as_str(),
                }
            }

            /// The values the specification lists.
            pub fn documented() -> &'static [&'static str] {
                &[ $($wire),* ]
            }

            /// Whether this value is one the specification lists.
            pub fn is_documented(&self) -> bool {
                !matches!(self, $name::Other(_))
            }
        }

        impl ::std::convert::From<&str> for $name {
            fn from(value: &str) -> Self {
                match value {
                    $( $wire => $name::$variant, )*
                    other => $name::Other(other.to_owned()),
                }
            }
        }

        impl ::std::convert::From<::std::string::String> for $name {
            fn from(value: ::std::string::String) -> Self {
                $name::from(value.as_str())
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = ::std::convert::Infallible;

            fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
                ::std::result::Result::Ok($name::from(value))
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> ::std::result::Result<Self, D::Error> {
                let value = <::std::string::String as ::serde::Deserialize>::deserialize(deserializer)?;
                let parsed = $name::from(value);

                #[cfg(feature = "audit")]
                {
                    if let $name::Other(ref undocumented) = parsed {
                        $crate::core::audit::record_undocumented_value(
                            ::std::stringify!($name),
                            undocumented,
                            $name::documented(),
                        );
                    }
                }

                ::std::result::Result::Ok(parsed)
            }
        }

        impl ::std::convert::From<$name> for $crate::core::QueryValue {
            fn from(value: $name) -> Self {
                $crate::core::QueryValue::Scalar(value.as_str().to_owned())
            }
        }
    };
}
