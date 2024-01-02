use crate::error::Error;
use crate::model::annotations::AnnotationOnlyBody;
use crate::model::check::Validate;
use crate::model::identifiers::{Identifier, IdentifierReference};
use crate::model::modules::Module;
use crate::model::{References, Span};
use std::collections::HashSet;
use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Member
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rules `member` and `entity_identity`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Member {
    span: Option<Span>,
    name: Identifier,
    kind: MemberKind,
}

/// Corresponds to the definition component within grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MemberDef {
    span: Option<Span>,
    inverse_name: Option<Identifier>,
    target_cardinality: Cardinality,
    target_type: TypeReference,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Private Types ❱ Members ❱ MemberKind
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
enum MemberKind {
    PropertyReference(IdentifierReference),
    Definition(MemberDef),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Member
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(Member);

impl_has_source_span_for!(Member);

impl Validate for Member {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        match &self.kind {
            MemberKind::PropertyReference(_) => {
                // TODO: check this is a property reference
                Ok(true)
            }
            MemberKind::Definition(v) => v.is_complete(top),
        }
    }

    fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
        match &self.kind {
            MemberKind::PropertyReference(_) => {
                // TODO: check this is a property reference
                Ok(true)
            }
            MemberKind::Definition(v) => v.is_valid(check_constraints, top),
        }
    }
}

impl References for Member {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match &self.kind {
            MemberKind::PropertyReference(v) => {
                names.insert(v);
            }
            MemberKind::Definition(v) => v.referenced_annotations(names),
        }
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match &self.kind {
            MemberKind::PropertyReference(v) => {
                names.insert(v);
            }
            MemberKind::Definition(v) => v.referenced_types(names),
        }
    }
}

impl Member {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new_property_reference(role: Identifier, in_property: IdentifierReference) -> Self {
        Self {
            span: None,
            name: role,
            kind: MemberKind::PropertyReference(in_property),
        }
    }

    pub fn new_definition(name: Identifier, definition: MemberDef) -> Self {
        Self {
            span: None,
            name,
            kind: MemberKind::Definition(definition),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub fn is_property_reference(&self) -> bool {
        matches!(self.kind, MemberKind::PropertyReference(_))
    }

    pub fn as_property_reference(&self) -> Option<&IdentifierReference> {
        if let MemberKind::PropertyReference(v) = &self.kind {
            Some(v)
        } else {
            None
        }
    }

    pub fn is_definition(&self) -> bool {
        matches!(self.kind, MemberKind::Definition(_))
    }

    pub fn as_definition(&self) -> Option<&MemberDef> {
        if let MemberKind::Definition(v) = &self.kind {
            Some(v)
        } else {
            None
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ MemberDef
// ------------------------------------------------------------------------------------------------

impl_has_cardinality_for!(MemberDef);

impl_has_optional_body_for!(MemberDef);

impl_has_source_span_for!(MemberDef);

impl_has_type_for!(MemberDef);

impl References for MemberDef {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.target_type.referenced_types(names);
    }
}

impl Validate for MemberDef {
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

impl MemberDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<T>(target_type: T) -> Self
    where
        T: Into<TypeReference>,
    {
        Self {
            span: None,
            target_type: target_type.into(),
            target_cardinality: Cardinality::zero_or_one(),
            inverse_name: None,
            body: None,
        }
    }

    pub const fn new_unknown() -> Self {
        Self {
            span: None,
            target_type: TypeReference::Unknown,
            target_cardinality: Cardinality::zero_or_one(),
            inverse_name: None,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub inverse_name, set_inverse_name, unset_inverse_name => optional has_inverse_name, Identifier);
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod cardinality;
pub use cardinality::{
    Cardinality, CardinalityRange, HasCardinality, Ordering, PseudoSequenceType, Uniqueness,
    DEFAULT_CARDINALITY, DEFAULT_CARDINALITY_RANGE, TYPE_BAG_CARDINALITY, TYPE_LIST_CARDINALITY,
    TYPE_MAYBE_CARDINALITY, TYPE_ORDERED_SET_CARDINALITY, TYPE_SET_CARDINALITY,
};

mod types;
pub use types::{HasType, MappingType, TypeReference};
