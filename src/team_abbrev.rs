use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// Length of every NHL team abbreviation, current and historical (`"MTL"`,
/// `"MWN"`, `"UTA"`, ...).
const TEAM_ABBREV_LEN: usize = 3;

/// Error returned when a string is not a valid [`TeamAbbrev`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("invalid team abbreviation {0:?}: expected {TEAM_ABBREV_LEN} ASCII letters")]
pub struct TeamAbbrevError(String);

impl TeamAbbrevError {
    /// The rejected input.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// A validated NHL team abbreviation (e.g. `"MTL"`), safe to put in a URL path.
///
/// Holds exactly three ASCII letters, stored uppercase (the API's own form);
/// lowercase input is accepted and normalized. Anything else, including
/// path separators, query characters and non-ASCII text, is rejected, so a
/// caller-supplied string can never change the requested endpoint.
///
/// Client methods take `impl AsRef<str>` and validate it through this type, so
/// both `"MTL"` and a `TeamAbbrev` work as arguments. Serializes as its string;
/// deserializing validates the same way as [`TeamAbbrev::new`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TeamAbbrev(String);

impl TeamAbbrev {
    /// Validates `abbrev` and normalizes it to uppercase.
    ///
    /// # Errors
    /// Returns [`TeamAbbrevError`] unless `abbrev` is exactly three ASCII letters.
    pub fn new(abbrev: &str) -> Result<Self, TeamAbbrevError> {
        if abbrev.len() == TEAM_ABBREV_LEN && abbrev.bytes().all(|b| b.is_ascii_alphabetic()) {
            Ok(Self(abbrev.to_ascii_uppercase()))
        } else {
            Err(TeamAbbrevError(abbrev.to_string()))
        }
    }

    /// The abbreviation, uppercase.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TeamAbbrev {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TeamAbbrev {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for TeamAbbrev {
    type Err = TeamAbbrevError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for TeamAbbrev {
    type Error = TeamAbbrevError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for TeamAbbrev {
    type Error = TeamAbbrevError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl Serialize for TeamAbbrev {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for TeamAbbrev {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::new(&raw).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[path = "team_abbrev_test.rs"]
mod tests;
