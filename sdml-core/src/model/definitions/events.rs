use crate::model::{
    definitions::StructureBody,
    identifiers::{Identifier, IdentifierReference},
    Span,
};
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
    span: Option<Span>,
    name: Identifier,
    event_source: IdentifierReference,
    body: Option<StructureBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(EventDef);

impl_has_optional_body_for!(EventDef, StructureBody);

impl_references_for!(EventDef => delegate optional body);

impl_has_source_span_for!(EventDef);

impl_validate_for!(EventDef => delegate optional body, false, true);

impl EventDef {
    pub fn new(name: Identifier, event_source: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            event_source,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn event_source(&self) -> &IdentifierReference {
        &self.event_source
    }

    pub fn set_event_source(&mut self, event_source: IdentifierReference) {
        self.event_source = event_source;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
