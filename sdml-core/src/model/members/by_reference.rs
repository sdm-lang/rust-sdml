use crate::error::Error;
use crate::model::annotations::AnnotationOnlyBody;
use crate::model::check::Validate;
use crate::model::identifiers::{Identifier, IdentifierReference};
use crate::model::members::{Cardinality, MemberKind, TypeReference};
use crate::model::modules::Module;
use crate::model::{References, Span};
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
    kind: MemberKind<ByReferenceMemberDef>,
}

/// Corresponds to the definition component within grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ByReferenceMemberDef {
    span: Option<Span>,
    inverse_name: Option<Identifier>,
    target_cardinality: Cardinality,
    target_type: TypeReference,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ ByReference
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(ByReferenceMember);

impl_has_source_span_for!(ByReferenceMember);

impl_member_for!(ByReferenceMember, ByReferenceMemberDef);

impl_member_outer_for!(ByReferenceMember, ByReferenceMemberDef);

impl_references_for!(ByReferenceMember => delegate kind);

impl_validate_for!(ByReferenceMember => delegate kind);

// ------------------------------------------------------------------------------------------------

impl Into<MemberKind<ByReferenceMemberDef>> for ByReferenceMemberDef {
    fn into(self) -> MemberKind<ByReferenceMemberDef> {
        MemberKind::Definition(self)
    }
}

impl_has_cardinality_for!(ByReferenceMemberDef);

impl_has_optional_body_for!(ByReferenceMemberDef);

impl_has_source_span_for!(ByReferenceMemberDef);

impl_has_type_for!(ByReferenceMemberDef);

impl_member_def_references_for!(ByReferenceMemberDef);

impl Validate for ByReferenceMemberDef {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        Ok(self.target_type.is_complete(top)? && self.target_cardinality.is_complete(top)?)
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        // TODO: check inverse name exists
        // TODO: check target type exists
        // TODO: check property reference exists
        Ok(true)
    }
}

impl ByReferenceMemberDef {
    pub fn new<T>(target_type: T) -> Self
    where
        T: Into<TypeReference>,
    {
        Self {
            span: Default::default(),
            target_type: target_type.into(),
            target_cardinality: Cardinality::zero_or_one(),
            inverse_name: None,
            body: None,
        }
    }

    pub fn new_unknown() -> Self {
        Self {
            span: Default::default(),
            target_type: TypeReference::Unknown,
            target_cardinality: Cardinality::zero_or_one(),
            inverse_name: None,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn inverse_name(&self) -> Option<&Identifier> {
        self.inverse_name.as_ref()
    }

    pub fn set_inverse_name(&mut self, inverse_name: Identifier) {
        self.inverse_name = Some(inverse_name);
    }

    pub fn unset_inverse_name(&mut self) {
        self.inverse_name = None;
    }
}
