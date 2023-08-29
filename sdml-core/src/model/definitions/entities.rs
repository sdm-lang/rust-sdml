use crate::error::Error;
use crate::model::References;
use crate::model::{
    annotations::Annotation,
    check::Validate,
    definitions::{HasGroups, HasMembers},
    identifiers::{Identifier, IdentifierReference},
    members::{ByReferenceMember, ByValueMember, IdentityMember},
    modules::Module,
    HasName, Span,
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

impl References for EntityBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.flat_members()
            .for_each(|m| m.referenced_annotations(names))
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.flat_members().for_each(|m| m.referenced_types(names))
    }
}

impl Validate for EntityBody {
    fn is_complete(&self, _top: &Module) -> Result<bool, Error> {
        todo!()
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        todo!()
    }
}

impl EntityBody {
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

    pub fn identity(&self) -> &IdentityMember {
        &self.identity
    }

    pub fn set_identity(&mut self, identity: IdentityMember) {
        self.identity = identity;
    }

    // --------------------------------------------------------------------------------------------

    pub fn flat_members(&self) -> impl Iterator<Item = &EntityMember> {
        self.members()
            .chain(self.groups().flat_map(|g| g.members()))
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(EntityMember, ByValue, ByValueMember);

impl_from_for_variant!(EntityMember, ByReference, ByReferenceMember);

impl_references_for!(EntityMember => variants ByValue, ByReference);

impl_validate_for!(EntityMember => variants ByValue, ByReference);

impl EntityMember {
    pub fn is_by_value(&self) -> bool {
        matches!(self, Self::ByValue(_))
    }

    pub fn as_by_value(&self) -> Option<&ByValueMember> {
        match self {
            Self::ByValue(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_by_reference(&self) -> bool {
        matches!(self, Self::ByReference(_))
    }

    pub fn as_by_reference(&self) -> Option<&ByReferenceMember> {
        match self {
            Self::ByReference(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn name(&self) -> &Identifier {
        match self {
            Self::ByValue(v) => v.name(),
            Self::ByReference(v) => v.name(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn target_type(&self) -> Option<&IdentifierReference> {
        match self {
            Self::ByValue(v) => v.target_type(),
            Self::ByReference(v) => v.target_type(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(EntityGroup);

impl_has_annotations_for!(EntityGroup);

impl_has_members_for!(EntityGroup, EntityMember);

impl References for EntityGroup {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_annotations(names));
    }
}

impl Validate for EntityGroup {
    fn is_complete(&self, _top: &Module) -> Result<bool, Error> {
        todo!()
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
