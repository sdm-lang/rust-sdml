use crate::{
    cache::ModuleCache,
    load::ModuleLoader,
    model::{
        annotations::{Annotation, AnnotationOnlyBody, HasAnnotations},
        check::{MaybeIncomplete, Validate},
        definitions::EntityIdentityDef,
        identifiers::{Identifier, IdentifierReference},
        members::{Cardinality, HasCardinality, HasType, MemberDef, TypeReference},
        modules::Module,
        HasName, HasOptionalBody, References, Span,
    },
};
use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
use std::{collections::HashSet, fmt::Debug};
use tracing::warn;

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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<PropertyBody>,
}

/// Corresponds to the grammar rule `property_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PropertyBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    annotations: Vec<Annotation>,
    roles: Vec<PropertyRole>, // assert!(!roles.is_empty());
}

/// Corresponds to the grammar rule `property_role`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PropertyRole {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    definition: PropertyRoleDef,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PropertyRoleDef {
    Identity(EntityIdentityDef),
    Member(MemberDef),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Properties
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(PropertyDef);

impl_has_optional_body_for!(PropertyDef, PropertyBody);

impl_has_source_span_for!(PropertyDef);

impl_references_for!(PropertyDef => delegate optional body);

impl_maybe_incomplete_for!(PropertyDef);

impl_annotation_builder!(PropertyDef, optional body);

impl Validate for PropertyDef {
    fn validate(
        &self,
        top: &crate::model::modules::Module,
        cache: &crate::cache::ModuleCache,
        loader: &impl crate::load::ModuleLoader,
        check_constraints: bool,
    ) {
        self.name()
            .validate(top, loader, Some(IdentifierCaseConvention::TypeDefinition));
        if let Some(body) = &self.body {
            body.validate(top, cache, loader, check_constraints);
        }
    }
}

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

impl_maybe_incomplete_for!(PropertyBody; over roles);

impl Validate for PropertyBody {
    fn validate(
        &self,
        top: &Module,
        cache: &ModuleCache,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.annotations()
            .for_each(|ann| ann.validate(top, cache, loader, check_constraints));
        self.roles()
            .for_each(|m| m.validate(top, cache, loader, check_constraints));
    }
}

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

impl MaybeIncomplete for PropertyRole {
    fn is_incomplete(&self, top: &Module, cache: &ModuleCache) -> bool {
        self.definition.is_incomplete(top, cache)
    }
}

impl Validate for PropertyRole {
    fn validate(
        &self,
        top: &Module,
        cache: &ModuleCache,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.target_type()
            .validate(top, cache, loader, check_constraints);
        self.target_cardinality()
            .validate(top, cache, loader, check_constraints);
    }
}

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

impl From<EntityIdentityDef> for PropertyRoleDef {
    fn from(value: EntityIdentityDef) -> Self {
        Self::Identity(value)
    }
}

impl From<MemberDef> for PropertyRoleDef {
    fn from(value: MemberDef) -> Self {
        Self::Member(value)
    }
}

impl_has_type_for!(PropertyRoleDef => variants Identity, Member);

impl_has_optional_body_for!(PropertyRoleDef => variants Identity, Member);

impl_maybe_incomplete_for!(PropertyRoleDef; variants Identity, Member);

impl References for PropertyRoleDef {
    fn referenced_types(&self, _: &mut HashSet<&IdentifierReference>) {}

    fn referenced_annotations(&self, _: &mut HashSet<&IdentifierReference>) {}
}

impl Validate for PropertyRoleDef {
    fn validate(
        &self,
        _top: &Module,
        _cache: &ModuleCache,
        _loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        warn!("Missing Validation for PropertyRoleDef");
    }
}

impl PropertyRoleDef {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Identity (EntityIdentityDef) => is_identity, as_identity);

    is_as_variant!(Member (MemberDef) => is_member, as_member);

    // --------------------------------------------------------------------------------------------
    // Delegated
    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    fn target_cardinality(&self) -> &Cardinality {
        match self {
            PropertyRoleDef::Identity(v) => v.target_cardinality(),
            PropertyRoleDef::Member(v) => v.target_cardinality(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
