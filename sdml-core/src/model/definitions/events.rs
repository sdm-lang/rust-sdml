use crate::model::{
    check::Validate,
    definitions::StructureBody,
    identifiers::{Identifier, IdentifierReference},
    HasName, Span,
};
use sdml_error::diagnostics::functions::IdentifierCaseConvention;
use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `event_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EventDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    event_source: IdentifierReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<StructureBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(EventDef);

impl_has_optional_body_for!(EventDef, StructureBody);

impl_references_for!(EventDef => delegate optional body);

impl_has_source_span_for!(EventDef);

impl_maybe_invalid_for!(EventDef);

impl Validate for EventDef {
    fn validate(
        &self,
        top: &crate::model::modules::Module,
        cache: &crate::cache::ModuleCache,
        loader: &impl crate::load::ModuleLoader,
        check_constraints: bool,
    ) {
        // TODO: need to include event_source in validation!!
        self.name()
            .validate(top, loader, Some(IdentifierCaseConvention::TypeDefinition));
        if let Some(body) = &self.body {
            body.validate(top, cache, loader, check_constraints);
        }
    }
}

impl_annotation_builder!(EventDef, optional body);

impl EventDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name: Identifier, event_source: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            event_source,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub event_source, set_event_source => IdentifierReference);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
