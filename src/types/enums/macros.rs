//! Declarative macro for NHL string-backed enums.
//!
//! `nhl_string_enum!` generates the uniform surface every NHL string enum needs:
//! a canonical API code, an optional human-readable name, `Display`, `FromStr`
//! (with parse aliases), and serde `Serialize`/`Deserialize` — all routed through
//! the canonical strings so the wire format is stable regardless of the Rust
//! variant names.
//!
//! This is the Rust-idiomatic replacement for the Go project's build-time
//! `internal/enumgen` codegen (a declarative macro rather than a generator, so
//! there is no CI drift guard to maintain).
//!
//! # Design decisions (2.2 must follow these)
//!
//! - **`code()` and `name()` are always generated.** Go treated the display name
//!   as optional; in Rust both accessors are zero-cost and every NHL enum has a
//!   canonical code and a human name, so a mandatory `name = "..."` per variant
//!   keeps the generated `name()` match exhaustive with a single macro arm.
//! - **`Display` picks one of the three Go modes** via the `display = ...` key:
//!   `code` (raw canonical string, e.g. `PeriodType` → `"REG"`), `name`, or
//!   `display_name`. `name` and `display_name` both render `name()`; Go's only
//!   difference between them was whether a separate `Name()` method existed, and
//!   in Rust `name()` is always present, so the two keywords are aliases kept for
//!   documentation intent.
//! - **`FromStr` matches the canonical string plus any declared aliases** and is
//!   the single validation entry point. serde `Deserialize` delegates to it, so
//!   aliases work identically on the wire. There is deliberately no `is_valid` —
//!   parsing *is* the validation (plan deviation #7: strict parsing stays).
//! - **Unknown values return the shared [`UnknownEnumValue`] error** from
//!   `FromStr`. serde cannot carry a typed error, so `Deserialize` surfaces it via
//!   [`serde::de::Error::custom`]; the type name and offending value are preserved
//!   in the message (`invalid <enum_name>: "<value>"`) but not as a recoverable
//!   typed error at the serde boundary.
//!
//! [`UnknownEnumValue`]: crate::types::enums::UnknownEnumValue

/// Generate an NHL string-backed enum with a uniform code/name/Display/FromStr/serde surface.
///
/// See the module docs for the design contract. Invocation shape:
///
/// ```ignore
/// nhl_string_enum! {
///     error_name = "period type",
///     display = code,
///     /// NHL period type
///     pub enum PeriodType {
///         /// Regulation period
///         Regulation = "REG", name = "Regulation";
///         /// Overtime period
///         Overtime = "OT", name = "Overtime";
///         /// Shootout
///         Shootout = "SO", name = "Shootout";
///     }
/// }
/// ```
///
/// A variant may declare parse aliases (accepted by `FromStr`/`Deserialize`, never
/// emitted by `Display`/`Serialize`):
///
/// ```ignore
/// OvertimeLoss = "O", name = "Overtime Loss", aliases = ["OTL"];
/// ```
macro_rules! nhl_string_enum {
    // Internal: resolve the Display source expression for a given mode keyword.
    (@display_value $value:ident, code) => {
        $value.code()
    };
    (@display_value $value:ident, name) => {
        $value.name()
    };
    (@display_value $value:ident, display_name) => {
        $value.name()
    };

    (
        error_name = $error_name:literal,
        display = $display_mode:ident,
        $(#[$enum_meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$var_meta:meta])*
                $variant:ident = $canonical:literal, name = $display_name:literal
                $(, aliases = [$($alias:literal),* $(,)?])?
            );* $(;)?
        }
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $name {
            $(
                $(#[$var_meta])*
                $variant,
            )*
        }

        impl $name {
            /// Returns the canonical API code for this value (the string used on the wire).
            pub const fn code(&self) -> &'static str {
                match self {
                    $( Self::$variant => $canonical, )*
                }
            }

            /// Returns the human-readable name for this value.
            pub const fn name(&self) -> &'static str {
                match self {
                    $( Self::$variant => $display_name, )*
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let value = self;
                ::std::write!(f, "{}", nhl_string_enum!(@display_value value, $display_mode))
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::types::enums::UnknownEnumValue;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match s {
                    $(
                        $canonical $( $(| $alias)* )? => ::core::result::Result::Ok(Self::$variant),
                    )*
                    _ => ::core::result::Result::Err($crate::types::enums::UnknownEnumValue {
                        enum_name: $error_name,
                        value: s.to_string(),
                    }),
                }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(self.code())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let s = <::std::string::String as ::serde::Deserialize>::deserialize(deserializer)?;
                <Self as ::std::str::FromStr>::from_str(&s).map_err(::serde::de::Error::custom)
            }
        }
    };
}

