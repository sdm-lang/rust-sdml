use crate::error::Error;
use crate::model::References;
use crate::model::{
    annotations::{Annotation, HasAnnotations},
    check::Validate,
    definitions::{HasGroups, HasMembers},
    identifiers::{Identifier, IdentifierReference},
    members::{ByReferenceMember, ByValueMember, IdentityMember},
    modules::Module,
    Span,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `entity_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<EntityBody>,
}

/// Corresponds to the grammar rule `entity_body`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityBody {
    span: Option<Span>,
    identity: IdentityMember,
    annotations: Vec<Annotation>,
    members: Vec<EntityMember>,
    groups: Vec<EntityGroup>,
}

/// Corresponds to the inner part of the grammar rule `entity_group`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum EntityMember {
    ByValue(ByValueMember),
    ByReference(ByReferenceMember),
}

/// Corresponds to the grammar rule `entity_group`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityGroup {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<EntityMember>, // assert!(!members.is_empty());
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(EntityDef);

impl_has_optional_body_for!(EntityDef, EntityBody);

impl_has_source_span_for!(EntityDef);

impl_references_for!(EntityDef => delegate optional body);

impl_validate_for!(EntityDef => delegate optional body, false, true);

impl EntityDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(EntityBody);

impl_has_groups_for!(EntityBody, EntityGroup, EntityMember);

impl_has_members_for!(EntityBody, EntityMember);

impl_has_source_span_for!(EntityBody);

impl_validate_for_annotations_and_members!(EntityBody);

impl References for EntityBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.flat_members()
            .for_each(|m| m.referenced_annotations(names))
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.flat_members().for_each(|m| m.referenced_types(names))
    }
}

impl EntityBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(identity: IdentityMember) -> Self {
        Self {
            span: None,
            identity,
            annotations: Default::default(),
            members: Default::default(),
            groups: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub identity, set_identity => IdentityMember);

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn flat_members(&self) -> impl Iterator<Item = &EntityMember> {
        self.members()
            .chain(self.groups().flat_map(|g| g.members()))
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(EntityMember, ByValue, ByValueMember);

impl_from_for_variant!(EntityMember, ByReference, ByReferenceMember);

impl_has_name_for!(EntityMember => variants ByValue, ByReference);

impl_references_for!(EntityMember => variants ByValue, ByReference);

//impl_has_type_for!(EntityMember => variants ByValue, ByReference);

impl_validate_for!(EntityMember => variants ByValue, ByReference);

impl EntityMember {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(ByValue (ByValueMember) => is_by_value, as_by_value);

    is_as_variant!(ByReference (ByReferenceMember) => is_by_reference, as_by_reference);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(EntityGroup);

impl_has_annotations_for!(EntityGroup);

impl_has_members_for!(EntityGroup, EntityMember);

impl_validate_for_annotations_and_members!(EntityGroup);

impl References for EntityGroup {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_annotations(names));
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
