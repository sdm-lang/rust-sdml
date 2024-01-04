use crate::{
    error::Error,
    model::{
        annotations::AnnotationOnlyBody, check::Validate, identifiers::Identifier, modules::Module,
        Span,
    },
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `rdf_thing_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum RdfDef {
    Class(RdfDefBody),
    Property(RdfDefBody),
}

/// Corresponds to the grammar rule `rdf_class_def` and `rdf_property_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RdfDefBody {
    span: Option<Span>,
    name: Identifier,
    body: AnnotationOnlyBody,
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

impl_has_name_for!(RdfDef => variants Class, Property);

impl_has_source_span_for!(RdfDef => variants Class, Property);

impl_references_for!(RdfDef => variants Class, Property);

impl Validate for RdfDef {
    fn is_complete(&self, _: &Module) -> Result<bool, Error> {
        // TODO: is this truly true?
        Ok(true)
    }

    fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
        match self {
            Self::Class(def) => def.is_valid_as_class(check_constraints, top),
            Self::Property(def) => def.is_valid_as_property(check_constraints, top),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_name_for!(RdfDefBody);

impl_has_body_for!(RdfDefBody, AnnotationOnlyBody);

impl_has_source_span_for!(RdfDefBody);

impl_references_for!(RdfDefBody => delegate body);

impl RdfDefBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: Default::default(),
        }
    }

    pub fn is_valid_as_class(
        &self,
        _check_constraints: bool,
        _top: &Module,
    ) -> Result<bool, Error> {
        todo!()
    }

    pub fn is_valid_as_property(
        &self,
        _check_constraints: bool,
        _top: &Module,
    ) -> Result<bool, Error> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
