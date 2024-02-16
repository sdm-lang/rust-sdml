use crate::{
    cache::ModuleCache,
    load::ModuleLoader,
    model::{
        annotations::{AnnotationBuilder, AnnotationOnlyBody, HasAnnotations},
        check::Validate,
        identifiers::{Identifier, QualifiedIdentifier},
        modules::Module,
        Span,
    },
    stdlib,
};
use tracing::info;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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

impl_maybe_invalid_for!(RdfDef; always false);

impl Validate for RdfDef {
    fn validate(
        &self,
        _top: &Module,
        _cache: &ModuleCache,
        _loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        info!("RdfDef is always valid.");
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

    pub fn is_class(&self) -> bool {
        self.body.has_rdf_type(&QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::CLASS),
        ).into())
    }

    pub fn datatype(name: Identifier) -> Self {
        Self::new(name).with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::DATATYPE),
        ))
    }

    pub fn is_datatype(&self) -> bool {
        self.body.has_rdf_type(&QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::DATATYPE),
        ).into())
    }

    pub fn property(name: Identifier) -> Self {
        Self::new(name).with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdf::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdf::PROPERTY),
        ))
    }

    pub fn is_property(&self) -> bool {
        self.body.has_rdf_type(&QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdf::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdf::PROPERTY),
        ).into())
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
