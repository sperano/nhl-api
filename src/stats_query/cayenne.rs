//! Cayenne filter expressions, the query language of the `stats/rest`
//! endpoints (`cayenneExp` and `factCayenneExp`).
//!
//! Build an expression from a field name, then combine with
//! [`CayenneExpr::and`], [`CayenneExpr::or`] and `!`:
//!
//! ```
//! use nhl_api::CayenneExpr;
//!
//! let expr = CayenneExpr::field("points")
//!     .ge(100)
//!     .and(CayenneExpr::field("lastName").like("Mc%"));
//! assert_eq!(expr.to_string(), "points>=100 and lastName like 'Mc%'");
//! ```

use std::fmt;
use std::ops::Not;

use crate::date::Season;
use crate::ids::{FranchiseId, GameId, PlayerId, TeamId};
use crate::types::GameType;

/// A comparison operator between a field and a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    /// Case-sensitive SQL-style pattern match (`%` is the wildcard).
    Like,
    /// Case-insensitive pattern match.
    LikeIgnoreCase,
}

impl CompareOp {
    /// The operator as written in a Cayenne expression.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::Ne => "!=",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Like => " like ",
            Self::LikeIgnoreCase => " likeIgnoreCase ",
        }
    }
}

impl fmt::Display for CompareOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A literal in a Cayenne expression.
#[derive(Debug, Clone, PartialEq)]
pub enum CayenneValue {
    Int(i64),
    Float(f64),
    /// Rendered single-quoted, with `\` and `'` backslash-escaped.
    Str(String),
}

impl CayenneValue {
    /// Whether the value can be sent: every number except NaN and the
    /// infinities.
    pub fn is_finite(&self) -> bool {
        match self {
            Self::Float(value) => value.is_finite(),
            Self::Int(_) | Self::Str(_) => true,
        }
    }
}

impl fmt::Display for CayenneValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(value) => write!(f, "{value}"),
            Self::Float(value) => write!(f, "{value}"),
            Self::Str(value) => {
                f.write_str("'")?;
                for c in value.chars() {
                    if matches!(c, '\\' | '\'') {
                        f.write_str("\\")?;
                    }
                    write!(f, "{c}")?;
                }
                f.write_str("'")
            }
        }
    }
}

macro_rules! cayenne_value_from {
    ($variant:ident($target:ty): $($source:ty),+) => {
        $(
            impl From<$source> for CayenneValue {
                fn from(value: $source) -> Self {
                    Self::$variant(<$target>::from(value))
                }
            }
        )+
    };
}

cayenne_value_from!(Int(i64): i32, i64, u8, u16, u32);
cayenne_value_from!(Float(f64): f64);
cayenne_value_from!(Str(String): &str, String);

impl From<Season> for CayenneValue {
    fn from(season: Season) -> Self {
        Self::Int(i64::from(season.id()))
    }
}

impl From<GameType> for CayenneValue {
    fn from(game_type: GameType) -> Self {
        Self::Int(i64::from(game_type.to_int()))
    }
}

macro_rules! cayenne_value_from_id {
    ($($id:ty),+) => {
        $(
            impl From<$id> for CayenneValue {
                fn from(id: $id) -> Self {
                    Self::Int(id.as_i64())
                }
            }
        )+
    };
}

cayenne_value_from_id!(GameId, PlayerId, TeamId, FranchiseId);

/// A Cayenne filter expression.
///
/// Displays as the text sent in `cayenneExp`. `And` and `Or` render their
/// children joined by ` and ` / ` or `, parenthesising a child of the other
/// kind so precedence is explicit. An empty `And`/`Or` (and a `Not` of one)
/// renders as an empty string, and is skipped when it is a child of another
/// group, so it filters nothing.
#[derive(Debug, Clone, PartialEq)]
pub enum CayenneExpr {
    Compare {
        field: String,
        op: CompareOp,
        value: CayenneValue,
    },
    In {
        field: String,
        values: Vec<CayenneValue>,
    },
    Not(Box<CayenneExpr>),
    And(Vec<CayenneExpr>),
    Or(Vec<CayenneExpr>),
}

/// A field name awaiting a comparison; see [`CayenneExpr::field`].
#[derive(Debug, Clone)]
pub struct CayenneField(String);

impl CayenneField {
    fn compare(self, op: CompareOp, value: impl Into<CayenneValue>) -> CayenneExpr {
        CayenneExpr::Compare {
            field: self.0,
            op,
            value: value.into(),
        }
    }

    /// `field=value`
    pub fn eq(self, value: impl Into<CayenneValue>) -> CayenneExpr {
        self.compare(CompareOp::Eq, value)
    }

    /// `field!=value`
    pub fn ne(self, value: impl Into<CayenneValue>) -> CayenneExpr {
        self.compare(CompareOp::Ne, value)
    }

    /// `field<value`
    pub fn lt(self, value: impl Into<CayenneValue>) -> CayenneExpr {
        self.compare(CompareOp::Lt, value)
    }

    /// `field<=value`
    pub fn le(self, value: impl Into<CayenneValue>) -> CayenneExpr {
        self.compare(CompareOp::Le, value)
    }

    /// `field>value`
    pub fn gt(self, value: impl Into<CayenneValue>) -> CayenneExpr {
        self.compare(CompareOp::Gt, value)
    }

    /// `field>=value`
    pub fn ge(self, value: impl Into<CayenneValue>) -> CayenneExpr {
        self.compare(CompareOp::Ge, value)
    }

