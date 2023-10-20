use crate::{
    error::Error,
    model::{
        annotations::{Annotation, HasAnnotations},
        check::Validate,
        definitions::HasMembers,
        identifiers::IdentifierReference,
        members::Member,
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
// Public Types ❱ Members ❱ Group
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `member_group`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MemberGroup {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<Member>, // assert!(!members.is_empty());
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
// Implementations ❱ Members ❱ Group
// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(MemberGroup);

impl_has_members_for!(MemberGroup);

impl_has_source_span_for!(MemberGroup);

impl_validate_for_annotations_and_members!(MemberGroup);

impl References for MemberGroup {
    fn referenced_types<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {}

    fn referenced_annotations<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {
        // TODO: self plus members
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
