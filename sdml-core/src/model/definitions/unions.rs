use crate::{
    load::ModuleLoader,
    model::{
        annotations::{
            Annotation, AnnotationBuilder, AnnotationOnlyBody, AnnotationProperty, HasAnnotations,
        },
        check::{MaybeIncomplete, Validate},
        identifiers::{Identifier, IdentifierReference},
        modules::Module,
        values::Value,
        HasName, HasNameReference, HasOptionalBody, HasSourceSpan, References, Span,
    },
    store::ModuleStore,
};
use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
use std::{collections::{HashMap, HashSet}, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `union_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UnionDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<UnionBody>,
}

/// Corresponds to the grammar rule `union_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UnionBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "HashMap::is_empty"))]
    variants: HashMap<Identifier, TypeVariant>, // assert!(!variants.is_empty());
}

/// Corresponds to the grammar rule `type_variant`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TypeVariant {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name_reference: IdentifierReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    rename: Option<Identifier>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ UnionDef
// ------------------------------------------------------------------------------------------------

impl HasName for UnionDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasOptionalBody for UnionDef {
    type Body = UnionBody;

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

impl HasSourceSpan for UnionDef {
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

impl MaybeIncomplete for UnionDef {
    fn is_incomplete(&self, _: &Module, _: &impl ModuleStore) -> bool {
        self.body.is_none()
    }
}

impl Validate for UnionDef {
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

impl References for UnionDef {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl UnionDef {
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

    pub fn with_body(self, body: UnionBody) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ UnionBody
// ------------------------------------------------------------------------------------------------

impl HasAnnotations for UnionBody {
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

impl HasSourceSpan for UnionBody {
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

impl Validate for UnionBody {
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

impl AnnotationBuilder for UnionDef {
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

impl References for UnionBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.variants
            .iter()
            .for_each(|v| v.referenced_annotations(names));
    }
}

impl UnionBody {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn with_variants<I>(self, variants: I) -> Self
    where
        I: IntoIterator<Item = TypeVariant>,
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

    pub fn is_empty(&self) -> bool {
        self.variants.is_empty()
    }

    pub fn len(&self) -> usize {
        self.variants.len()
    }

    pub fn contains(&self, name: &Identifier) -> bool {
        self.variants.contains_key(name)
    }

    pub fn get(&self, name: &Identifier) -> Option<&TypeVariant> {
        self.variants.get(name)
    }

    pub fn get_mut(&mut self, name: &Identifier) -> Option<&mut TypeVariant> {
        self.variants.get_mut(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &TypeVariant> {
        self.variants.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TypeVariant> {
        self.variants.values_mut()
    }

    pub fn names(&self) -> impl Iterator<Item = &Identifier> {
        self.variants.keys()
    }

    pub fn insert(&mut self, value: TypeVariant) -> Option<TypeVariant> {
        self.variants.insert(value.name().clone(), value)
    }

    pub fn extend<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = TypeVariant>,
    {
        self.variants.extend(
            extension
                .into_iter()
                .map(|elem| (elem.name().clone(), elem)),
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ TypeVariant
// ------------------------------------------------------------------------------------------------

impl HasNameReference for TypeVariant {
    fn name_reference(&self) -> &IdentifierReference {
        &self.name_reference
    }

    fn set_name_reference(&mut self, name: IdentifierReference) {
        self.name_reference = name;
    }
}

impl HasOptionalBody for TypeVariant {
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

impl HasSourceSpan for TypeVariant {
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

impl AnnotationBuilder for TypeVariant {
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

impl Validate for TypeVariant {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.name_reference.validate(top, loader);
        if let Some(rename) = &self.rename {
            rename.validate(top, loader, Some(IdentifierCaseConvention::TypeDefinition));
        }
        if let Some(body) = &self.body {
            body.validate(top, cache, loader, check_constraints);
        }
    }
}

impl References for TypeVariant {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl TypeVariant {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name_reference: IdentifierReference) -> Self {
        Self {
            span: None,
            name_reference,
            rename: None,
            body: None,
        }
    }

    pub fn with_body(self, body: AnnotationOnlyBody) -> Self {
        let mut self_mut = self;
        self_mut.body = Some(body);
        self_mut
    }

    pub fn with_rename(self, rename: Identifier) -> Self {
        let mut self_mut = self;
        self_mut.rename = Some(rename);
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn has_rename(&self) -> bool {
        self.rename.is_some()
    }

    pub const fn rename(&self) -> Option<&Identifier> {
        self.rename.as_ref()
    }

    pub fn set_rename(&mut self, rename: Identifier) {
        self.rename = Some(rename);
    }

    pub fn unset_rename(&mut self) {
        self.rename = None;
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn name(&self) -> &Identifier {
        if let Some(rename) = self.rename() {
            rename
        } else {
            match &self.name_reference {
                IdentifierReference::Identifier(name) => name,
                IdentifierReference::QualifiedIdentifier(name) => name.member(),
            }
        }
    }
}
