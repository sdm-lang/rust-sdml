use crate::model::identifiers::IdentifierReference;
use crate::model::values::SimpleValue;
use crate::model::Span;
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
    List(SequenceOfPredicateValues),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SequenceOfPredicateValues {
    span: Option<Span>,
    values: Vec<PredicateSequenceMember>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PredicateSequenceMember {
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

impl From<SequenceOfPredicateValues> for PredicateValue {
    fn from(v: SequenceOfPredicateValues) -> Self {
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
    pub fn as_list(&self) -> Option<&SequenceOfPredicateValues> {
        match self {
            Self::List(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

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

impl_has_source_span_for!(SequenceOfPredicateValues);

impl SequenceOfPredicateValues {
    // --------------------------------------------------------------------------------------------

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

impl Display for PredicateSequenceMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PredicateSequenceMember::Simple(v) => v.to_string(),
                PredicateSequenceMember::Reference(v) => v.to_string(),
            }
        )
    }
}

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

impl PredicateSequenceMember {
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
