use crate::{
    load::ModuleLoader,
    model::{
        annotations::{
            Annotation, AnnotationBuilder, AnnotationOnlyBody, AnnotationProperty, HasAnnotations,
        },
        check::{find_definition, MaybeIncomplete, Validate},
        identifiers::{Identifier, IdentifierReference},
        members::Member,
        modules::Module,
        values::Value,
        HasName, HasOptionalBody, HasSourceSpan, References, Span,
    },
    stdlib::is_builtin_type_name,
    store::ModuleStore,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

use sdml_errors::diagnostics::functions::{
    dimension_parent_not_entity, source_entity_missing_member, source_entity_not_entity,
    type_definition_not_found, IdentifierCaseConvention,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Definitions ❱ Dimensions
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    parents: BTreeMap<Identifier, DimensionParent>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    members: BTreeMap<Identifier, Member>,
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
// Implementations ❱ Definitions ❱ DimensionDef
// ------------------------------------------------------------------------------------------------

impl HasName for DimensionDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasOptionalBody for DimensionDef {
    type Body = DimensionBody;

    fn body(&self) -> Option<&Self::Body> {
        self.body.as_ref()
    }

    fn body_mut(&mut self) -> Option<&mut Self::Body> {
        self.body.as_mut()
    }

    fn set_body(&mut self, body: Self::Body) {
        self.body = Some(body);
    }

    fn unset_body(&mut self) {
        self.body = None;
    }
}

impl HasSourceSpan for DimensionDef {
    fn with_source_span(self, span: Span) -> Self {
        let mut self_mut = self;
        self_mut.span = Some(span);
        self_mut
    }

    fn source_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn set_source_span(&mut self, span: Span) {
        self.span = Some(span);
    }

    fn unset_source_span(&mut self) {
        self.span = None;
    }
}

impl References for DimensionDef {
    fn referenced_annotations<'a>(
        &'a self,
        names: &mut ::std::collections::BTreeSet<&'a IdentifierReference>,
    ) {
        if let Some(inner) = &self.body {
            inner.referenced_annotations(names);
        }
    }

    fn referenced_types<'a>(
        &'a self,
        names: &mut ::std::collections::BTreeSet<&'a IdentifierReference>,
    ) {
        if let Some(inner) = &self.body {
            inner.referenced_types(names);
        }
    }
}

impl AnnotationBuilder for DimensionDef {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        let mut self_mut = self;
        if let Some(ref mut inner) = self_mut.body {
            inner.add_to_annotations(AnnotationProperty::new(predicate.into(), value.into()));
        }
        self_mut
    }
}

impl MaybeIncomplete for DimensionDef {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        if let Some(body) = &self.body {
            body.is_incomplete(top, cache)
        } else {
            true
        }
    }
}

impl Validate for DimensionDef {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
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
// Implementations ❱ Definitions ❱ DimensionBody
// ------------------------------------------------------------------------------------------------

impl HasAnnotations for DimensionBody {
    fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }

    fn annotation_count(&self) -> usize {
        self.annotations.len()
    }

    fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }

    fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }

    fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }

    fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension.into_iter())
    }
}

impl HasSourceSpan for DimensionBody {
    fn with_source_span(self, span: Span) -> Self {
        let mut self_mut = self;
        self_mut.span = Some(span);
        self_mut
    }

    fn source_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn set_source_span(&mut self, span: Span) {
        self.span = Some(span);
    }

    fn unset_source_span(&mut self) {
        self.span = None;
    }
}

impl MaybeIncomplete for DimensionBody {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        self.identity().is_incomplete(top, cache)
            || self.members().any(|elem| elem.is_incomplete(top, cache))
    }
}

impl Validate for DimensionBody {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.annotations()
            .for_each(|a| a.validate(top, cache, loader, check_constraints));
        self.identity()
            .validate(top, cache, loader, check_constraints);
        self.parents()
            .for_each(|m| m.validate(top, cache, loader, check_constraints));
        self.members()
            .for_each(|m| m.validate(top, cache, loader, check_constraints));
    }
}

