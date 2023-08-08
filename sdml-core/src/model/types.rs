
use super::{
    Annotation, AnnotationProperty, ByReferenceMember, ByValueMember, Cardinality, Constraint,
    Identifier, IdentifierReference, IdentityMember, ModelElement, Span, TypeReference,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `type_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Definition {
    Datatype(DatatypeDef),
    Entity(EntityDef),
    Enum(EnumDef),
    Event(EventDef),
    Structure(StructureDef),
    Union(UnionDef),
    Property(PropertyDef),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `data_type_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DatatypeDef {
    span: Option<Span>,
    name: Identifier,
    /// Corresponds to the grammar rule `data_type_base`.
    base_type: IdentifierReference,
    body: Option<AnnotationOnlyBody>,
}

/// Corresponds to the grammar rule `annotation_only_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AnnotationOnlyBody {
    span: Option<Span>,
    annotations: Vec<Annotation>, // assert!(!annotations.is_empty());
}

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
    identity: IdentityMember,
    annotations: Vec<Annotation>,
    members: Vec<EntityMember>,
    groups: Vec<EntityGroup>,
}

/// Corresponds to the inner part of the grammar rule `entity_group`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum EntityMember {
    ByValue(ByValueMember),
    ByReference(ByReferenceMember),
}

/// Corresponds to the grammar rule `entity_group`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityGroup {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<EntityMember>, // assert!(!members.is_empty());
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `enum_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EnumDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<EnumBody>,
}

/// Corresponds to the grammar rule `enum_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EnumBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    variants: Vec<ValueVariant>, // assert!(!variants.is_empty());
}

/// Corresponds to the grammar rule `enum_variant`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ValueVariant {
    span: Option<Span>,
    name: Identifier,
    value: u32,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `event_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EventDef {
    span: Option<Span>,
    name: Identifier,
    event_source: IdentifierReference,
    body: Option<StructureBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `structure_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<StructureBody>,
}

/// Corresponds to the grammar rule `structure_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>,
    groups: Vec<StructureGroup>,
}

/// Corresponds to the grammar rule `structure_group`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureGroup {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>, // assert!(!members.is_empty());
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `union_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UnionDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<UnionBody>,
}

/// Corresponds to the grammar rule `union_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UnionBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    variants: Vec<TypeVariant>, // assert!(!variants.is_empty());
}

/// Corresponds to the grammar rule `type_variant`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TypeVariant {
    span: Option<Span>,
    name: IdentifierReference,
    rename: Option<Identifier>,
    body: Option<AnnotationOnlyBody>,
}

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
    target_type: TypeReference,
    inverse_name: Option<Option<Identifier>>,
    target_cardinality: Option<Cardinality>,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Type Definitions
// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(Definition, Datatype, DatatypeDef);

impl_from_for_variant!(Definition, Entity, EntityDef);

impl_from_for_variant!(Definition, Enum, EnumDef);

impl_from_for_variant!(Definition, Event, EventDef);

impl_from_for_variant!(Definition, Structure, StructureDef);

impl_from_for_variant!(Definition, Union, UnionDef);

impl_from_for_variant!(Definition, Property, PropertyDef);

//enum_display_impl!(Definition => Datatype, Entity, Enum, Event, Structure, Union, Property);

impl ModelElement for Definition {
    fn ts_span(&self) -> Option<&Span> {
        match self {
            Self::Datatype(v) => v.ts_span(),
            Self::Entity(v) => v.ts_span(),
            Self::Enum(v) => v.ts_span(),
            Self::Event(v) => v.ts_span(),
            Self::Structure(v) => v.ts_span(),
            Self::Union(v) => v.ts_span(),
            Self::Property(v) => v.ts_span(),
        }
    }

    fn set_ts_span(&mut self, span: Span) {
        match self {
            Self::Datatype(v) => v.set_ts_span(span),
            Self::Entity(v) => v.set_ts_span(span),
            Self::Enum(v) => v.set_ts_span(span),
            Self::Event(v) => v.set_ts_span(span),
            Self::Structure(v) => v.set_ts_span(span),
            Self::Union(v) => v.set_ts_span(span),
            Self::Property(v) => v.set_ts_span(span),
        }
    }

