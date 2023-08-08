use super::Span;
use crate::error::invalid_identifier_error;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Named {
    fn name(&self) -> &Identifier;
    fn set_name(&mut self, name: &Identifier);
}

///
/// Corresponds the grammar rule `identifier`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Identifier {
    span: Option<Span>,
    value: String,
}

///
/// Corresponds the grammar rule `qualified_identifier`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct QualifiedIdentifier {
    span: Option<Span>,
    module: Identifier,
    member: Identifier,
}

///
/// Corresponds the grammar rule `identifier_reference`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum IdentifierReference {
    Identifier(Identifier),
    QualifiedIdentifier(QualifiedIdentifier),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref IDENTIFIER: Regex = Regex::new(r"^[\p{Lu}\p{Ll}]+(_[\p{Lu}\p{Ll}]+)*$").unwrap();
}

const RESERVED_KEYWORDS: [&str; 19] = [
    "as",
    "base",
    "datatype",
    "end",
    "entity",
    "enum",
    "event",
    "group",
    "identity",
    "import",
    "is",
    "module",
    "of",
    "property",
    "ref",
    "source",
    "structure",
    "union",
    "unknown",
];
const RESERVED_TYPES: [&str; 6] = ["string", "double", "decimal", "integer", "boolean", "iri"];
const RESERVED_MODULES: [&str; 6] = ["owl", "rdf", "rdfs", "sdml", "xml", "xsd"];

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for Identifier {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self {
                span: None,
                value: s.to_string(),
            })
        } else {
            Err(invalid_identifier_error(s))
        }
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.value
    }
}

impl From<&Identifier> for String {
    fn from(value: &Identifier) -> Self {
        value.value.clone()
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        self.value.as_str()
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Identifier {}

impl Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // ignore: self.span.hash(state);
        self.value.hash(state);
    }
}

impl Identifier {
    pub fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    #[inline(always)]
    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    #[inline(always)]
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    #[inline(always)]
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    #[inline(always)]
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    #[inline(always)]
    pub fn with_module(&self, module: Identifier) -> QualifiedIdentifier {
        QualifiedIdentifier::new(module, self.clone())
    }

    #[inline(always)]
    pub fn with_member(&self, member: Identifier) -> QualifiedIdentifier {
        QualifiedIdentifier::new(self.clone(), member)
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn is_valid(s: &str) -> bool {
        IDENTIFIER.is_match(s) && !Self::is_keyword(s) && !Self::is_type_name(s)
    }

    #[inline(always)]
    pub fn is_keyword(s: &str) -> bool {
        RESERVED_KEYWORDS.contains(&s)
    }

    #[inline(always)]
    pub fn is_type_name(s: &str) -> bool {
        RESERVED_TYPES.contains(&s)
    }

    #[inline(always)]
    pub fn is_reserved_module_name(s: &str) -> bool {
        RESERVED_MODULES.contains(&s)
    }

    #[inline(always)]
    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for QualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.module, self.member)
    }
}

impl From<QualifiedIdentifier> for String {
    fn from(value: QualifiedIdentifier) -> Self {
        String::from(&value)
    }
}

impl From<&QualifiedIdentifier> for String {
    fn from(value: &QualifiedIdentifier) -> Self {
        value.to_string()
    }
}

impl PartialEq for QualifiedIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module && self.member == other.member
    }
}

impl Eq for QualifiedIdentifier {}

impl Hash for QualifiedIdentifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // ignore: self.span.hash(state);
        self.module.hash(state);
        self.member.hash(state);
    }
}

impl QualifiedIdentifier {
    pub fn new(module: Identifier, member: Identifier) -> Self {
        Self {
            span: None,
            module,
            member,
        }
    }

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    #[inline(always)]
    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    #[inline(always)]
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    #[inline(always)]
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    #[inline(always)]
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    #[inline(always)]
    pub fn module(&self) -> &Identifier {
        &self.module
    }

    #[inline(always)]
    pub fn member(&self) -> &Identifier {
        &self.member
    }

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.module == other.module && self.member == other.member
    }
}

// ------------------------------------------------------------------------------------------------

enum_display_impl!(IdentifierReference => Identifier, QualifiedIdentifier);

impl_from_for_variant!(IdentifierReference, Identifier, Identifier);
impl_from_for_variant!(
    IdentifierReference,
    QualifiedIdentifier,
    QualifiedIdentifier
);

impl From<IdentifierReference> for String {
    fn from(value: IdentifierReference) -> Self {
        String::from(&value)
    }
}

impl From<&IdentifierReference> for String {
    fn from(value: &IdentifierReference) -> Self {
        match value {
            IdentifierReference::Identifier(v) => v.to_string(),
            IdentifierReference::QualifiedIdentifier(v) => v.to_string(),
        }
    }
}

impl PartialEq for IdentifierReference {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0.eq(r0),
            (Self::QualifiedIdentifier(l0), Self::QualifiedIdentifier(r0)) => l0.eq(r0),
            _ => false,
        }
    }
}

impl Eq for IdentifierReference {}

impl Hash for IdentifierReference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl IdentifierReference {
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            Self::Identifier(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_qualified_identifier(&self) -> bool {
        matches!(self, Self::QualifiedIdentifier(_))
    }
    pub fn as_qualified_identifier(&self) -> Option<&QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(v) => Some(v),
            _ => None,
        }
    }

    pub fn has_ts_span(&self) -> bool {
        match self {
            Self::Identifier(v) => v.has_ts_span(),
            Self::QualifiedIdentifier(v) => v.has_ts_span(),
        }
    }

    pub fn ts_span(&self) -> Option<&Span> {
        match self {
            Self::Identifier(v) => v.ts_span(),
            Self::QualifiedIdentifier(v) => v.ts_span(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn eq_with_span(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0.eq_with_span(r0),
            (Self::QualifiedIdentifier(l0), Self::QualifiedIdentifier(r0)) => l0.eq_with_span(r0),
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
