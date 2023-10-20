use crate::error::Error;
use crate::model::{
    check::Validate, constraints::Constraint, identifiers::IdentifierReference, modules::Module,
    values::Value, HasNameReference, Span,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::References;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasAnnotations {
    fn has_annotations(&self) -> bool;

    fn annotations_len(&self) -> usize;

    fn annotations(&self) -> Box<dyn Iterator<Item = &Annotation> + '_>;

    fn annotations_mut(&mut self) -> Box<dyn Iterator<Item = &mut Annotation> + '_>;

    fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>;

    fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>;

    fn has_annotation_properties(&self) -> bool {
        self.annotations().any(|a| a.is_annotation_property())
    }

    fn annotation_properties(&self) -> Box<dyn Iterator<Item = &AnnotationProperty> + '_> {
        Box::new(
            self.annotations()
                .filter_map(|a| a.as_annotation_property()),
        )
    }

    fn has_constraints(&self) -> bool {
        self.annotations().any(|a| a.is_constraint())
    }

    fn annotation_constraints<I>(&self) -> Box<dyn Iterator<Item = &Constraint> + '_> {
        Box::new(self.annotations().filter_map(|a| a.as_constraint()))
    }
}

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
    name_reference: IdentifierReference,
    value: Value,
}

/// Corresponds to the grammar rule `annotation_only_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AnnotationOnlyBody {
    span: Option<Span>,
    annotations: Vec<Annotation>, // assert!(!annotations.is_empty());
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn skos_pref_label(element: &impl HasAnnotations) -> impl Iterator<Item = &AnnotationProperty> {
    element
        .annotation_properties()
        .filter(|ann| ann.name_reference() == "skos:prefLabel")
}

pub fn skos_alt_label(element: &impl HasAnnotations) -> impl Iterator<Item = &AnnotationProperty> {
    element
        .annotation_properties()
        .filter(|ann| ann.name_reference() == "skos:altLabel")
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations
// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(Annotation => variants Property, Constraint);

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

impl References for Annotation {}

impl Validate for Annotation {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        match self {
            Annotation::Property(v) => v.is_complete(top),
            Annotation::Constraint(v) => v.is_complete(top),
        }
    }

    fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
        match (self, check_constraints) {
            (Annotation::Property(v), _) => v.is_valid(check_constraints, top),
            (Annotation::Constraint(v), true) => v.is_valid(check_constraints, top),
            _ => Ok(true),
        }
    }
}

impl Annotation {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Property (AnnotationProperty) => is_annotation_property, as_annotation_property);

    is_as_variant!(Constraint (Constraint) => is_constraint, as_constraint);
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations ❱ Annotation Properties
// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(AnnotationProperty);

impl_has_name_reference_for!(AnnotationProperty);

impl Validate for AnnotationProperty {
    fn is_complete(&self, _top: &Module) -> Result<bool, Error> {
        Ok(true)
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        // TODO: ensure type/value conformance.
        Ok(true)
    }
}

impl AnnotationProperty {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name_reference: IdentifierReference, value: Value) -> Self {
        Self {
            span: None,
            name_reference,
            value,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub value, set_value => Value);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(AnnotationOnlyBody);

impl_has_annotations_for!(AnnotationOnlyBody);

impl From<Vec<Annotation>> for AnnotationOnlyBody {
    fn from(annotations: Vec<Annotation>) -> Self {
        Self {
            span: Default::default(),
            annotations,
        }
    }
}

impl From<AnnotationOnlyBody> for Vec<Annotation> {
    fn from(value: AnnotationOnlyBody) -> Self {
        value.annotations
    }
}

impl References for AnnotationOnlyBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        names.extend(self.annotation_properties().map(|ann| ann.name_reference()));
    }
}

impl Validate for AnnotationOnlyBody {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        let failed: Result<Vec<bool>, Error> =
            self.annotations().map(|ann| ann.is_complete(top)).collect();
        Ok(failed?.iter().all(|b| *b))
    }

    fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
        let failed: Result<Vec<bool>, Error> = self
            .annotations()
            .map(|ann| ann.is_valid(check_constraints, top))
            .collect();
        Ok(failed?.iter().all(|b| *b))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
