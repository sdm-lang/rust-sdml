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

    pub fn is_annotation_property(&self) -> bool {
        matches!(self, Self::Property(_))
    }

    pub fn as_annotation_property(&self) -> Option<&AnnotationProperty> {
        match self {
            Self::Property(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_constraint(&self) -> bool {
        matches!(self, Self::Constraint(_))
    }

    pub fn as_constraint(&self) -> Option<&Constraint> {
        match self {
            Self::Constraint(v) => Some(v),
            _ => None,
        }
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        // TODO: what about identifiers? or value constructors?
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

    pub fn name(&self) -> &IdentifierReference {
        &self.name
    }
    pub fn set_name(&mut self, name: IdentifierReference) {
        self.name = name;
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
