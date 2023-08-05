use super::{Constraint, IdentifierReference, Span, Value};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `annotation`.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)] // TODO: why is this reported as an issue?
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Annotation {
    Property(AnnotationProperty),
    Constraint(Constraint),
}

/// Corresponds to the grammar rule `annotation_property`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AnnotationProperty {
    span: Option<Span>,
    name: IdentifierReference,
    value: Value,
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
// Implementations ❱ Annotations
// ------------------------------------------------------------------------------------------------

impl From<AnnotationProperty> for Annotation {
    fn from(value: AnnotationProperty) -> Self {
        Self::Property(value)
    }
}

impl From<Constraint> for Annotation {
    fn from(value: Constraint) -> Self {
        Self::Constraint(value)
    }
}

impl Annotation {
    is_as_variant!(pub annotation_property => Property, AnnotationProperty);
    is_as_variant!(pub constraint => Constraint, Constraint);

    pub fn has_ts_span(&self) -> bool {
        match self {
            Annotation::Property(v) => v.has_ts_span(),
            Annotation::Constraint(v) => v.has_ts_span(),
        }
    }

    pub fn ts_span(&self) -> Option<&Span> {
        match self {
            Annotation::Property(v) => v.ts_span(),
            Annotation::Constraint(v) => v.ts_span(),
        }
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        Default::default()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations ❱ Annotation Properties
// ------------------------------------------------------------------------------------------------

impl AnnotationProperty {
    pub fn new(name: IdentifierReference, value: Value) -> Self {
        Self {
            span: None,
            name,
            value,
        }
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate!(pub name => IdentifierReference);

    get_and_mutate!(pub value => Value);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