    fn unset_ts_span(&mut self) {
        match self {
            Self::Datatype(v) => v.unset_ts_span(),
            Self::Entity(v) => v.unset_ts_span(),
            Self::Enum(v) => v.unset_ts_span(),
            Self::Event(v) => v.unset_ts_span(),
            Self::Structure(v) => v.unset_ts_span(),
            Self::Union(v) => v.unset_ts_span(),
            Self::Property(v) => v.unset_ts_span(),
        }
    }

    fn name(&self) -> &Identifier {
        match self {
            Self::Datatype(v) => v.name(),
            Self::Entity(v) => v.name(),
            Self::Enum(v) => v.name(),
            Self::Event(v) => v.name(),
            Self::Structure(v) => v.name(),
            Self::Union(v) => v.name(),
            Self::Property(v) => v.name(),
        }
    }

    fn set_name(&mut self, name: Identifier) {
        match self {
            Self::Datatype(v) => v.set_name(name),
            Self::Entity(v) => v.set_name(name),
            Self::Enum(v) => v.set_name(name),
            Self::Event(v) => v.set_name(name),
            Self::Structure(v) => v.set_name(name),
            Self::Union(v) => v.set_name(name),
            Self::Property(v) => v.set_name(name),
        }
    }

    fn is_complete(&self) -> bool {
        match self {
            Self::Datatype(v) => v.is_complete(),
            Self::Entity(v) => v.is_complete(),
            Self::Enum(v) => v.is_complete(),
            Self::Event(v) => v.is_complete(),
            Self::Structure(v) => v.is_complete(),
            Self::Union(v) => v.is_complete(),
            Self::Property(v) => v.is_complete(),
        }
    }

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        match self {
            Self::Datatype(v) => v.referenced_types(),
            Self::Entity(v) => v.referenced_types(),
            Self::Enum(v) => v.referenced_types(),
            Self::Event(v) => v.referenced_types(),
            Self::Structure(v) => v.referenced_types(),
            Self::Union(v) => v.referenced_types(),
            Self::Property(v) => v.referenced_types(),
        }
    }

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        match self {
            Self::Datatype(v) => v.referenced_annotations(),
            Self::Entity(v) => v.referenced_annotations(),
            Self::Enum(v) => v.referenced_annotations(),
            Self::Event(v) => v.referenced_annotations(),
            Self::Structure(v) => v.referenced_annotations(),
            Self::Union(v) => v.referenced_annotations(),
            Self::Property(v) => v.referenced_annotations(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

impl ModelElement for DatatypeDef {
    fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    fn name(&self) -> &Identifier {
        &self.name
    }
    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    fn is_complete(&self) -> bool {
        true
    }

    // --------------------------------------------------------------------------------------------

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        [self.base_type()].into_iter().collect()
    }

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }
}

impl DatatypeDef {
    pub fn new(name: Identifier, base_type: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            base_type,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn base_type(&self) -> &IdentifierReference {
        &self.base_type
    }
    pub fn set_base_type(&mut self, base_type: IdentifierReference) {
        self.base_type = base_type;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    pub fn body(&self) -> Option<&AnnotationOnlyBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: AnnotationOnlyBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl AnnotationOnlyBody {
    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    pub fn annotations_len(&self) -> usize {
        self.annotations.len()
    }
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }
    pub fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }
    pub fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }
    pub fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension)
    }

    pub fn annotation_properties(&self) -> impl Iterator<Item = &AnnotationProperty> {
        self.annotations()
            .filter_map(|a| a.as_annotation_property())
    }

    pub fn annotation_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.annotations().filter_map(|a| a.as_constraint())
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.annotation_properties().map(|a| a.name()).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

impl ModelElement for EntityDef {
    fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    fn name(&self) -> &Identifier {
        &self.name
    }
    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    // --------------------------------------------------------------------------------------------

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_types())
            .unwrap_or_default()
    }
}

