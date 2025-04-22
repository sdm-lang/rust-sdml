use crate::{
    load::ModuleLoader,
    model::{
        annotations::{Annotation, AnnotationBuilder, AnnotationProperty, HasAnnotations},
        check::{validate_multiple_method_duplicates, MaybeIncomplete, Validate},
        definitions::{FromDefinition, HasMultiMembers, HasOptionalFromDefinition, SourceEntity},
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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `event_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EventDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<EventBody>,
}

/// Corresponds to the grammar rule `event_body`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EventBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    source_entity: SourceEntity,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    from: Option<FromDefinition>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    members: BTreeMap<Identifier, Member>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ EventDef
// ------------------------------------------------------------------------------------------------

impl HasName for EventDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasOptionalBody for EventDef {
    type Body = EventBody;

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

impl References for EventDef {
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

impl HasSourceSpan for EventDef {
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

impl MaybeIncomplete for EventDef {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        if let Some(body) = &self.body {
            body.is_incomplete(top, cache)
        } else {
            true
        }
    }
}

impl AnnotationBuilder for EventDef {
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

impl Validate for EventDef {
    fn validate(
        &self,
        top: &crate::model::modules::Module,
        cache: &impl crate::store::ModuleStore,
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

impl EventDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name: Identifier) -> Self {
        Self {
            span: Default::default(),
            name,
            body: Default::default(),
        }
    }

    pub fn with_body(self, body: EventBody) -> Self {
        let mut self_mut = self;
        self_mut.body = Some(body);
        self_mut
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ EventBody
// ------------------------------------------------------------------------------------------------

impl HasAnnotations for EventBody {
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

impl HasSourceSpan for EventBody {
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

impl HasOptionalFromDefinition for EventBody {
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

impl MaybeIncomplete for EventBody {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        self.members().any(|elem| elem.is_incomplete(top, cache))
    }
}

impl Validate for EventBody {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        validate_multiple_method_duplicates(self, top, cache, loader);

        self.source_entity()
            .validate(top, cache, loader, check_constraints);
        self.annotations()
            .for_each(|a| a.validate(top, cache, loader, check_constraints));
        self.members()
            .for_each(|m| m.validate(top, cache, loader, check_constraints));
    }
}

impl References for EventBody {
    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.source_entity().referenced_types(names);
        self.members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.source_entity().referenced_annotations(names);
        self.members().for_each(|m| m.referenced_annotations(names));
    }
}

impl HasMultiMembers for EventBody {
    fn has_any_members(&self) -> bool {
        !(self.has_source_members() || self.has_members())
    }

    fn contains_any_member(&self, name: &Identifier) -> bool {
        self.contains_source_member(name) || self.contains_member(name)
    }

    fn all_member_count(&self) -> usize {
        self.source_member_count() + self.members.len()
    }

    fn all_member_names(&self) -> impl Iterator<Item = &Identifier> {
        self.source_member_names().chain(self.member_names())
    }
}

impl EventBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    /// Creates a new [`EventBody`] with the provided, and required, [`SourceEntity`].
    pub fn new(source_entity: SourceEntity) -> Self {
        Self {
            span: Default::default(),
            annotations: Default::default(),
            source_entity,
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

    /// Returns a reference to the source entity of this [`EventBody`].
    pub const fn source_entity(&self) -> &SourceEntity {
        &self.source_entity
    }

    /// Sets the source entity of this [`EventBody`].
    pub fn set_source_entity(&mut self, source_entity: SourceEntity) {
        self.source_entity = source_entity;
    }

    /// Returns the has source *members* of this [`EventBody`].
    fn has_source_members(&self) -> bool {
        self.source_entity.has_members()
    }

    /// Returns the source *member* count of this [`EventBody`].
    fn source_member_count(&self) -> usize {
        self.source_entity.member_count()
    }

    fn source_member_names(&self) -> impl Iterator<Item = &Identifier> {
        self.source_entity.members()
    }

    fn contains_source_member(&self, name: &Identifier) -> bool {
        self.source_entity.contains_member(name)
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
