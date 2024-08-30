use crate::load::ModuleLoader;
use crate::model::References;
use crate::model::{
    annotations::Annotation,
    check::Validate,
    definitions::HasMembers,
    identifiers::{Identifier, IdentifierReference},
    members::Member,
    modules::Module,
    Span,
};
use crate::store::ModuleStore;
use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `entity_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<EntityBody>,
}

/// Corresponds to the grammar rule `entity_body`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    identity: Member,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    members: Vec<Member>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(EntityDef);

impl_has_optional_body_for!(EntityDef, EntityBody);

impl_has_source_span_for!(EntityDef);

impl_references_for!(EntityDef => delegate optional body);

impl_annotation_builder!(EntityDef, optional body);

impl_maybe_incomplete_for!(EntityDef);

impl Validate for EntityDef {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.name
            .validate(top, loader, Some(IdentifierCaseConvention::TypeDefinition));
        if let Some(body) = &self.body {
            body.validate(top, cache, loader, check_constraints);
        }
    }
}

impl EntityDef {
    // --------------------------------------------------------------------------------------------
    // EntityDef :: Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(EntityBody);

impl_has_members_for!(EntityBody);

impl_has_source_span_for!(EntityBody);

impl_maybe_incomplete_for!(EntityBody; over members);

impl Validate for EntityBody {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.identity
            .validate(top, cache, loader, check_constraints);
        for annotation in &self.annotations {
            annotation.validate(top, cache, loader, check_constraints);
        }
        for member in &self.members {
            member.validate(top, cache, loader, check_constraints);
        }
    }
}

impl References for EntityBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_annotations(names))
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_types(names))
    }
}

impl EntityBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(identity: Member) -> Self {
        Self {
            span: None,
            identity,
            annotations: Default::default(),
            members: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub identity, set_identity => Member);
}
