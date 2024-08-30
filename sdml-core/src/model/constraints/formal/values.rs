use crate::model::identifiers::IdentifierReference;
use crate::model::values::{MappingValue, SimpleValue, ValueConstructor};
use crate::model::Span;
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱ Terms ❱ Values
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
        Self::Sequence(v)
    }
}

impl PredicateValue {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Simple (SimpleValue) => is_simple, as_simple);

    is_as_variant!(Sequence (SequenceOfPredicateValues) => is_sequence, as_sequence);
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

impl_as_sequence!(pub SequenceOfPredicateValues => PredicateSequenceMember);

// ------------------------------------------------------------------------------------------------

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
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Simple (SimpleValue) => is_simple, as_simple);

    is_as_variant!(Reference (IdentifierReference) => is_reference, as_reference);
}
