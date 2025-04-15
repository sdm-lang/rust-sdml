/*!
Provide the Rust types that implement *identifier*-related components of the SDML Grammar.
*/
use crate::config::{is_builtin_type_name_str, is_library_module_str};
use crate::load::ModuleLoader;
use crate::model::modules::Module;
use crate::model::{HasSourceSpan, Span};
use crate::syntax::{
    PC_QUALIFIED_IDENTIFIER_SEPARATOR, RESERVED_CONSTRAINT_KEYWORDS, RESERVED_KEYWORDS,
};
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use regex::Regex;
use sdml_errors::diagnostics::functions::{
    identifier_not_preferred_case, invalid_identifier, IdentifierCaseConvention,
};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};
use tracing::error;

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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    value: String,
}

///
/// Corresponds the grammar rule `qualified_identifier`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct QualifiedIdentifier {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref IDENTIFIER: Regex =
        Regex::new(r"^[\p{Lu}\p{Ll}][\p{Lu}\p{Ll}\p{Nd}]*(?:_+[\p{Lu}\p{Ll}\p{Nd}]+)*$").unwrap();
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Identifier
// ------------------------------------------------------------------------------------------------

impl From<&Identifier> for String {
    fn from(value: &Identifier) -> Self {
        value.value.clone()
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.value
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
            error!("Identifier::from_str({s}) is invalid");
            Err(invalid_identifier(0, None, s).into())
        }
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
        Some(self.cmp(other))
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // ignore: self.span.hash(state);
        self.value.hash(state);
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl HasSourceSpan for Identifier {
    fn with_source_span(self, span: Span) -> Self {
        let mut self_mut = self;
        self_mut.span = Some(span);
        self_mut
    }

    fn source_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn set_source_span(&mut self, span: Span) {
        self.span = Some(span);
    }

    fn unset_source_span(&mut self) {
        self.span = None;
    }
}

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

    pub fn validate(
        &self,
        top: &Module,
        loader: &impl ModuleLoader,
        as_case: Option<IdentifierCaseConvention>,
    ) {
        if !Self::is_valid(&self.value) {
            loader
                .report(&invalid_identifier(
                    top.file_id().copied().unwrap_or_default(),
                    self.span.clone().map(|s| s.into()),
                    &self.value,
                ))
                .unwrap();
        }
        if let Some(case) = as_case {
            if !case.is_valid(self) {
                loader
                    .report(&identifier_not_preferred_case(
                        top.file_id().copied().unwrap_or_default(),
                        self.source_span().map(|span| span.byte_range()),
                        self,
                        case,
                    ))
                    .unwrap();
            }
        }
    }

    #[inline(always)]
    pub fn is_valid<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        let s = s.as_ref();
        IDENTIFIER.is_match(s) && !Self::is_keyword(s)
    }

    #[inline(always)]
    pub fn is_keyword<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        RESERVED_KEYWORDS.contains(&s.as_ref())
    }

    #[inline(always)]
    pub fn is_keyword_in_constraint<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        Self::is_keyword(&s) || RESERVED_CONSTRAINT_KEYWORDS.contains(&s.as_ref())
    }

    #[inline(always)]
    pub fn is_type_name<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        is_builtin_type_name_str(s.as_ref())
    }

    #[inline(always)]
    pub fn is_library_module_name<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        is_library_module_str(s.as_ref())
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
// Implementations ❱ QualifiedIdentifier
// ------------------------------------------------------------------------------------------------

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

impl From<(Identifier, Identifier)> for QualifiedIdentifier {
    fn from(value: (Identifier, Identifier)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl FromStr for QualifiedIdentifier {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(PC_QUALIFIED_IDENTIFIER_SEPARATOR)
            .collect::<Vec<&str>>();
        if parts.len() == 2 {
            Ok(Self::new(
                Identifier::from_str(parts[0])?,
                Identifier::from_str(parts[1])?,
            ))
        } else {
            error!("QualifiedIdentifier::from_str({s:?}) is invalid");
            Err(invalid_identifier(0, None, s).into())
        }
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

impl PartialOrd for QualifiedIdentifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QualifiedIdentifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.module.as_ref().cmp(other.module.as_ref()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.member.as_ref().cmp(other.member.as_ref())
    }
}

impl Display for QualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.module, PC_QUALIFIED_IDENTIFIER_SEPARATOR, self.member
        )
    }
}

impl HasSourceSpan for QualifiedIdentifier {
    fn with_source_span(self, span: Span) -> Self {
        let mut self_mut = self;
        self_mut.span = Some(span);
        self_mut
    }
    fn source_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_source_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_source_span(&mut self) {
        self.span = None;
    }
}

