use crate::model::{
    annotations::{Annotation, AnnotationOnlyBody, HasAnnotations},
    check::Validate,
    definitions::HasMembers,
    identifiers::{Identifier, IdentifierReference},
    members::Member,
    References, Span,
};
use std::{collections::HashSet, fmt::Debug};

use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Dimensions
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `dimension_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DimensionDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<DimensionBody>,
}

/// Corresponds to the grammar rule `dimension_body`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DimensionBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    identity: DimensionIdentity,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    parents: Vec<DimensionParent>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    members: Vec<Member>,
}

/// Corresponds to the anonymous grammar rule in `dimension_body`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum DimensionIdentity {
    Source(SourceEntity),
    Identity(Member),
}

/// Corresponds to the grammar rule `dimension_parent`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DimensionParent {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    target_entity: IdentifierReference,
    body: Option<AnnotationOnlyBody>,
}

/// Corresponds to the grammar rule `source_entity`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SourceEntity {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    target_entity: IdentifierReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    with_members: Vec<Identifier>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Dimensions
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(DimensionDef);

impl_has_optional_body_for!(DimensionDef, DimensionBody);

impl_has_source_span_for!(DimensionDef);

impl_references_for!(DimensionDef => delegate optional body);

impl_annotation_builder!(DimensionDef, optional body);

impl_maybe_incomplete_for!(DimensionDef);

impl Validate for DimensionDef {
    fn validate(
        &self,
        top: &crate::model::modules::Module,
        cache: &impl crate::store::ModuleStore,
        loader: &impl crate::load::ModuleLoader,
        check_constraints: bool,
    ) {
        self.name
            .validate(top, loader, Some(IdentifierCaseConvention::TypeDefinition));
        if let Some(body) = &self.body {
            body.validate(top, cache, loader, check_constraints);
        }
    }
}

impl DimensionDef {
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

    pub fn with_body(self, body: DimensionBody) -> Self {
        let mut self_mut = self;
        self_mut.body = Some(body);
        self_mut
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(DimensionBody);

impl_has_members_for!(DimensionBody);

impl_has_source_span_for!(DimensionBody);

impl_maybe_incomplete_for!(DimensionBody; over members);

impl_validate_for_annotations_and_members!(DimensionBody);

impl References for DimensionBody {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.identity().referenced_types(names);
        self.parents().for_each(|m| m.referenced_types(names));
        self.members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_annotations(names));
    }
}

impl DimensionBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<E>(entity: E) -> Self
    where
        E: Into<DimensionIdentity>,
    {
        Self {
            span: Default::default(),
            annotations: Default::default(),
            identity: entity.into(),
            parents: Default::default(),
            members: Default::default(),
        }
    }

    pub fn with_members<I>(self, members: I) -> Self
    where
        I: IntoIterator<Item = Member>,
    {
        let mut self_mut = self;
        self_mut.members = members.into_iter().collect();
        self_mut
    }

    pub fn with_parents<I>(self, parents: I) -> Self
    where
        I: IntoIterator<Item = DimensionParent>,
    {
        let mut self_mut = self;
        self_mut.parents = parents.into_iter().collect();
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub identity, set_identity => into DimensionIdentity);

    get_and_set_vec!(
        pub
        has has_parents,
        parents_len,
        parents,
        parents_mut,
        add_to_parents,
        extend_parents
            => parents, DimensionParent
    );
}

// ------------------------------------------------------------------------------------------------

impl From<SourceEntity> for DimensionIdentity {
    fn from(value: SourceEntity) -> Self {
        Self::Source(value)
    }
}

impl From<&SourceEntity> for DimensionIdentity {
    fn from(value: &SourceEntity) -> Self {
        Self::Source(value.clone())
    }
}

impl From<Member> for DimensionIdentity {
    fn from(value: Member) -> Self {
        Self::Identity(value)
    }
}

impl From<&Member> for DimensionIdentity {
    fn from(value: &Member) -> Self {
        Self::Identity(value.clone())
    }
}

impl References for DimensionIdentity {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match self {
            DimensionIdentity::Source(v) => v.referenced_types(names),
            DimensionIdentity::Identity(v) => v.referenced_types(names),
        }
    }
}

impl DimensionIdentity {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Source(SourceEntity) => is_source_entity, as_source_entity);
    is_as_variant!(Identity(Member) => is_identity_member, as_identity_member);
}

// ------------------------------------------------------------------------------------------------

impl_has_name_for!(DimensionParent);

impl_has_source_span_for!(DimensionParent);

impl_has_optional_body_for!(DimensionParent, AnnotationOnlyBody);

impl References for DimensionParent {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        names.insert(self.target_entity());
    }
}

impl DimensionParent {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<I>(name: Identifier, target_entity: I) -> Self
    where
        I: Into<IdentifierReference>,
    {
        Self {
            span: Default::default(),
            name,
            target_entity: target_entity.into(),
            body: Default::default(),
        }
    }

    pub fn with_body(self, body: AnnotationOnlyBody) -> Self {
        let mut self_mut = self;
        self_mut.body = Some(body);
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub target_entity, set_target_entity => into IdentifierReference);
}

// ------------------------------------------------------------------------------------------------

impl From<IdentifierReference> for SourceEntity {
    fn from(value: IdentifierReference) -> Self {
        Self::new(value)
    }
}

impl From<&IdentifierReference> for SourceEntity {
    fn from(value: &IdentifierReference) -> Self {
        Self::new(value.clone())
    }
}

impl_has_source_span_for!(SourceEntity);

impl References for SourceEntity {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        names.insert(self.target_entity());
    }
}

impl SourceEntity {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<T>(target_entity: T) -> Self
    where
        T: Into<IdentifierReference>,
    {
        Self {
            span: Default::default(),
            target_entity: target_entity.into(),
            with_members: Default::default(),
        }
    }

    pub fn with_members<I>(self, members: I) -> Self
    where
        I: IntoIterator<Item = Identifier>,
    {
        let mut self_mut = self;
        self_mut.with_members = members.into_iter().collect();
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub target_entity, set_target_entity => into IdentifierReference);

    get_and_set_vec!(
        pub
        has has_members,
        members_len,
        members,
        members_mut,
        add_to_members,
        extend_members
            => with_members, Identifier
    );
}
