use crate::model::{
    identifiers::IdentifierReference,
    values::{MappingValue, SimpleValue, ValueConstructor},
    HasSourceSpan, Span,
};
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Constraints ❱ Values
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PredicateValue {
    Simple(SimpleValue),
    Sequence(SequenceOfPredicateValues),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SequenceOfPredicateValues {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    values: Vec<PredicateSequenceMember>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PredicateSequenceMember {
    Simple(SimpleValue),
    ValueConstructor(ValueConstructor),
    Mapping(MappingValue),
    Reference(IdentifierReference),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ PredicateValue
// ------------------------------------------------------------------------------------------------

impl<T> From<T> for PredicateValue
where
    T: Into<SimpleValue>,
{
    fn from(v: T) -> Self {
        Self::Simple(v.into())
    }
}

impl From<SequenceOfPredicateValues> for PredicateValue {
    fn from(v: SequenceOfPredicateValues) -> Self {
        Self::Sequence(v)
    }
}

impl PredicateValue {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_simple(&self) -> bool {
        matches!(self, Self::Simple(_))
    }

    pub const fn as_simple(&self) -> Option<&SimpleValue> {
        match self {
            Self::Simple(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub const fn is_sequence(&self) -> bool {
        matches!(self, Self::Sequence(_))
    }

    pub const fn as_sequence(&self) -> Option<&SequenceOfPredicateValues> {
        match self {
            Self::Sequence(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ SequenceOfPredicateValues
// ------------------------------------------------------------------------------------------------

impl From<Vec<PredicateSequenceMember>> for SequenceOfPredicateValues {
    fn from(values: Vec<PredicateSequenceMember>) -> Self {
        Self { span: None, values }
    }
}

impl FromIterator<PredicateSequenceMember> for SequenceOfPredicateValues {
    fn from_iter<T: IntoIterator<Item = PredicateSequenceMember>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl Display for SequenceOfPredicateValues {
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

impl HasSourceSpan for SequenceOfPredicateValues {
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

impl SequenceOfPredicateValues {
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PredicateSequenceMember> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PredicateSequenceMember> {
        self.values.iter_mut()
    }

    pub fn push<I>(&mut self, value: I)
    where
        I: Into<PredicateSequenceMember>,
    {
        self.values.push(value.into())
    }

    pub fn extend<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = PredicateSequenceMember>,
    {
        self.values.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ PredicateSequenceMember
// ------------------------------------------------------------------------------------------------

impl<T> From<T> for PredicateSequenceMember
where
    T: Into<SimpleValue>,
{
    fn from(v: T) -> Self {
        Self::Simple(v.into())
    }
}

impl From<IdentifierReference> for PredicateSequenceMember {
    fn from(v: IdentifierReference) -> Self {
        Self::Reference(v)
    }
}

impl Display for PredicateSequenceMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PredicateSequenceMember::Simple(v) => v.to_string(),
                PredicateSequenceMember::ValueConstructor(v) => v.to_string(),
                PredicateSequenceMember::Mapping(v) => v.to_string(),
                PredicateSequenceMember::Reference(v) => v.to_string(),
            }
        )
    }
}

impl PredicateSequenceMember {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_simple(&self) -> bool {
        matches!(self, Self::Simple(_))
    }

    pub const fn as_simple(&self) -> Option<&SimpleValue> {
        match self {
            Self::Simple(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub const fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub const fn as_reference(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Reference(v) => Some(v),
            _ => None,
        }
    }
}