impl QualifiedIdentifier {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new_unchecked(module: &str, member: &str) -> Self {
        Self {
            span: None,
            module: Identifier::new_unchecked(module),
            member: Identifier::new_unchecked(member),
        }
    }

    pub const fn new(module: Identifier, member: Identifier) -> Self {
        Self {
            span: None,
            module,
            member,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn module(&self) -> &Identifier {
        &self.module
    }
    pub const fn member(&self) -> &Identifier {
        &self.member
    }
    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn validate(&self, top: &Module, loader: &impl ModuleLoader) {
        self.module
            .validate(top, loader, Some(IdentifierCaseConvention::Module));
        self.member
            .validate(top, loader, Some(IdentifierCaseConvention::ImportedMember));
    }

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.module == other.module && self.member == other.member
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ IdentifierReference
// ------------------------------------------------------------------------------------------------

impl From<&Identifier> for IdentifierReference {
    fn from(v: &Identifier) -> Self {
        Self::Identifier(v.clone())
    }
}

impl From<Identifier> for IdentifierReference {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

impl From<&QualifiedIdentifier> for IdentifierReference {
    fn from(v: &QualifiedIdentifier) -> Self {
        Self::QualifiedIdentifier(v.clone())
    }
}

impl From<QualifiedIdentifier> for IdentifierReference {
    fn from(v: QualifiedIdentifier) -> Self {
        Self::QualifiedIdentifier(v)
    }
}

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

impl FromStr for IdentifierReference {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(PC_QUALIFIED_IDENTIFIER_SEPARATOR)
            .collect::<Vec<&str>>();
        if parts.len() == 1 {
            Ok(Self::Identifier(Identifier::from_str(parts[0])?))
        } else if parts.len() == 2 {
            Ok(Self::QualifiedIdentifier(QualifiedIdentifier::new(
                Identifier::from_str(parts[0])?,
                Identifier::from_str(parts[1])?,
            )))
        } else {
            error!("QualifiedIdentifier::from_str({s:?}) is invalid");
            Err(invalid_identifier(0, None, s).into())
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

impl PartialOrd for IdentifierReference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IdentifierReference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl std::fmt::Display for IdentifierReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Identifier(v) => v.to_string(),
                Self::QualifiedIdentifier(v) => v.to_string(),
            }
        )
    }
}

impl HasSourceSpan for IdentifierReference {
    #[inline]
    fn with_source_span(self, span: Span) -> Self {
        match self {
            Self::Identifier(v) => Self::Identifier(v.with_source_span(span)),
            Self::QualifiedIdentifier(v) => Self::QualifiedIdentifier(v.with_source_span(span)),
        }
    }
    #[inline]
    fn source_span(&self) -> Option<&Span> {
        match self {
            Self::Identifier(v) => v.source_span(),
            Self::QualifiedIdentifier(v) => v.source_span(),
        }
    }
    #[inline]
    fn set_source_span(&mut self, span: Span) {
        match self {
            Self::Identifier(v) => v.set_source_span(span),
            Self::QualifiedIdentifier(v) => v.set_source_span(span),
        }
    }
    #[inline]
    fn unset_source_span(&mut self) {
        match self {
            Self::Identifier(v) => v.unset_source_span(),
            Self::QualifiedIdentifier(v) => v.unset_source_span(),
        }
    }
}

impl IdentifierReference {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }

    pub const fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            Self::Identifier(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_qualified_identifier(&self) -> bool {
        matches!(self, Self::QualifiedIdentifier(_))
    }

    pub const fn as_qualified_identifier(&self) -> Option<&QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn module(&self) -> Option<&Identifier> {
        match self {
            IdentifierReference::Identifier(_) => None,
            IdentifierReference::QualifiedIdentifier(v) => Some(v.module()),
        }
    }

    pub const fn member(&self) -> &Identifier {
        match self {
            IdentifierReference::Identifier(v) => v,
            IdentifierReference::QualifiedIdentifier(v) => v.member(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn validate(&self, top: &Module, loader: &impl ModuleLoader) {
        match self {
            IdentifierReference::Identifier(v) => v.validate(top, loader, None),
            IdentifierReference::QualifiedIdentifier(v) => v.validate(top, loader),
        };
    }

    pub fn eq_with_span(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0.eq_with_span(r0),
            (Self::QualifiedIdentifier(l0), Self::QualifiedIdentifier(r0)) => l0.eq_with_span(r0),
            _ => false,
        }
    }
}

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
