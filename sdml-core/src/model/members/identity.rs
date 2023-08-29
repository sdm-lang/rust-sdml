use crate::error::Error;
use crate::model::annotations::AnnotationOnlyBody;
use crate::model::check::Validate;
use crate::model::members::{Cardinality, MemberKind, TypeReference, BY_IDENTITY_CARDINALITY};
use crate::model::modules::Module;
use crate::model::{Identifier, IdentifierReference, References, Span};
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
    kind: MemberKind<IdentityMemberDef>,
}

/// Corresponds to the definition component within grammar rule `identity_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IdentityMemberDef {
    span: Option<Span>,
    target_type: TypeReference,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Identity
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(IdentityMember);

impl_has_source_span_for!(IdentityMember);

impl_member_for!(IdentityMember, IdentityMemberDef);

impl_member_outer_for!(IdentityMember, IdentityMemberDef);

impl_validate_for!(IdentityMember => delegate kind);

impl_references_for!(IdentityMember => delegate kind);

// ------------------------------------------------------------------------------------------------

impl Into<MemberKind<IdentityMemberDef>> for IdentityMemberDef {
    fn into(self) -> MemberKind<IdentityMemberDef> {
        MemberKind::Definition(self)
    }
}

// No need for an implementation of Cardinality trait.

impl_has_optional_body_for!(IdentityMemberDef);

impl_has_source_span_for!(IdentityMemberDef);

impl_has_type_for!(IdentityMemberDef);

impl_member_def_references_for!(IdentityMemberDef);

impl Validate for IdentityMemberDef {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        self.target_type.is_complete(top)
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        Ok(true)
    }
}

impl IdentityMemberDef {
    pub fn new<T>(target_type: T) -> Self
    where
        T: Into<TypeReference>,
    {
        Self {
            span: Default::default(),
            target_type: target_type.into(),
            body: None,
        }
    }

    pub fn new_unknown() -> Self {
        Self {
            span: Default::default(),
            target_type: TypeReference::Unknown,
            body: None,
        }
    }

    pub fn target_cardinality(&self) -> &Cardinality {
        &BY_IDENTITY_CARDINALITY
    }
}
