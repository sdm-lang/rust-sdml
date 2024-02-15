use crate::model::{
    annotations::{Annotation, HasAnnotations},
    check::Validate,
    definitions::HasMembers,
    identifiers::{Identifier, IdentifierReference},
    members::Member,
    References, Span,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `structure_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<StructureBody>,
}

/// Corresponds to the grammar rule `structure_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    members: Vec<Member>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(StructureDef);

impl_has_optional_body_for!(StructureDef, StructureBody);

impl_has_source_span_for!(StructureDef);

impl_references_for!(StructureDef => delegate optional body);

impl_validate_for!(StructureDef => delegate optional body);

impl_annotation_builder!(StructureDef, optional body);

impl_maybe_invalid_for!(StructureDef);

impl StructureDef {
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

impl_has_annotations_for!(StructureBody);

impl_has_members_for!(StructureBody);

impl_has_source_span_for!(StructureBody);

impl_maybe_invalid_for!(StructureBody; over members);

impl_validate_for_annotations_and_members!(StructureBody);

impl References for StructureBody {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_annotations(names));
    }
}
