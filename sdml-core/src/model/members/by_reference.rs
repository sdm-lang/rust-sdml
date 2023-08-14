use crate::model::members::{Cardinality, TypeReference};
use crate::model::{AnnotationOnlyBody, Identifier, IdentifierReference, ModelElement, Span};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ ByReference
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ByReferenceMember {
    span: Option<Span>,
    name: Identifier,
    inner: ByReferenceMemberInner,
}

/// Corresponds to the choice component within grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ByReferenceMemberInner {
    PropertyRole(Identifier),
    Defined(ByReferenceMemberDef),
}

/// Corresponds to the definition component within grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ByReferenceMemberDef {
    inverse_name: Option<Identifier>,
    target_type: TypeReference,
    target_cardinality: Cardinality,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ ByReference
// ------------------------------------------------------------------------------------------------

member_model_element_impl!(ByReferenceMember);

member_impl!(
    ByReferenceMember,
    ByReferenceMemberInner,
    ByReferenceMemberDef
);

member_inner_impl!(ByReferenceMemberInner, ByReferenceMemberDef);

impl ByReferenceMemberDef {
    pub fn new(target_type: TypeReference) -> Self {
        Self {
            target_type,
            target_cardinality: Cardinality::zero_or_one(),
            inverse_name: None,
            body: None,
        }
    }
    pub fn new_named(target_type: IdentifierReference) -> Self {
        Self {
            target_type: target_type.into(),
            target_cardinality: Cardinality::zero_or_one(),
            inverse_name: None,
            body: None,
        }
    }
    pub fn new_unknown() -> Self {
        Self {
            target_type: TypeReference::Unknown,
            target_cardinality: Cardinality::zero_or_one(),
            inverse_name: None,
            body: None,
        }
    }

    pub fn target_cardinality(&self) -> &Cardinality {
        &self.target_cardinality
    }

    pub fn set_target_cardinality(&mut self, target_cardinality: Cardinality) {
        self.target_cardinality = target_cardinality;
    }

    pub fn inverse_name(&self) -> Option<&Identifier> {
        self.inverse_name.as_ref()
    }

    pub fn set_inverse_name(&mut self, inverse_name: Identifier) {
        self.inverse_name = Some(inverse_name);
    }

    pub fn unset_inverse_name(&mut self) {
        self.inverse_name = None;
    }

    member_def_impl!();
}
