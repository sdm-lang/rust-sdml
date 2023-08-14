use crate::model::{Identifier, IdentifierReference, ModelElement, Span, StructureBody};
use std::{collections::HashSet, fmt::Debug};

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

impl ModelElement for EventDef {
    fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    fn name(&self) -> &Identifier {
        &self.name
    }
    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    // --------------------------------------------------------------------------------------------

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_types())
            .unwrap_or_default()
    }
}

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

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn event_source(&self) -> &IdentifierReference {
        &self.event_source
    }
    pub fn set_event_source(&mut self, event_source: IdentifierReference) {
        self.event_source = event_source;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    pub fn body(&self) -> Option<&StructureBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: StructureBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
