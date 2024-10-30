use crate::load::ModuleLoader;
use crate::model::annotations::AnnotationBuilder;
use crate::model::annotations::AnnotationProperty;
use crate::model::check::MaybeIncomplete;
use crate::model::definitions::SourceEntity;
use crate::model::modules::Module;
use crate::model::values::Value;
use crate::model::HasOptionalBody;
use crate::model::HasSourceSpan;
use crate::model::{
    annotations::{Annotation, HasAnnotations},
    check::Validate,
    identifiers::{Identifier, IdentifierReference},
    members::Member,
    HasName, References, Span,
};
use crate::store::ModuleStore;
use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
use std::collections::HashMap;
use std::{collections::HashSet, fmt::Debug};

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
    event_source: SourceEntity,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "HashMap::is_empty"))]
    members: HashMap<Identifier, Member>,
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
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        if let Some(inner) = &self.body {
            inner.referenced_annotations(names);
        }
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
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

    fn annotations_len(&self) -> usize {
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
        // TODO: need to include event_source in validation!!

        self.annotations()
            .for_each(|a| a.validate(top, cache, loader, check_constraints));
        self.members()
            .for_each(|m| m.validate(top, cache, loader, check_constraints));
    }
}

impl References for EventBody {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.members().for_each(|m| m.referenced_annotations(names));
    }
}

impl EventBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(event_source: SourceEntity) -> Self {
        Self {
            span: Default::default(),
            annotations: Default::default(),
            event_source,
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

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn event_source(&self) -> &SourceEntity {
        &self.event_source
    }

    pub fn set_event_source(&mut self, event_source: SourceEntity) {
        self.event_source = event_source;
    }

    // --------------------------------------------------------------------------------------------
    // Members
    // --------------------------------------------------------------------------------------------

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn contains(&self, name: &Identifier) -> bool {
        self.members.contains_key(name)
    }

    pub fn get(&self, name: &Identifier) -> Option<&Member> {
        self.members.get(name)
    }

    pub fn get_mut(&mut self, name: &Identifier) -> Option<&mut Member> {
        self.members.get_mut(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Member> {
        self.members.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Member> {
        self.members.values_mut()
    }

    pub fn names(&self) -> impl Iterator<Item = &Identifier> {
        self.members.keys()
    }

    pub fn insert(&mut self, value: Member) -> Option<Member> {
        self.members.insert(value.name().clone(), value)
    }

    pub fn extend<I>(&mut self, extension: I)
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
