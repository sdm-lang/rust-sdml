use crate::model::members::{Cardinality, TypeReference};
use crate::model::{AnnotationOnlyBody, Identifier, IdentifierReference, ModelElement, Span};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ ByValue
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `by_value_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ByValueMember {
    span: Option<Span>,
    name: Identifier,
    inner: ByValueMemberInner,
}

/// Corresponds to the choice component within grammar rule `by_value_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ByValueMemberInner {
    PropertyRole(Identifier),
    Defined(ByValueMemberDef),
}

/// Corresponds to the definition component within grammar rule `by_value_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ByValueMemberDef {
    target_type: TypeReference,
    target_cardinality: Cardinality,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ ByValue
// ------------------------------------------------------------------------------------------------

member_model_element_impl!(ByValueMember);

member_impl!(ByValueMember, ByValueMemberInner, ByValueMemberDef);

member_inner_impl!(ByValueMemberInner, ByValueMemberDef);

impl ByValueMemberDef {
    pub fn new(target_type: TypeReference) -> Self {
        Self {
            target_type,
            target_cardinality: Cardinality::one(),
            body: None,
        }
    }
    pub fn new_named(target_type: IdentifierReference) -> Self {
        Self {
            target_type: target_type.into(),
            target_cardinality: Cardinality::one(),
            body: None,
        }
    }
    pub fn new_unknown() -> Self {
        Self {
            target_type: TypeReference::Unknown,
            target_cardinality: Cardinality::one(),
            body: None,
        }
    }

    pub fn target_cardinality(&self) -> &Cardinality {
        &self.target_cardinality
    }

    pub fn set_target_cardinality(&mut self, target_cardinality: Cardinality) {
        self.target_cardinality = target_cardinality;
    }

    member_def_impl!();
}
