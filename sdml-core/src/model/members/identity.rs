use crate::model::members::TypeReference;
use crate::model::{AnnotationOnlyBody, Identifier, IdentifierReference, ModelElement, Span};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Identity
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `identify_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IdentityMember {
    span: Option<Span>,
    name: Identifier,
    inner: IdentityMemberInner,
}

/// Corresponds to the choice component within grammar rule `identity_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum IdentityMemberInner {
    PropertyRole(Identifier),
    Defined(IdentityMemberDef),
}

/// Corresponds to the definition component within grammar rule `identity_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IdentityMemberDef {
    target_type: TypeReference,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Identity
// ------------------------------------------------------------------------------------------------

member_model_element_impl!(IdentityMember);

member_impl!(IdentityMember, IdentityMemberInner, IdentityMemberDef);

member_inner_impl!(IdentityMemberInner, IdentityMemberDef);

impl IdentityMemberDef {
    pub fn new(target_type: TypeReference) -> Self {
        Self {
            target_type,
            body: None,
        }
    }
    pub fn new_named(target_type: IdentifierReference) -> Self {
        Self {
            target_type: target_type.into(),
            body: None,
        }
    }
    pub fn new_unknown() -> Self {
        Self {
            target_type: TypeReference::Unknown,
            body: None,
        }
    }
    member_def_impl!();
}
