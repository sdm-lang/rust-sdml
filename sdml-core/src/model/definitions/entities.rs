use crate::cache::ModuleCache;
use crate::load::ModuleLoader;
use crate::model::annotations::AnnotationOnlyBody;
use crate::model::check::MaybeIncomplete;
use crate::model::members::TypeReference;
use crate::model::References;
use crate::model::{
    annotations::{Annotation, HasAnnotations},
    check::Validate,
    definitions::HasMembers,
    identifiers::{Identifier, IdentifierReference},
    members::{Cardinality, Member, DEFAULT_CARDINALITY},
    modules::Module,
    Span,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tracing::warn;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `entity_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<EntityBody>,
}

/// Corresponds to the grammar rule `entity_body`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    identity: EntityIdentity,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    members: Vec<Member>,
}

// TODO make this just Member!

/// Corresponds to the grammar rules `member` and `entity_identity`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityIdentity {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    kind: MemberKind,
}

/// Corresponds to the definition component within grammar rule `entity_identity`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityIdentityDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    target_type: TypeReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<AnnotationOnlyBody>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
enum MemberKind {
    PropertyReference(IdentifierReference),
    Definition(EntityIdentityDef),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(EntityDef);

impl_has_optional_body_for!(EntityDef, EntityBody);

impl_has_source_span_for!(EntityDef);

impl_references_for!(EntityDef => delegate optional body);

impl_validate_for!(EntityDef => delegate optional body);

impl_annotation_builder!(EntityDef, optional body);

impl_maybe_invalid_for!(EntityDef);

impl EntityDef {
    // --------------------------------------------------------------------------------------------
    // EntityDef :: Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(EntityBody);

impl_has_members_for!(EntityBody);

impl_has_source_span_for!(EntityBody);

impl_validate_for_annotations_and_members!(EntityBody);

impl_maybe_invalid_for!(EntityBody; over members);

impl References for EntityBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_annotations(names))
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_types(names))
    }
}

impl EntityBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(identity: EntityIdentity) -> Self {
        Self {
            span: None,
            identity,
            annotations: Default::default(),
            members: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub identity, set_identity => EntityIdentity);
}

// ------------------------------------------------------------------------------------------------

impl_has_name_for!(EntityIdentity);

impl_has_source_span_for!(EntityIdentity);

impl MaybeIncomplete for EntityIdentity {
    fn is_incomplete(&self, top: &Module, cache: &ModuleCache) -> bool {
        match &self.kind {
            MemberKind::PropertyReference(_) => false,
            MemberKind::Definition(v) => v.is_incomplete(top, cache),
        }
    }
}

impl Validate for EntityIdentity {
    fn validate(
        &self,
        top: &Module,
        cache: &ModuleCache,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        match &self.kind {
            MemberKind::PropertyReference(_) => {
                // TODO: check this is a property reference
            }
            MemberKind::Definition(v) => v.validate(top, cache, loader, check_constraints),
        }
    }
}

impl References for EntityIdentity {
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

impl EntityIdentity {
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

    pub fn new_definition(name: Identifier, definition: EntityIdentityDef) -> Self {
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

    pub fn as_definition(&self) -> Option<&EntityIdentityDef> {
        if let MemberKind::Definition(v) = &self.kind {
            Some(v)
        } else {
            None
        }
    }
}

// ------------------------------------------------------------------------------------------------

// No need for an implementation of Cardinality trait.

impl_has_optional_body_for!(EntityIdentityDef);

impl_has_source_span_for!(EntityIdentityDef);

impl_has_type_for!(EntityIdentityDef);

impl References for EntityIdentityDef {
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

impl MaybeIncomplete for EntityIdentityDef {
    fn is_incomplete(&self, top: &Module, cache: &ModuleCache) -> bool {
        self.target_type.is_incomplete(top, cache)
    }
}

impl Validate for EntityIdentityDef {
    fn validate(
        &self,
        _top: &Module,
        _cache: &ModuleCache,
        _loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        warn!("");
        // TODO: check type reference
    }
}

impl EntityIdentityDef {
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
            body: None,
        }
    }

    pub const fn new_unknown() -> Self {
        Self {
            span: None,
            target_type: TypeReference::Unknown,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn target_cardinality(&self) -> &Cardinality {
        &DEFAULT_CARDINALITY
    }
}