pub(crate) use nhl_string_enum;

#[cfg(test)]
mod tests {
    use super::nhl_string_enum;
    use crate::types::enums::UnknownEnumValue;
    use std::str::FromStr;

    nhl_string_enum! {
        error_name = "test decision",
        display = name,
        /// Test enum exercising aliases and the `name` Display mode.
        pub enum TestDecision {
            Win = "W", name = "Win";
            OvertimeLoss = "O", name = "Overtime Loss", aliases = ["OTL", "otl"];
        }
    }

    nhl_string_enum! {
        error_name = "test side",
        display = display_name,
        /// Test enum exercising the `display_name` Display mode (no aliases).
        pub enum TestSide {
            Left = "left", name = "Left Side";
            Right = "right", name = "Right Side";
        }
    }

    #[test]
    fn test_macro_code_and_name() {
        assert_eq!(TestDecision::Win.code(), "W");
        assert_eq!(TestDecision::Win.name(), "Win");
        assert_eq!(TestDecision::OvertimeLoss.code(), "O");
        assert_eq!(TestDecision::OvertimeLoss.name(), "Overtime Loss");
    }

    #[test]
    fn test_macro_display_name_mode() {
        assert_eq!(TestDecision::Win.to_string(), "Win");
        assert_eq!(TestDecision::OvertimeLoss.to_string(), "Overtime Loss");
        assert_eq!(TestSide::Left.to_string(), "Left Side");
        assert_eq!(TestSide::Right.to_string(), "Right Side");
    }

    #[test]
    fn test_macro_from_str_canonical() {
        assert_eq!(TestDecision::from_str("W").unwrap(), TestDecision::Win);
        assert_eq!(
            TestDecision::from_str("O").unwrap(),
            TestDecision::OvertimeLoss
        );
    }

    #[test]
    fn test_macro_from_str_alias() {
        assert_eq!(
            TestDecision::from_str("OTL").unwrap(),
            TestDecision::OvertimeLoss
        );
        assert_eq!(
            TestDecision::from_str("otl").unwrap(),
            TestDecision::OvertimeLoss
        );
    }

    #[test]
    fn test_macro_from_str_unknown_value() {
        let err = TestDecision::from_str("XYZ").unwrap_err();
        assert_eq!(
            err,
            UnknownEnumValue {
                enum_name: "test decision",
                value: "XYZ".to_string(),
            }
        );
    }

    #[test]
    fn test_macro_serialize_uses_canonical() {
        // Aliases are never emitted on serialize — only the canonical code.
        assert_eq!(
            serde_json::to_string(&TestDecision::OvertimeLoss).unwrap(),
            r#""O""#
        );
        assert_eq!(serde_json::to_string(&TestDecision::Win).unwrap(), r#""W""#);
    }

    #[test]
    fn test_macro_deserialize_canonical_and_alias() {
        assert_eq!(
            serde_json::from_str::<TestDecision>(r#""O""#).unwrap(),
            TestDecision::OvertimeLoss
        );
        // Aliases parse through serde as well.
        assert_eq!(
            serde_json::from_str::<TestDecision>(r#""OTL""#).unwrap(),
            TestDecision::OvertimeLoss
        );
    }

    #[test]
    fn test_macro_deserialize_unknown_error_message() {
        let err = serde_json::from_str::<TestDecision>(r#""XYZ""#).unwrap_err();
        let message = err.to_string();
        assert!(
            message.contains("test decision"),
            "message missing enum name: {message}"
        );
        assert!(
            message.contains("XYZ"),
            "message missing offending value: {message}"
        );
    }

    #[test]
    fn test_macro_serde_roundtrip() {
        for value in [TestDecision::Win, TestDecision::OvertimeLoss] {
            let serialized = serde_json::to_string(&value).unwrap();
            let deserialized: TestDecision = serde_json::from_str(&serialized).unwrap();
            assert_eq!(value, deserialized);
        }
        for value in [TestSide::Left, TestSide::Right] {
            let serialized = serde_json::to_string(&value).unwrap();
            let deserialized: TestSide = serde_json::from_str(&serialized).unwrap();
            assert_eq!(value, deserialized);
        }
    }
}