impl References for DimensionBody {
    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.identity().referenced_types(names);
        self.parents().for_each(|m| m.referenced_types(names));
        self.members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
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
        self_mut.members = members
            .into_iter()
            .map(|mem| (mem.name().clone(), mem))
            .collect();
        self_mut
    }

    pub fn with_parents<I>(self, parents: I) -> Self
    where
        I: IntoIterator<Item = DimensionParent>,
    {
        let mut self_mut = self;
        self_mut.parents = parents
            .into_iter()
            .map(|mem| (mem.name().clone(), mem))
            .collect();
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn identity(&self) -> &DimensionIdentity {
        &self.identity
    }

    pub fn set_identity<T>(&mut self, identity: T)
    where
        T: Into<DimensionIdentity>,
    {
        self.identity = identity.into();
    }

    // --------------------------------------------------------------------------------------------
    // Members
    // --------------------------------------------------------------------------------------------

    pub fn has_members(&self) -> bool {
        !self.members.is_empty()
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    pub fn contains_member(&self, name: &Identifier) -> bool {
        self.members.contains_key(name)
    }

    pub fn member(&self, name: &Identifier) -> Option<&Member> {
        self.members.get(name)
    }

    pub fn member_mut(&mut self, name: &Identifier) -> Option<&mut Member> {
        self.members.get_mut(name)
    }

    pub fn members(&self) -> impl Iterator<Item = &Member> {
        self.members.values()
    }

    pub fn members_mut(&mut self) -> impl Iterator<Item = &mut Member> {
        self.members.values_mut()
    }

    pub fn member_names(&self) -> impl Iterator<Item = &Identifier> {
        self.members.keys()
    }

    pub fn add_to_members(&mut self, value: Member) -> Option<Member> {
        self.members.insert(value.name().clone(), value)
    }

    pub fn extend_member<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Member>,
    {
        self.members.extend(
            extension
                .into_iter()
                .map(|elem| (elem.name().clone(), elem)),
        )
    }

    // --------------------------------------------------------------------------------------------
    // Parents
    // --------------------------------------------------------------------------------------------

    pub fn has_parents(&self) -> bool {
        !self.parents.is_empty()
    }

    pub fn parent_count(&self) -> usize {
        self.parents.len()
    }

    pub fn contains_parent(&self, name: &Identifier) -> bool {
        self.parents.contains_key(name)
    }

    pub fn parent(&self, name: &Identifier) -> Option<&DimensionParent> {
        self.parents.get(name)
    }

    pub fn parent_mut(&mut self, name: &Identifier) -> Option<&mut DimensionParent> {
        self.parents.get_mut(name)
    }

    pub fn parents(&self) -> impl Iterator<Item = &DimensionParent> {
        self.parents.values()
    }

    pub fn parents_mut(&mut self) -> impl Iterator<Item = &mut DimensionParent> {
        self.parents.values_mut()
    }

    pub fn parent_names(&self) -> impl Iterator<Item = &Identifier> {
        self.parents.keys()
    }

    pub fn add_to_parents(&mut self, value: DimensionParent) -> Option<DimensionParent> {
        self.parents.insert(value.name().clone(), value)
    }

    pub fn extend_parents<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = DimensionParent>,
    {
        self.parents.extend(
            extension
                .into_iter()
                .map(|elem| (elem.name().clone(), elem)),
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ DimensionIdentity
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
    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        match self {
            DimensionIdentity::Source(v) => v.referenced_types(names),
            DimensionIdentity::Identity(v) => v.referenced_types(names),
        }
    }
}

impl MaybeIncomplete for DimensionIdentity {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        match self {
            Self::Source(_) => false,
            Self::Identity(member) => member.is_incomplete(top, cache),
        }
    }
}

impl Validate for DimensionIdentity {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        match self {
            Self::Source(src) => src.validate(top, cache, loader, check_constraints),
            Self::Identity(member) => member.validate(top, cache, loader, check_constraints),
        }
    }
}

impl DimensionIdentity {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_source_entity(&self) -> bool {
        matches!(self, Self::Source(_))
    }

