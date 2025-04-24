use crate::{
    load::ModuleLoader,
    model::{
        annotations::{Annotation, AnnotationBuilder, AnnotationProperty, HasAnnotations},
        check::{validate_multiple_method_duplicates, MaybeIncomplete, Validate},
        definitions::{FromDefinition, HasMultiMembers, HasOptionalFromDefinition},
        identifiers::{Identifier, IdentifierReference},
        members::Member,
        modules::Module,
        values::Value,
        HasName, HasOptionalBody, HasSourceSpan, References, Span,
    },
    store::ModuleStore,
};
use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Definitions ❱ Entities
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
    identity: Member,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    from: Option<FromDefinition>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    members: BTreeMap<Identifier, Member>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ EntityDef
// ------------------------------------------------------------------------------------------------

impl HasName for EntityDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasOptionalBody for EntityDef {
    type Body = EntityBody;

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

impl HasSourceSpan for EntityDef {
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

impl References for EntityDef {
    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        if let Some(inner) = &self.body {
            inner.referenced_annotations(names);
        }
    }

    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        if let Some(inner) = &self.body {
            inner.referenced_types(names);
        }
    }
}

impl AnnotationBuilder for EntityDef {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        let mut self_mut = self;
        if let Some(ref mut inner) = self_mut.body_mut() {
            inner.add_to_annotations(AnnotationProperty::new(predicate.into(), value.into()));
        } else {
            warn!("No body present on model element, could not add annotation property. type: EntityDef, predicate: {}, value: {}", predicate.into(), value.into());
        }
        self_mut
    }
}

impl MaybeIncomplete for EntityDef {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        if let Some(body) = &self.body {
            body.is_incomplete(top, cache)
        } else {
            true
        }
    }
}

impl Validate for EntityDef {
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

impl EntityDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
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
// Implementations ❱ Definitions ❱ EntityBody
// ------------------------------------------------------------------------------------------------

impl HasAnnotations for EntityBody {
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
        self.annotations.extend(extension)
    }
}

impl HasSourceSpan for EntityBody {
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

impl HasOptionalFromDefinition for EntityBody {
    fn from_definition(&self) -> Option<&FromDefinition> {
        self.from.as_ref()
    }

    fn from_definition_mut(&mut self) -> Option<&mut FromDefinition> {
        self.from.as_mut()
    }

    fn set_from_definition(&mut self, from_definition: FromDefinition) {
        self.from = Some(from_definition);
    }

    fn unset_from_definition(&mut self) {
        self.from = None;
    }
}

impl MaybeIncomplete for EntityBody {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        self.members().any(|elem| elem.is_incomplete(top, cache))
    }
}

impl Validate for EntityBody {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        validate_multiple_method_duplicates(self, top, cache, loader);

        self.identity
            .validate(top, cache, loader, check_constraints);
        for annotation in &self.annotations {
            annotation.validate(top, cache, loader, check_constraints);
        }
        for member in self.members() {
            member.validate(top, cache, loader, check_constraints);
        }
    }
}

impl References for EntityBody {
    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.identity().referenced_annotations(names);
        self.members().for_each(|m| m.referenced_annotations(names));
    }

    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.identity().referenced_types(names);
        self.members().for_each(|m| m.referenced_types(names));
    }
}

impl HasMultiMembers for EntityBody {
    fn has_any_members(&self) -> bool {
        !(self.has_identity() || self.has_members())
    }

    fn contains_any_member(&self, name: &Identifier) -> bool {
        self.contains_identity(name) || self.contains_member(name)
    }

    fn all_member_count(&self) -> usize {
        self.identity_count() + self.members.len()
    }

    fn all_member_names(&self) -> impl Iterator<Item = &Identifier> {
        self.identity_names().chain(self.member_names())
    }
}

impl EntityBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(identity: Member) -> Self {
        Self {
            span: None,
            identity,
            annotations: Default::default(),
            from: Default::default(),
            members: Default::default(),
        }
    }

    pub fn with_members<I>(self, members: I) -> Self
    where
        I: IntoIterator<Item = Member>,
    {
        let mut self_mut = self;
        self_mut.extend_members(members);
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn identity(&self) -> &Member {
        &self.identity
    }

    pub fn set_identity(&mut self, identity: Member) {
        self.identity = identity;
    }

    #[inline(always)]
    const fn has_identity(&self) -> bool {
        true
    }

    #[inline(always)]
    const fn identity_count(&self) -> usize {
        1
    }

    #[inline(always)]
    fn identity_names(&self) -> impl Iterator<Item = &Identifier> {
        std::iter::once(self.identity.name())
    }

    #[inline(always)]
    fn contains_identity(&self, name: &Identifier) -> bool {
        self.identity.name() == name
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

    pub fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Member>,
    {
        self.members.extend(
            extension
                .into_iter()
                .map(|elem| (elem.name().clone(), elem)),
        )
    }
}
