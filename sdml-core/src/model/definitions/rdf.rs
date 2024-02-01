use crate::{
    cache::ModuleCache,
    model::{
        annotations::{AnnotationBuilder, AnnotationOnlyBody, HasAnnotations},
        check::Validate,
        identifiers::{Identifier, QualifiedIdentifier},
        modules::Module,
        Span,
    },
    stdlib,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tracing::info;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `rdf_class_def` and `rdf_property_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RdfDef {
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

impl_has_name_for!(RdfDef);

impl_has_body_for!(RdfDef, AnnotationOnlyBody);

impl_has_source_span_for!(RdfDef);

impl_references_for!(RdfDef => delegate body);

impl_annotation_builder!(RdfDef);

impl Validate for RdfDef {
    fn is_complete(&self, _: &Module, _: &ModuleCache) -> Result<bool, crate::error::Error> {
        info!("RdfDef::is_complete true by definition");
        Ok(true)
    }

    fn is_valid(&self, _: bool, _: &Module, _: &ModuleCache) -> Result<bool, crate::error::Error> {
        info!("RdfDef::is_valid true-enough by definition");
        Ok(true)
    }
}

impl RdfDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: Default::default(),
        }
    }

    pub fn class(name: Identifier) -> Self {
        Self::new(name).with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::CLASS),
        ))
    }

    pub fn datatype(name: Identifier) -> Self {
        Self::new(name).with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::DATATYPE),
        ))
    }

    pub fn property(name: Identifier) -> Self {
        Self::new(name).with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdf::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdf::PROPERTY),
        ))
    }

    pub fn individual(name: Identifier) -> Self {
        Self::new(name)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
