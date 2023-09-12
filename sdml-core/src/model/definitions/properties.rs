use crate::{
    error::Error,
    model::{
        annotations::{Annotation, AnnotationOnlyBody},
        check::Validate,
        identifiers::{Identifier, IdentifierReference},
        members::{
            ByReferenceMemberDef, ByValueMemberDef, Cardinality, HasCardinality, HasType,
            IdentityMemberDef, TypeReference,
        },
        modules::Module,
        HasOptionalBody, References, Span,
    },
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Properties
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `property_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PropertyDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<PropertyBody>,
}

/// Corresponds to the grammar rule `property_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PropertyBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    roles: Vec<PropertyRole>, // assert!(!roles.is_empty());
}

/// Corresponds to the grammar rule `property_role`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PropertyRole {
    span: Option<Span>,
    name: Identifier,
    definition: PropertyRoleDef,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PropertyRoleDef {
    Identity(IdentityMemberDef),
    ByReference(ByReferenceMemberDef),
    ByValue(ByValueMemberDef),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Properties
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(PropertyDef);

impl_has_optional_body_for!(PropertyDef, PropertyBody);

impl_has_source_span_for!(PropertyDef);

impl_references_for!(PropertyDef => delegate optional body);

impl_validate_for!(PropertyDef => delegate optional body, false, true);

impl PropertyDef {
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

impl_has_annotations_for!(PropertyBody);

impl_has_source_span_for!(PropertyBody);

impl_validate_for!(PropertyBody => todo!);

impl References for PropertyBody {
    fn referenced_types<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {}

    fn referenced_annotations<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {}
}

impl PropertyBody {
    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set_vec!(
        pub
        has has_roles,
        roles_len,
        roles,
        roles_mut,
        add_to_roles,
        extend_roles
            => roles, PropertyRole
    );
}

// ------------------------------------------------------------------------------------------------

impl_has_name_for!(PropertyRole);

impl_has_source_span_for!(PropertyRole);

impl_validate_for!(PropertyRole => todo!);

impl References for PropertyRole {
    fn referenced_types<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {}

    fn referenced_annotations<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {}
}

impl PropertyRole {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<D>(name: Identifier, definition: D) -> Self
    where
        D: Into<PropertyRoleDef>,
    {
        Self {
            span: None,
            name,
            definition: definition.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn definition(&self) -> &PropertyRoleDef {
        &self.definition
    }

    pub fn set_definition<D>(&mut self, definition: D)
    where
        D: Into<PropertyRoleDef>,
    {
        self.definition = definition.into();
    }

    // --------------------------------------------------------------------------------------------
    // Delegated
    // --------------------------------------------------------------------------------------------

    delegate!(pub body, Option<&AnnotationOnlyBody>, definition);

    delegate!(pub target_cardinality, &Cardinality, definition);

    delegate!(pub target_type, &TypeReference, definition);
}

// ------------------------------------------------------------------------------------------------

impl From<IdentityMemberDef> for PropertyRoleDef {
    fn from(value: IdentityMemberDef) -> Self {
        Self::Identity(value)
    }
}

impl From<ByValueMemberDef> for PropertyRoleDef {
    fn from(value: ByValueMemberDef) -> Self {
        Self::ByValue(value)
    }
}

impl From<ByReferenceMemberDef> for PropertyRoleDef {
    fn from(value: ByReferenceMemberDef) -> Self {
        Self::ByReference(value)
    }
}

impl References for PropertyRoleDef {
    fn referenced_types(&self, _names: &mut HashSet<&IdentifierReference>) {}

    fn referenced_annotations(&self, _names: &mut HashSet<&IdentifierReference>) {}
}

impl Validate for PropertyRoleDef {
    fn is_complete(&self, _top: &Module) -> Result<bool, Error> {
        todo!()
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        todo!()
    }
}

impl_has_type_for!(PropertyRoleDef => variants Identity, ByReference, ByValue);

impl_has_optional_body_for!(PropertyRoleDef => variants Identity, ByReference, ByValue);

impl PropertyRoleDef {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Identity (IdentityMemberDef) => is_identity, as_identity);

    is_as_variant!(ByValue (ByValueMemberDef) => is_by_value, as_by_value);

    is_as_variant!(ByReference (ByReferenceMemberDef) => is_by_reference, as_by_reference);

    // --------------------------------------------------------------------------------------------
    // Delegated
    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    fn target_cardinality(&self) -> &Cardinality {
        match self {
            PropertyRoleDef::Identity(v) => v.target_cardinality(),
            PropertyRoleDef::ByReference(v) => v.target_cardinality(),
            PropertyRoleDef::ByValue(v) => v.target_cardinality(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
