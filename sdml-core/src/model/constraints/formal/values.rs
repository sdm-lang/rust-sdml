use crate::model::{IdentifierReference, SimpleValue, Span};
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱ Terms ❱ Values
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PredicateValue {
    Simple(SimpleValue),
    List(PredicateValueList),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PredicateValueList {
    span: Option<Span>,
    values: Vec<PredicateListMember>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PredicateListMember {
    Simple(SimpleValue),
    Reference(IdentifierReference),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱ Terms ❱ Values
// ------------------------------------------------------------------------------------------------

impl<T> From<T> for PredicateValue
where
    T: Into<SimpleValue>,
{
    fn from(v: T) -> Self {
        Self::Simple(v.into())
    }
}

impl From<PredicateValueList> for PredicateValue {
    fn from(v: PredicateValueList) -> Self {
        Self::List(v)
    }
}

impl PredicateValue {
    pub fn is_simple(&self) -> bool {
        matches!(self, Self::Simple(_))
    }
    pub fn as_simple(&self) -> Option<&SimpleValue> {
        match self {
            Self::Simple(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }
    pub fn as_list(&self) -> Option<&PredicateValueList> {
        match self {
            Self::List(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for PredicateValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl From<Vec<PredicateListMember>> for PredicateValueList {
    fn from(values: Vec<PredicateListMember>) -> Self {
        Self { span: None, values }
    }
}

impl FromIterator<PredicateListMember> for PredicateValueList {
    fn from_iter<T: IntoIterator<Item = PredicateListMember>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl PredicateValueList {
    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &PredicateListMember> {
        self.values.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PredicateListMember> {
        self.values.iter_mut()
    }
    pub fn push<I>(&mut self, value: I)
    where
        I: Into<PredicateListMember>,
    {
        self.values.push(value.into())
    }
    pub fn extend<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = PredicateListMember>,
    {
        self.values.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for PredicateListMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PredicateListMember::Simple(v) => v.to_string(),
                PredicateListMember::Reference(v) => v.to_string(),
            }
        )
    }
}

impl<T> From<T> for PredicateListMember
where
    T: Into<SimpleValue>,
{
    fn from(v: T) -> Self {
        Self::Simple(v.into())
    }
}

impl From<IdentifierReference> for PredicateListMember {
    fn from(v: IdentifierReference) -> Self {
        Self::Reference(v)
    }
}

impl PredicateListMember {
    pub fn is_simple(&self) -> bool {
        matches!(self, Self::Simple(_))
    }
    pub fn as_simple(&self) -> Option<&SimpleValue> {
        match self {
            Self::Simple(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }
    pub fn as_reference(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Reference(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
