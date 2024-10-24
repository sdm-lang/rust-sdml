use crate::model::definitions::HasMembers;
use crate::model::definitions::SourceEntity;
use crate::model::{
    annotations::{Annotation, HasAnnotations},
    check::Validate,
    identifiers::{Identifier, IdentifierReference},
    members::Member,
    HasName, References, Span,
};
use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<EventBody>,
}

/// Corresponds to the grammar rule `event_body`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EventBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    event_source: SourceEntity,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    members: Vec<Member>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(EventDef);

impl_has_optional_body_for!(EventDef, EventBody);

impl_references_for!(EventDef => delegate optional body);

impl_has_source_span_for!(EventDef);

impl_maybe_incomplete_for!(EventDef);

impl Validate for EventDef {
    fn validate(
        &self,
        top: &crate::model::modules::Module,
        cache: &impl crate::store::ModuleStore,
        loader: &impl crate::load::ModuleLoader,
        check_constraints: bool,
    ) {
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

    pub fn new(name: Identifier) -> Self {
        Self {
            span: Default::default(),
            name,
            body: Default::default(),
        }
    }

    pub fn with_body(self, body: EventBody) -> Self {
        let mut self_mut = self;
        self_mut.body = Some(body);
        self_mut
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(EventBody);

impl_has_members_for!(EventBody);

impl_has_source_span_for!(EventBody);

impl_maybe_incomplete_for!(EventBody; over members);

// TODO: need to include event_source in validation!!
impl_validate_for_annotations_and_members!(EventBody);

impl References for EventBody {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_annotations(names));
    }
}

impl EventBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(event_source: SourceEntity) -> Self {
        Self {
            span: Default::default(),
            annotations: Default::default(),
            event_source,
            members: Default::default(),
        }
    }

    pub fn with_members<I>(self, members: I) -> Self
    where
        I: IntoIterator<Item = Member>,
    {
        let mut self_mut = self;
        self_mut.members = members.into_iter().collect();
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub event_source, set_event_source => SourceEntity);
}
