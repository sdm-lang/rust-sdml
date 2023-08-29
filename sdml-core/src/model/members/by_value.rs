use crate::error::Error;
use crate::model::annotations::AnnotationOnlyBody;
use crate::model::check::Validate;
use crate::model::members::{Cardinality, MemberKind, TypeReference};
use crate::model::modules::Module;
use crate::model::{Identifier, IdentifierReference, References, Span};
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
    kind: MemberKind<ByValueMemberDef>,
}

/// Corresponds to the definition component within grammar rule `by_value_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ByValueMemberDef {
    span: Option<Span>,
    target_cardinality: Cardinality,
    target_type: TypeReference,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ ByValue
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(ByValueMember);

impl_has_source_span_for!(ByValueMember);

impl_member_for!(ByValueMember, ByValueMemberDef);

impl_member_outer_for!(ByValueMember, ByValueMemberDef);

impl_references_for!(ByValueMember => delegate kind);

impl_validate_for!(ByValueMember => delegate kind);

// ------------------------------------------------------------------------------------------------

#[allow(clippy::from_over_into)]
impl Into<MemberKind<ByValueMemberDef>> for ByValueMemberDef {
    fn into(self) -> MemberKind<ByValueMemberDef> {
        MemberKind::Definition(self)
    }
}

impl_has_cardinality_for!(ByValueMemberDef);

impl_has_optional_body_for!(ByValueMemberDef);

impl_has_source_span_for!(ByValueMemberDef);

impl_has_type_for!(ByValueMemberDef);

impl_member_def_references_for!(ByValueMemberDef);

impl Validate for ByValueMemberDef {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        Ok(self.target_type.is_complete(top)? && self.target_cardinality.is_complete(top)?)
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        // TODO: check target type exists
        // TODO: check property reference exists
        Ok(true)
    }
}

impl ByValueMemberDef {
    pub fn new<T>(target_type: T) -> Self
    where
        T: Into<TypeReference>,
    {
        Self {
            span: Default::default(),
            target_type: target_type.into(),
            target_cardinality: Cardinality::one(),
            body: None,
        }
    }

    pub fn new_unknown() -> Self {
        Self {
            span: Default::default(),
            target_type: TypeReference::Unknown,
            target_cardinality: Cardinality::one(),
            body: None,
        }
    }
}
