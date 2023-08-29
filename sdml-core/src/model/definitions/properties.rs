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

impl References for PropertyBody {
    fn referenced_types<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {}

    fn referenced_annotations<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {}
}

impl Validate for PropertyBody {
    fn is_complete(&self, _top: &Module) -> Result<bool, Error> {
        todo!()
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        todo!()
    }
}

impl PropertyBody {
    pub fn has_roles(&self) -> bool {
        !self.roles.is_empty()
    }

    pub fn roles_len(&self) -> usize {
        self.roles.len()
    }

    pub fn roles(&self) -> impl Iterator<Item = &PropertyRole> {
        self.roles.iter()
    }

    pub fn roles_mut(&mut self) -> impl Iterator<Item = &mut PropertyRole> {
        self.roles.iter_mut()
    }

    pub fn add_to_roles(&mut self, value: PropertyRole) {
        self.roles.push(value)
    }

    pub fn extend_roles<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = PropertyRole>,
    {
        self.roles.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_name_for!(PropertyRole);

impl_has_source_span_for!(PropertyRole);

impl References for PropertyRole {
    fn referenced_types<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {}

    fn referenced_annotations<'a>(&'a self, _names: &mut HashSet<&'a IdentifierReference>) {}
}

impl Validate for PropertyRole {
    fn is_complete(&self, _top: &Module) -> Result<bool, Error> {
        todo!()
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        todo!()
    }
}

impl PropertyRole {
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
    fn is_complete(
        &self,
        _top: &crate::model::modules::Module,
    ) -> Result<bool, crate::error::Error> {
        todo!()
    }

    fn is_valid(
        &self,
        _check_constraints: bool,
        _top: &crate::model::modules::Module,
    ) -> Result<bool, crate::error::Error> {
        todo!()
    }
}

impl PropertyRoleDef {
    pub fn is_identity(&self) -> bool {
        matches!(self, Self::Identity(_))
    }

    pub fn as_identity(&self) -> Option<&IdentityMemberDef> {
        match self {
            Self::Identity(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_by_value(&self) -> bool {
        matches!(self, Self::ByValue(_))
    }

    pub fn as_by_value(&self) -> Option<&ByValueMemberDef> {
        match self {
            Self::ByValue(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_by_reference(&self) -> bool {
        matches!(self, Self::ByReference(_))
    }

    pub fn as_by_reference(&self) -> Option<&ByReferenceMemberDef> {
        match self {
            Self::ByReference(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    fn body(&self) -> Option<&AnnotationOnlyBody> {
        match self {
            PropertyRoleDef::Identity(v) => v.body(),
            PropertyRoleDef::ByReference(v) => v.body(),
            PropertyRoleDef::ByValue(v) => v.body(),
        }
    }

    #[inline(always)]
    fn target_cardinality(&self) -> &Cardinality {
        match self {
            PropertyRoleDef::Identity(v) => v.target_cardinality(),
            PropertyRoleDef::ByReference(v) => v.target_cardinality(),
            PropertyRoleDef::ByValue(v) => v.target_cardinality(),
        }
    }

    #[inline(always)]
    fn target_type(&self) -> &TypeReference {
        match self {
            PropertyRoleDef::Identity(v) => v.target_type(),
            PropertyRoleDef::ByReference(v) => v.target_type(),
            PropertyRoleDef::ByValue(v) => v.target_type(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
