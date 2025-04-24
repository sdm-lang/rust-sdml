use crate::{
    load::ModuleLoader,
    model::{
        annotations::{
            Annotation, AnnotationBuilder, AnnotationOnlyBody, AnnotationProperty, HasAnnotations,
        },
        check::{MaybeIncomplete, Validate},
        definitions::{FromDefinition, HasOptionalFromDefinition},
        identifiers::{Identifier, IdentifierReference},
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
// Public Types ❱ Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `enum_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EnumDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<EnumBody>,
}

/// Corresponds to the grammar rule `enum_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EnumBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    from: Option<FromDefinition>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    variants: BTreeMap<Identifier, ValueVariant>, // assert!(!variants.is_empty());
}

/// Corresponds to the grammar rule `enum_variant`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ValueVariant {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ EnumDef
// ------------------------------------------------------------------------------------------------

impl HasName for EnumDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasOptionalBody for EnumDef {
    type Body = EnumBody;

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

impl HasSourceSpan for EnumDef {
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

impl MaybeIncomplete for EnumDef {
    fn is_incomplete(&self, _: &Module, _: &impl ModuleStore) -> bool {
        self.body.is_none()
    }
}

impl AnnotationBuilder for EnumDef {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        let mut self_mut = self;
        if self_mut.body.is_none() {
            self_mut.set_body(EnumBody::default());
        }
        if let Some(ref mut inner) = self_mut.body {
            inner.add_to_annotations(AnnotationProperty::new(predicate.into(), value.into()));
        }
        self_mut
    }
}

impl Validate for EnumDef {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        // TODO check that any equivalent class is a datatype.
        self.name()
            .validate(top, loader, Some(IdentifierCaseConvention::TypeDefinition));
        if let Some(body) = &self.body {
            body.validate(top, cache, loader, check_constraints);
        }
    }
}

impl References for EnumDef {
    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl EnumDef {
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

    pub fn with_body(self, body: EnumBody) -> Self {
        let mut self_mut = self;
        self_mut.body = Some(body);
        self_mut
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ EnumBody
// ------------------------------------------------------------------------------------------------

impl HasAnnotations for EnumBody {
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

impl HasSourceSpan for EnumBody {
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

impl HasOptionalFromDefinition for EnumBody {
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

impl References for EnumBody {
    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.variants()
            .for_each(|v| v.referenced_annotations(names));
    }
}

impl Validate for EnumBody {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.annotations()
            .for_each(|a| a.validate(top, cache, loader, check_constraints));
        self.variants()
            .for_each(|v| v.validate(top, cache, loader, check_constraints));
    }
}

impl EnumBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn with_variants<I>(self, variants: I) -> Self
    where
        I: IntoIterator<Item = ValueVariant>,
    {
        let mut self_mut = self;
        self_mut.variants = variants
            .into_iter()
            .map(|elem| (elem.name().clone(), elem))
            .collect();
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }

    pub fn variant_count(&self) -> usize {
        self.variants.len()
    }

    pub fn contains_variant(&self, name: &Identifier) -> bool {
        self.variants.contains_key(name)
    }

    pub fn variant(&self, name: &Identifier) -> Option<&ValueVariant> {
        self.variants.get(name)
    }

    pub fn variant_mut(&mut self, name: &Identifier) -> Option<&mut ValueVariant> {
        self.variants.get_mut(name)
    }

    pub fn variants(&self) -> impl Iterator<Item = &ValueVariant> {
        self.variants.values()
    }

    pub fn variants_mut(&mut self) -> impl Iterator<Item = &mut ValueVariant> {
        self.variants.values_mut()
    }

    pub fn variant_names(&self) -> impl Iterator<Item = &Identifier> {
        self.variants.keys()
    }

    pub fn add_to_variants(&mut self, value: ValueVariant) -> Option<ValueVariant> {
        self.variants.insert(value.name().clone(), value)
    }

    pub fn extend_variants<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = ValueVariant>,
    {
        self.variants.extend(
            extension
                .into_iter()
                .map(|elem| (elem.name().clone(), elem)),
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ ValueVariant
// ------------------------------------------------------------------------------------------------

impl HasName for ValueVariant {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasOptionalBody for ValueVariant {
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

impl HasSourceSpan for ValueVariant {
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

impl AnnotationBuilder for ValueVariant {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        let mut self_mut = self;
        if self_mut.body.is_none() {
            self_mut.set_body(AnnotationOnlyBody::default());
        }
        if let Some(ref mut inner) = self_mut.body {
            inner.add_to_annotations(AnnotationProperty::new(predicate.into(), value.into()));
        }
        self_mut
    }
}

impl Validate for ValueVariant {
    fn validate(
        &self,
        top: &Module,
        _cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        warn!("Missing validation for ValueVariant values.");
        self.name()
            .validate(top, loader, Some(IdentifierCaseConvention::ValueVariant));
    }
}

impl References for ValueVariant {
    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl ValueVariant {
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

    pub fn with_body(self, body: AnnotationOnlyBody) -> Self {
        let mut self_mut = self;
        self_mut.body = Some(body);
        self_mut
    }
}
