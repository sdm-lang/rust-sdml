use crate::error::Error;
use crate::model::annotations::AnnotationOnlyBody;
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

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `entity_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<EntityBody>,
}

/// Corresponds to the grammar rule `entity_body`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityBody {
    span: Option<Span>,
    identity: EntityIdentity,
    annotations: Vec<Annotation>,
    members: Vec<Member>,
}

/// Corresponds to the grammar rules `member` and `entity_identity`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityIdentity {
    span: Option<Span>,
    name: Identifier,
    kind: MemberKind,
}

/// Corresponds to the definition component within grammar rule `entity_identity`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityIdentityDef {
    span: Option<Span>,
    target_type: TypeReference,
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

impl_validate_for!(EntityDef => delegate optional body, false, true);

impl EntityDef {
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

impl_has_annotations_for!(EntityBody);

impl_has_members_for!(EntityBody);

impl_has_source_span_for!(EntityBody);

impl_validate_for_annotations_and_members!(EntityBody);

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

impl Validate for EntityIdentity {
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

impl Validate for EntityIdentityDef {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        self.target_type.is_complete(top)
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        // TOD: check type reference
        Ok(true)
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