impl EntityDef {
    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    pub fn body(&self) -> Option<&EntityBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: EntityBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl EntityBody {
    pub fn new(identity: IdentityMember) -> Self {
        Self {
            span: None,
            identity,
            annotations: Default::default(),
            members: Default::default(),
            groups: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    pub fn annotations_len(&self) -> usize {
        self.annotations.len()
    }
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }
    pub fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }
    pub fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }
    pub fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension)
    }

    pub fn annotation_properties(&self) -> impl Iterator<Item = &AnnotationProperty> {
        self.annotations()
            .filter_map(|a| a.as_annotation_property())
    }

    pub fn annotation_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.annotations().filter_map(|a| a.as_constraint())
    }

    // --------------------------------------------------------------------------------------------

    pub fn identity(&self) -> &IdentityMember {
        &self.identity
    }
    pub fn set_identity(&mut self, identity: IdentityMember) {
        self.identity = identity;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_members(&self) -> bool {
        !self.members.is_empty()
    }
    pub fn members_len(&self) -> usize {
        self.members.len()
    }
    pub fn members(&self) -> impl Iterator<Item = &EntityMember> {
        self.members.iter()
    }
    pub fn members_mut(&mut self) -> impl Iterator<Item = &mut EntityMember> {
        self.members.iter_mut()
    }
    pub fn add_to_members<I>(&mut self, value: I)
    where
        I: Into<EntityMember>,
    {
        self.members.push(value.into())
    }
    pub fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = EntityMember>,
    {
        self.members.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_groups(&self) -> bool {
        !self.groups.is_empty()
    }
    pub fn groups_len(&self) -> usize {
        self.groups.len()
    }
    pub fn groups(&self) -> impl Iterator<Item = &EntityGroup> {
        self.groups.iter()
    }
    pub fn groups_mut(&mut self) -> impl Iterator<Item = &mut EntityGroup> {
        self.groups.iter_mut()
    }
    pub fn add_to_groups<I>(&mut self, value: I)
    where
        I: Into<EntityGroup>,
    {
        self.groups.push(value.into())
    }
    pub fn extend_groups<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = EntityGroup>,
    {
        self.groups.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn flat_members(&self) -> impl Iterator<Item = &EntityMember> {
        self.members()
            .chain(self.groups().flat_map(|g| g.members()))
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete()) && self.groups().all(|m| m.is_complete())
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.flat_members()
            .flat_map(|m| m.referenced_annotations())
            .collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.flat_members()
            .flat_map(|m| m.referenced_types())
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(EntityMember, ByValue, ByValueMember);
impl_from_for_variant!(EntityMember, ByReference, ByReferenceMember);

impl EntityMember {
    pub fn is_by_value(&self) -> bool {
        matches!(self, Self::ByValue(_))
    }
    pub fn as_by_value(&self) -> Option<&ByValueMember> {
        match self {
            Self::ByValue(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_by_reference(&self) -> bool {
        matches!(self, Self::ByReference(_))
    }
    pub fn as_by_reference(&self) -> Option<&ByReferenceMember> {
        match self {
            Self::ByReference(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn name(&self) -> &Identifier {
        match self {
            Self::ByValue(v) => v.name(),
            Self::ByReference(v) => v.name(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn target_type(&self) -> Option<&IdentifierReference> {
        match self {
            Self::ByValue(v) => v.target_type(),
            Self::ByReference(v) => v.target_type(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        match self {
            Self::ByValue(v) => v.is_complete(),
            Self::ByReference(v) => v.is_complete(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        match self {
            Self::ByValue(v) => v.referenced_annotations(),
            Self::ByReference(v) => v.referenced_annotations(),
        }
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        match self {
            Self::ByValue(v) => v.referenced_types(),
            Self::ByReference(v) => v.referenced_types(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl EntityGroup {
    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    pub fn annotations_len(&self) -> usize {
        self.annotations.len()
    }
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }
    pub fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }
    pub fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }
    pub fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension)
    }

    pub fn annotation_properties(&self) -> impl Iterator<Item = &AnnotationProperty> {
        self.annotations()
            .filter_map(|a| a.as_annotation_property())
    }

    pub fn annotation_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.annotations().filter_map(|a| a.as_constraint())
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_members(&self) -> bool {
        !self.members.is_empty()
    }
    pub fn members_len(&self) -> usize {
        self.members.len()
    }
    pub fn members(&self) -> impl Iterator<Item = &EntityMember> {
        self.members.iter()
    }
    pub fn members_mut(&mut self) -> impl Iterator<Item = &mut EntityMember> {
        self.members.iter_mut()
    }
    pub fn add_to_members(&mut self, value: EntityMember) {
        self.members.push(value)
    }
    pub fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = EntityMember>,
    {
        self.members.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete())
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.annotation_properties()
            .map(|p| p.name())
            .chain(self.annotation_properties().map(|a| a.name()))
            .collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.members().flat_map(|m| m.referenced_types()).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

impl ModelElement for EnumDef {
    fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    fn name(&self) -> &Identifier {
        &self.name
    }
    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    // --------------------------------------------------------------------------------------------

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        Default::default()
    }
}

impl EnumDef {
    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    pub fn body(&self) -> Option<&EnumBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: EnumBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl EnumBody {
    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    pub fn annotations_len(&self) -> usize {
        self.annotations.len()
    }
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }
    pub fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }
    pub fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }
    pub fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension)
    }

    pub fn annotation_properties(&self) -> impl Iterator<Item = &AnnotationProperty> {
        self.annotations()
            .filter_map(|a| a.as_annotation_property())
    }

    pub fn annotation_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.annotations().filter_map(|a| a.as_constraint())
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }
    pub fn variants_len(&self) -> usize {
        self.variants.len()
    }
    pub fn variants(&self) -> impl Iterator<Item = &ValueVariant> {
        self.variants.iter()
    }
    pub fn variants_mut(&mut self) -> impl Iterator<Item = &mut ValueVariant> {
        self.variants.iter_mut()
    }
    pub fn add_to_variants(&mut self, value: ValueVariant) {
        self.variants.push(value)
    }
    pub fn extend_variants<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = ValueVariant>,
    {
        self.variants.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        let mut body: HashSet<&IdentifierReference> = self
            .annotations()
            .filter_map(|a| {
                if let Annotation::Property(aprop) = a {
                    Some(aprop.name())
                } else {
                    None
                }
            })
            .collect();
        let variants: HashSet<&IdentifierReference> = self
            .variants()
            .flat_map(|v| v.referenced_annotations())
            .collect();
        body.extend(variants);
        body
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        Default::default()
    }
}

// ------------------------------------------------------------------------------------------------

impl ValueVariant {
    pub fn new(name: Identifier, value: u32) -> Self {
        Self {
            span: None,
            name,
            value,
            body: None,
        }
    }

    pub fn new_with(name: Identifier, value: u32, body: AnnotationOnlyBody) -> Self {
        Self {
            span: None,
            name,
            value,
            body: Some(body),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn name(&self) -> &Identifier {
        &self.name
    }
    pub fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    pub fn value(&self) -> u32 {
        self.value
    }
    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    pub fn body(&self) -> Option<&AnnotationOnlyBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: AnnotationOnlyBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

impl ModelElement for EventDef {
    fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    fn name(&self) -> &Identifier {
        &self.name
    }
    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    // --------------------------------------------------------------------------------------------

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_types())
            .unwrap_or_default()
    }
}

impl EventDef {
    pub fn new(name: Identifier, event_source: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            event_source,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn event_source(&self) -> &IdentifierReference {
        &self.event_source
    }
    pub fn set_event_source(&mut self, event_source: IdentifierReference) {
        self.event_source = event_source;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    pub fn body(&self) -> Option<&StructureBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: StructureBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

impl ModelElement for StructureDef {
    fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    fn name(&self) -> &Identifier {
        &self.name
    }
    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    fn is_complete(&self) -> bool {
        self.body().is_some()
    }

    // --------------------------------------------------------------------------------------------

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_types())
            .unwrap_or_default()
    }
}

impl StructureDef {
    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    pub fn body(&self) -> Option<&StructureBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: StructureBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl StructureBody {
    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    pub fn annotations_len(&self) -> usize {
        self.annotations.len()
    }
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }
    pub fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }
    pub fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }
    pub fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension)
    }

    pub fn annotation_properties(&self) -> impl Iterator<Item = &AnnotationProperty> {
        self.annotations()
            .filter_map(|a| a.as_annotation_property())
    }

    pub fn annotation_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.annotations().filter_map(|a| a.as_constraint())
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_members(&self) -> bool {
        !self.members.is_empty()
    }
    pub fn members_len(&self) -> usize {
        self.members.len()
    }
    pub fn members(&self) -> impl Iterator<Item = &ByValueMember> {
        self.members.iter()
    }
    pub fn members_mut(&mut self) -> impl Iterator<Item = &mut ByValueMember> {
        self.members.iter_mut()
    }
    pub fn add_to_members(&mut self, value: ByValueMember) {
        self.members.push(value)
    }
    pub fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = ByValueMember>,
    {
        self.members.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_groups(&self) -> bool {
        !self.groups.is_empty()
    }
    pub fn groups_len(&self) -> usize {
        self.groups.len()
    }
    pub fn groups(&self) -> impl Iterator<Item = &StructureGroup> {
        self.groups.iter()
    }
    pub fn groups_mut(&mut self) -> impl Iterator<Item = &mut StructureGroup> {
        self.groups.iter_mut()
    }
    pub fn add_to_groups(&mut self, value: StructureGroup) {
        self.groups.push(value)
    }
    pub fn extend_groups<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = StructureGroup>,
    {
        self.groups.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn flat_members(&self) -> impl Iterator<Item = &ByValueMember> {
        self.members()
            .chain(self.groups().flat_map(|g| g.members()))
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete()) && self.groups().all(|m| m.is_complete())
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.flat_members()
            .flat_map(|m| m.referenced_annotations())
            .collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.flat_members()
            .flat_map(|m| m.referenced_types())
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl StructureGroup {
    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    pub fn annotations_len(&self) -> usize {
        self.annotations.len()
    }
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }
    pub fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }
    pub fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }
    pub fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension)
    }

    pub fn annotation_properties(&self) -> impl Iterator<Item = &AnnotationProperty> {
        self.annotations()
            .filter_map(|a| a.as_annotation_property())
    }

    pub fn annotation_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.annotations().filter_map(|a| a.as_constraint())
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_members(&self) -> bool {
        !self.members.is_empty()
    }
    pub fn members_len(&self) -> usize {
        self.members.len()
    }
    pub fn members(&self) -> impl Iterator<Item = &ByValueMember> {
        self.members.iter()
    }
    pub fn members_mut(&mut self) -> impl Iterator<Item = &mut ByValueMember> {
        self.members.iter_mut()
    }
    pub fn add_to_members(&mut self, value: ByValueMember) {
        self.members.push(value)
    }
    pub fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = ByValueMember>,
    {
        self.members.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete())
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.annotation_properties()
            .map(|p| p.name())
            .chain(self.annotation_properties().map(|a| a.name()))
            .collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.members().flat_map(|m| m.referenced_types()).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

impl ModelElement for UnionDef {
    fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    fn name(&self) -> &Identifier {
        &self.name
    }
    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    // --------------------------------------------------------------------------------------------

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_types())
            .unwrap_or_default()
    }
}

impl UnionDef {
    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    pub fn body(&self) -> Option<&UnionBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: UnionBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl UnionBody {
    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    pub fn annotations_len(&self) -> usize {
        self.annotations.len()
    }
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }
    pub fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }
    pub fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }
    pub fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension)
    }

    pub fn annotation_properties(&self) -> impl Iterator<Item = &AnnotationProperty> {
        self.annotations()
            .filter_map(|a| a.as_annotation_property())
    }

    pub fn annotation_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.annotations().filter_map(|a| a.as_constraint())
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }
    pub fn variants_len(&self) -> usize {
        self.variants.len()
    }
    pub fn variants(&self) -> impl Iterator<Item = &TypeVariant> {
        self.variants.iter()
    }
    pub fn variants_mut(&mut self) -> impl Iterator<Item = &mut TypeVariant> {
        self.variants.iter_mut()
    }
    pub fn add_to_variants(&mut self, value: TypeVariant) {
        self.variants.push(value)
    }
    pub fn extend_variants<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = TypeVariant>,
    {
        self.variants.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.variants().map(|v| v.name()).collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl TypeVariant {
    pub fn new(name: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            rename: None,
            body: None,
        }
    }

    pub fn new_with(name: IdentifierReference, body: AnnotationOnlyBody) -> Self {
        Self {
            span: None,
            name,
            rename: None,
            body: Some(body),
        }
    }

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    pub fn with_rename(self, rename: Identifier) -> Self {
        Self {
            rename: Some(rename),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body().is_some()
    }
    pub fn body(&self) -> Option<&AnnotationOnlyBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: AnnotationOnlyBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn name(&self) -> &IdentifierReference {
        &self.name
    }
    pub fn set_name(&mut self, name: IdentifierReference) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_rename(&self) -> bool {
        self.body().is_some()
    }
    pub fn rename(&self) -> Option<&Identifier> {
        self.rename.as_ref()
    }
    pub fn set_rename(&mut self, rename: Identifier) {
        self.rename = Some(rename);
    }
    pub fn unset_rename(&mut self) {
        self.rename = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Properties
// ------------------------------------------------------------------------------------------------

impl ModelElement for PropertyDef {
    fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    fn name(&self) -> &Identifier {
        &self.name
    }
    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    // --------------------------------------------------------------------------------------------

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_types())
            .unwrap_or_default()
    }
}

impl PropertyDef {
    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    pub fn body(&self) -> Option<&PropertyBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: PropertyBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl PropertyBody {
    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }
    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    pub fn annotations_len(&self) -> usize {
        self.annotations.len()
    }
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }
    pub fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }
    pub fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }
    pub fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension)
    }

    pub fn annotation_properties(&self) -> impl Iterator<Item = &AnnotationProperty> {
        self.annotations()
            .filter_map(|a| a.as_annotation_property())
    }

    pub fn annotation_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.annotations().filter_map(|a| a.as_constraint())
    }

    // --------------------------------------------------------------------------------------------

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

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.annotation_properties().map(|a| a.name()).collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.roles()
            .flat_map(|role| role.referenced_types())
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl PropertyRole {
    pub fn new(name: Identifier, target_type: TypeReference) -> Self {
        Self {
            span: None,
            name,
            target_type,
            inverse_name: Default::default(),
            target_cardinality: Default::default(),
            body: None,
        }
    }

    pub fn new_unknown(name: Identifier) -> Self {
        Self::new(name, TypeReference::Unknown)
    }

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    pub fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    pub fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    pub fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn name(&self) -> &Identifier {
        &self.name
    }
    pub fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    pub fn target_type(&self) -> &TypeReference {
        &self.target_type
    }
    pub fn set_target_type(&mut self, target_type: TypeReference) {
        self.target_type = target_type;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_inverse_name(&self) -> bool {
        self.inverse_name().is_some()
    }
    pub fn inverse_name(&self) -> Option<&Option<Identifier>> {
        self.inverse_name.as_ref()
    }
    pub fn set_inverse_name(&mut self, inverse_name: Option<Identifier>) {
        self.inverse_name = Some(inverse_name);
    }
    pub fn unset_inverse_name(&mut self) {
        self.inverse_name = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_target_cardinality(&self) -> bool {
        self.target_cardinality().is_some()
    }
    pub fn target_cardinality(&self) -> Option<&Cardinality> {
        self.target_cardinality.as_ref()
    }
    pub fn set_target_cardinality(&mut self, target_cardinality: Cardinality) {
        self.target_cardinality = Some(target_cardinality);
    }
    pub fn unset_target_cardinality(&mut self) {
        self.target_cardinality = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body().is_some()
    }
    pub fn body(&self) -> Option<&AnnotationOnlyBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: AnnotationOnlyBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
