use super::Span;
use crate::error::invalid_identifier_error;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

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
pub struct Identifier {
    span: Option<Span>,
    value: String,
}

///
/// Corresponds the grammar rule `qualified_identifier`.
///
#[derive(Clone, Debug)]
pub struct QualifiedIdentifier {
    span: Option<Span>,
    module: Identifier,
    member: Identifier,
}

///
/// Corresponds the grammar rule `identifier_reference`.
///
#[derive(Clone, Debug)]
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

simple_display_impl!(Identifier, value);
as_str_impl!(Identifier, value);
into_string_impl!(Identifier, value);

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

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    pub fn with_module(&self, module: Identifier) -> QualifiedIdentifier {
        QualifiedIdentifier::new(module, self.clone())
    }

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

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get!(pub module => Identifier);
    get!(pub member => Identifier);

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.module == other.module && self.member == other.member
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Identifier> for IdentifierReference {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

impl From<QualifiedIdentifier> for IdentifierReference {
    fn from(v: QualifiedIdentifier) -> Self {
        Self::QualifiedIdentifier(v)
    }
}

enum_display_impl!(IdentifierReference => Identifier, QualifiedIdentifier);

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
    is_variant!(pub identifier => Identifier);
    is_variant!(pub qualified_identifier => QualifiedIdentifier);

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
