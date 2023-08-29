use crate::{
    error::Error,
    model::{
        annotations::Annotation,
        check::Validate,
        definitions::{HasGroups, HasMembers},
        identifiers::{Identifier, IdentifierReference},
        members::ByValueMember,
        modules::Module,
        References, Span,
    },
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `structure_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<StructureBody>,
}

/// Corresponds to the grammar rule `structure_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>,
    groups: Vec<StructureGroup>,
}

/// Corresponds to the grammar rule `structure_group`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureGroup {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>, // assert!(!members.is_empty());
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(StructureDef);

impl_has_optional_body_for!(StructureDef, StructureBody);

impl_has_source_span_for!(StructureDef);

impl_references_for!(StructureDef => delegate optional body);

impl_validate_for!(StructureDef => delegate optional body, false, true);

impl StructureDef {
    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(StructureBody);

impl_has_groups_for!(StructureBody, StructureGroup, ByValueMember);

impl_has_members_for!(StructureBody, ByValueMember);

impl_has_source_span_for!(StructureBody);

impl References for StructureBody {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.flat_members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.flat_members()
            .for_each(|m| m.referenced_annotations(names));
    }
}

impl Validate for StructureBody {
    fn is_complete(&self, _top: &Module) -> Result<bool, Error> {
        todo!()
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        todo!()
    }
}

impl StructureBody {
    pub fn flat_members(&self) -> impl Iterator<Item = &ByValueMember> {
        self.members()
            .chain(self.groups().flat_map(|g| g.members()))
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(StructureGroup);

impl_has_members_for!(StructureGroup, ByValueMember);

impl_has_source_span_for!(StructureGroup);

impl References for StructureGroup {
    fn referenced_types<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {}

    fn referenced_annotations<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {
        // TODO: self plus members
    }
}

impl Validate for StructureGroup {
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
