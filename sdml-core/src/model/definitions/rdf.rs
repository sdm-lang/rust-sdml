use crate::{
    cache::ModuleCache,
    model::{
        annotations::{AnnotationOnlyBody, HasAnnotations, with_type},
        check::Validate,
        identifiers::{Identifier, QualifiedIdentifier},
        modules::Module,
        HasBody, Span,
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
        let mut new_self = Self::new(name);
        with_type(
            new_self.body_mut(),
            QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::CLASS_CLASS_NAME),
        ));
        new_self
    }

    pub fn datatype(name: Identifier) -> Self {
        let mut new_self = Self::new(name);
        with_type(
            new_self.body_mut(),
            QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::CLASS_DATATYPE_NAME),
            ));
        new_self
    }

    pub fn property(name: Identifier) -> Self {
        let mut new_self = Self::new(name);
        with_type(
            new_self.body_mut(),
            QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdf::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdf::CLASS_PROPERTY_NAME),
            ));
        new_self
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