    /// `field like 'pattern'`
    pub fn like(self, pattern: impl Into<CayenneValue>) -> CayenneExpr {
        self.compare(CompareOp::Like, pattern)
    }

    /// `field likeIgnoreCase 'pattern'`
    pub fn like_ignore_case(self, pattern: impl Into<CayenneValue>) -> CayenneExpr {
        self.compare(CompareOp::LikeIgnoreCase, pattern)
    }

    /// `field in (a,b,c)`. An empty list is rejected by
    /// [`StatsQuery::validate`](super::StatsQuery::validate).
    pub fn is_in<V: Into<CayenneValue>>(self, values: impl IntoIterator<Item = V>) -> CayenneExpr {
        CayenneExpr::In {
            field: self.0,
            values: values.into_iter().map(Into::into).collect(),
        }
    }
}

impl CayenneExpr {
    /// Starts an expression on `name`, e.g. `CayenneExpr::field("goals").gt(50)`.
    pub fn field(name: impl Into<String>) -> CayenneField {
        CayenneField(name.into())
    }

    /// `self and other`, merged into one `And` when either side already is one.
    pub fn and(self, other: Self) -> Self {
        let mut terms = self.into_and_terms();
        terms.extend(other.into_and_terms());
        Self::And(terms)
    }

    /// `self or other`, merged into one `Or` when either side already is one.
    pub fn or(self, other: Self) -> Self {
        let mut terms = self.into_or_terms();
        terms.extend(other.into_or_terms());
        Self::Or(terms)
    }

    fn into_and_terms(self) -> Vec<Self> {
        match self {
            Self::And(terms) => terms,
            other => vec![other],
        }
    }

    fn into_or_terms(self) -> Vec<Self> {
        match self {
            Self::Or(terms) => terms,
            other => vec![other],
        }
    }

    /// Whether the expression renders as nothing (an empty group).
    fn is_empty(&self) -> bool {
        match self {
            Self::And(terms) | Self::Or(terms) => terms.iter().all(Self::is_empty),
            Self::Not(inner) => inner.is_empty(),
            Self::Compare { .. } | Self::In { .. } => false,
        }
    }

    /// Calls `visit` with every field name and every value in the expression.
    pub(super) fn visit_leaves(&self, visit: &mut impl FnMut(&str, &[CayenneValue])) {
        match self {
            Self::Compare { field, value, .. } => visit(field, std::slice::from_ref(value)),
            Self::In { field, values } => visit(field, values),
            Self::Not(inner) => inner.visit_leaves(visit),
            Self::And(terms) | Self::Or(terms) => {
                terms.iter().for_each(|term| term.visit_leaves(visit))
            }
        }
    }

    /// The field of the first empty `in ()` list in the expression, if any.
    pub(super) fn empty_in_field(&self) -> Option<&str> {
        match self {
            Self::In { field, values } if values.is_empty() => Some(field),
            Self::Not(inner) => inner.empty_in_field(),
            Self::And(terms) | Self::Or(terms) => terms.iter().find_map(Self::empty_in_field),
            Self::Compare { .. } | Self::In { .. } => None,
        }
    }
}

impl Not for CayenneExpr {
    type Output = Self;

    /// `not (self)`
    fn not(self) -> Self {
        Self::Not(Box::new(self))
    }
}

/// Writes `terms` joined by `separator`, skipping empty ones and wrapping
/// those for which `needs_parens` holds.
fn write_group(
    f: &mut fmt::Formatter<'_>,
    terms: &[CayenneExpr],
    separator: &str,
    needs_parens: fn(&CayenneExpr) -> bool,
) -> fmt::Result {
    let mut written = false;
    for term in terms.iter().filter(|term| !term.is_empty()) {
        if written {
            f.write_str(separator)?;
        }
        written = true;
        if needs_parens(term) {
            write!(f, "({term})")?;
        } else {
            write!(f, "{term}")?;
        }
    }
    Ok(())
}

fn is_multi_term(terms: &[CayenneExpr]) -> bool {
    terms.iter().filter(|term| !term.is_empty()).count() > 1
}

fn is_or_group(term: &CayenneExpr) -> bool {
    matches!(term, CayenneExpr::Or(terms) if is_multi_term(terms))
}

fn is_and_group(term: &CayenneExpr) -> bool {
    matches!(term, CayenneExpr::And(terms) if is_multi_term(terms))
}

/// Renders `terms` as one conjunction, as `cayenneExp` expects.
pub(super) fn render_conjunction(terms: &[CayenneExpr]) -> String {
    struct Conjunction<'a>(&'a [CayenneExpr]);

    impl fmt::Display for Conjunction<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write_group(f, self.0, " and ", is_or_group)
        }
    }

    Conjunction(terms).to_string()
}

impl fmt::Display for CayenneExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compare { field, op, value } => write!(f, "{field}{op}{value}"),
            Self::In { field, values } => {
                write!(f, "{field} in (")?;
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        f.write_str(",")?;
                    }
                    write!(f, "{value}")?;
                }
                f.write_str(")")
            }
            Self::Not(inner) if inner.is_empty() => Ok(()),
            Self::Not(inner) => write!(f, "not ({inner})"),
            Self::And(terms) => write_group(f, terms, " and ", is_or_group),
            Self::Or(terms) => write_group(f, terms, " or ", is_and_group),
        }
    }
}

#[cfg(test)]
#[path = "cayenne_test.rs"]
mod tests;