    pub const fn as_source_entity(&self) -> Option<&SourceEntity> {
        match self {
            Self::Source(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_identity_member(&self) -> bool {
        matches!(self, Self::Identity(_))
    }

    pub const fn as_identity_member(&self) -> Option<&Member> {
        match self {
            Self::Identity(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ DimensionParent
// ------------------------------------------------------------------------------------------------

impl HasName for DimensionParent {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasSourceSpan for DimensionParent {
    fn with_source_span(self, span: Span) -> Self {
        let mut self_mut = self;
        self_mut.span = Some(span);
        self_mut
    }

    fn source_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn set_source_span(&mut self, span: Span) {
        self.span = Some(span);
    }

    fn unset_source_span(&mut self) {
        self.span = None;
    }
}

impl HasOptionalBody for DimensionParent {
    type Body = AnnotationOnlyBody;

    fn body(&self) -> Option<&Self::Body> {
        self.body.as_ref()
    }

    fn body_mut(&mut self) -> Option<&mut Self::Body> {
        self.body.as_mut()
    }

    fn set_body(&mut self, body: Self::Body) {
        self.body = Some(body);
    }

    fn unset_body(&mut self) {
        self.body = None;
    }
}

impl References for DimensionParent {
    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        names.insert(self.target_entity());
    }
}

impl Validate for DimensionParent {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        _: bool,
    ) {
        let name = self.target_entity();
        if let Some(defn) = find_definition(name, top, cache) {
            if !defn.is_entity() {
                loader
                    .report(&dimension_parent_not_entity(
                        top.file_id().copied().unwrap_or_default(),
                        name.source_span().as_ref().map(|span| (*span).into()),
                        name,
                    ))
                    .unwrap();
            }
        } else {
            if !name
                .as_identifier()
                .map(is_builtin_type_name)
                .unwrap_or_default()
            {
                loader
                    .report(&type_definition_not_found(
                        top.file_id().copied().unwrap_or_default(),
                        name.source_span().as_ref().map(|span| (*span).into()),
                        name,
                    ))
                    .unwrap();
            } else {
                loader
                    .report(&dimension_parent_not_entity(
                        top.file_id().copied().unwrap_or_default(),
                        name.source_span().as_ref().map(|span| (*span).into()),
                        name,
                    ))
                    .unwrap();
            }
        }
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

    pub const fn target_entity(&self) -> &IdentifierReference {
        &self.target_entity
    }

    pub fn set_target_entity<T>(&mut self, target_entity: T)
    where
        T: Into<IdentifierReference>,
    {
        self.target_entity = target_entity.into();
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ SourceEntity
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

impl HasSourceSpan for SourceEntity {
    fn with_source_span(self, span: Span) -> Self {
        let mut self_mut = self;
        self_mut.span = Some(span);
        self_mut
    }

    fn source_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn set_source_span(&mut self, span: Span) {
        self.span = Some(span);
    }

    fn unset_source_span(&mut self) {
        self.span = None;
    }
}

impl Validate for SourceEntity {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        let name = self.target_entity();
        if let Some(defn) = find_definition(name, top, cache) {
            if let Some(entity) = defn.as_entity() {
                match (self.has_members(), entity.body()) {
                    (true, Some(body)) => {
                        for member in self.members() {
                            if !body.contains_member(member) {
                                loader
                                    .report(&source_entity_missing_member(
                                        top.file_id().copied().unwrap_or_default(),
                                        name.source_span().as_ref().map(|span| (*span).into()),
                                        name,
                                    ))
                                    .unwrap();
                            }
                        }
                    }
                    (true, None) => {
                        for name in self.members() {
                            loader
                                .report(&source_entity_missing_member(
                                    top.file_id().copied().unwrap_or_default(),
                                    name.source_span().as_ref().map(|span| (*span).into()),
                                    name,
                                ))
                                .unwrap();
                        }
                    }
                    (false, _) => (),
                }
            } else {
                loader
                    .report(&source_entity_not_entity(
                        top.file_id().copied().unwrap_or_default(),
                        name.source_span().as_ref().map(|span| (*span).into()),
                        name,
                    ))
                    .unwrap();
            }
        } else {
            if !name
                .as_identifier()
                .map(is_builtin_type_name)
                .unwrap_or_default()
            {
                loader
                    .report(&type_definition_not_found(
                        top.file_id().copied().unwrap_or_default(),
                        name.source_span().as_ref().map(|span| (*span).into()),
                        name,
                    ))
                    .unwrap();
            } else {
                loader
                    .report(&source_entity_not_entity(
                        top.file_id().copied().unwrap_or_default(),
                        name.source_span().as_ref().map(|span| (*span).into()),
                        name,
                    ))
                    .unwrap();
            }
        }
    }
}

impl References for SourceEntity {
    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
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

    pub const fn target_entity(&self) -> &IdentifierReference {
        &self.target_entity
    }

    pub fn set_target_entity<T>(&mut self, target_entity: T)
    where
        T: Into<IdentifierReference>,
    {
        self.target_entity = target_entity.into();
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_members(&self) -> bool {
        !self.with_members.is_empty()
    }

    pub fn member_count(&self) -> usize {
        self.with_members.len()
    }

    pub fn members(&self) -> impl Iterator<Item = &Identifier> {
        self.with_members.iter()
    }

    pub fn members_mut(&mut self) -> impl Iterator<Item = &mut Identifier> {
        self.with_members.iter_mut()
    }

    pub fn add_to_members<I>(&mut self, value: I)
    where
        I: Into<Identifier>,
    {
        self.with_members.push(value.into())
    }

    pub fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Identifier>,
    {
        self.with_members.extend(extension)
    }
}
