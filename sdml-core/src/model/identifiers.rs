use super::Span;
use crate::error::invalid_identifier_error;
use convert_case::{Case, Casing};
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
const RESERVED_MODULES: [&str; 8] = ["dc", "owl", "rdf", "rdfs", "sdml", "skos", "xml", "xsd"];

// ------------------------------------------------------------------------------------------------

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

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.value.as_str() == other
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Identifier {}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // ignore: self.span.hash(state);
        self.value.hash(state);
    }
}

impl_has_source_span_for!(Identifier);

impl Identifier {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
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
    // Helpers
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
    pub fn is_library_module_name(s: &str) -> bool {
        RESERVED_MODULES.contains(&s)
    }

    #[inline(always)]
    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value
    }

    #[inline(always)]
    pub fn to_type_label(&self) -> String {
        self.value.to_case(Case::Title)
    }

    #[inline(always)]
    pub fn to_variant_label(&self) -> String {
        self.to_type_label()
    }

    #[inline(always)]
    pub fn to_member_label(&self) -> String {
        self.value.to_case(Case::Lower)
    }

    #[inline(always)]
    pub fn to_module_label(&self) -> String {
        self.to_member_label()
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

impl PartialEq<str> for QualifiedIdentifier {
    fn eq(&self, other: &str) -> bool {
        self.to_string().as_str() == other
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

impl_has_source_span_for!(QualifiedIdentifier, span);

impl QualifiedIdentifier {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(module: Identifier, member: Identifier) -> Self {
        Self {
            span: None,
            module,
            member,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    getter!(pub module => Identifier);

    getter!(pub member => Identifier);

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.module == other.module && self.member == other.member
    }
}

// ------------------------------------------------------------------------------------------------

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

impl PartialEq<str> for IdentifierReference {
    fn eq(&self, other: &str) -> bool {
        self.to_string().as_str() == other
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

enum_display_impl!(IdentifierReference => Identifier, QualifiedIdentifier);

impl_from_for_variant!(IdentifierReference, Identifier, Identifier);
impl_from_for_variant!(
    IdentifierReference,
    QualifiedIdentifier,
    QualifiedIdentifier
);

impl_has_source_span_for!(IdentifierReference => variants Identifier, QualifiedIdentifier);

impl IdentifierReference {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Identifier (Identifier) => is_identifier, as_identifier);

    is_as_variant!(QualifiedIdentifier (QualifiedIdentifier) => is_qualified_identifier, as_qualified_identifier);

    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub fn module(&self) -> Option<&Identifier> {
        match self {
            IdentifierReference::Identifier(_) => None,
            IdentifierReference::QualifiedIdentifier(v) => Some(v.module()),
        }
    }

    pub fn member(&self) -> &Identifier {
        match self {
            IdentifierReference::Identifier(v) => v,
            IdentifierReference::QualifiedIdentifier(v) => v.member(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
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

#[cfg(test)]
mod tests {
    use super::Identifier;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_type_label() {
        assert_eq!("Foo", Identifier::new_unchecked("Foo").to_type_label());
        assert_eq!(
            "Foo Bar",
            Identifier::new_unchecked("FooBar").to_type_label()
        );
        assert_eq!(
            "Foo Bar Baz",
            Identifier::new_unchecked("FooBarBaz").to_type_label()
        );
        assert_eq!(
            "Foo Bar Baz",
            Identifier::new_unchecked("Foo_Bar_Baz").to_type_label()
        );
    }

    #[test]
    fn test_member_label() {
        assert_eq!("foo", Identifier::new_unchecked("Foo").to_member_label());
        assert_eq!(
            "foo bar",
            Identifier::new_unchecked("FooBar").to_member_label()
        );
        assert_eq!(
            "foo bar baz",
            Identifier::new_unchecked("FooBarBaz").to_member_label()
        );
        assert_eq!(
            "foo bar baz",
            Identifier::new_unchecked("Foo_Bar_Baz").to_member_label()
        );
    }
}
