use crate::model::{
    Annotation, AnnotationProperty, Constraint, Definition, Identifier, IdentifierReference,
    ImportStatement, ModelElement, QualifiedIdentifier, Span,
};
use std::{collections::HashSet, fmt::Debug};
use url::Url;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Modules
// ------------------------------------------------------------------------------------------------

///
/// Corresponds the grammar rule `module`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Module {
    span: Option<Span>,
    name: Identifier,
    base: Option<Url>,
    body: ModuleBody,
}

///
/// Corresponds the grammar rule `module_body`.
///
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ModuleBody {
    span: Option<Span>,
    imports: Vec<ImportStatement>,
    annotations: Vec<Annotation>,
    definitions: Vec<Definition>,
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
// Implementations ❱ Modules
// ------------------------------------------------------------------------------------------------

impl ModelElement for Module {
    fn has_ts_span(&self) -> bool {
        self.ts_span().is_some()
    }
    fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_ts_span(&mut self) {
        self.span = None;
    }

    fn name(&self) -> &Identifier {
        &self.name
    }
    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    delegate!(is_complete, bool, body);
    delegate!(referenced_types, HashSet<&IdentifierReference>, body);
    delegate!(referenced_annotations, HashSet<&IdentifierReference>, body);
}

impl Module {
    pub fn empty(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            base: None,
            body: Default::default(),
        }
    }

    pub fn new(name: Identifier, body: ModuleBody) -> Self {
        Self {
            span: None,
            name,
            base: None,
            body,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    pub fn with_base(self, base: Url) -> Self {
        Self {
            base: Some(base),
            ..self
        }
    }

    pub fn has_base(&self) -> bool {
        self.base().is_some()
    }
    pub fn base(&self) -> Option<&Url> {
        self.base.as_ref()
    }
    pub fn set_base(&mut self, base: Url) {
        self.base = Some(base);
    }
    pub fn unset_base(&mut self) {
        self.base = None;
    }

    pub fn body(&self) -> &ModuleBody {
        &self.body
    }
    pub fn set_body(&mut self, body: ModuleBody) {
        self.body = body;
    }

    // --------------------------------------------------------------------------------------------

    delegate!(pub imported_modules, HashSet<&Identifier>, body);
    delegate!(pub imported_types, HashSet<&QualifiedIdentifier>, body);
    delegate!(pub defined_names, HashSet<&Identifier>, body);
    delegate!(pub referenced_types, HashSet<&IdentifierReference>, body);
    delegate!(pub referenced_annotations, HashSet<&IdentifierReference> , body);
}

// ------------------------------------------------------------------------------------------------

impl ModuleBody {
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

    pub fn has_imports(&self) -> bool {
        !self.imports.is_empty()
    }
    pub fn imports_len(&self) -> usize {
        self.imports.len()
    }
    pub fn imports(&self) -> impl Iterator<Item = &ImportStatement> {
        self.imports.iter()
    }
    pub fn imports_mut(&mut self) -> impl Iterator<Item = &mut ImportStatement> {
        self.imports.iter_mut()
    }
    pub fn add_to_imports(&mut self, value: ImportStatement) {
        self.imports.push(value)
    }
    pub fn extend_imports<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = ImportStatement>,
    {
        self.imports.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_definitions(&self) -> bool {
        !self.definitions.is_empty()
    }
    pub fn definitions_len(&self) -> usize {
        self.definitions.len()
    }
    pub fn definitions(&self) -> impl Iterator<Item = &Definition> {
        self.definitions.iter()
    }
    pub fn definitions_mut(&mut self) -> impl Iterator<Item = &mut Definition> {
        self.definitions.iter_mut()
    }
    pub fn add_to_definitions<I>(&mut self, value: I)
    where
        I: Into<Definition>,
    {
        self.definitions.push(value.into())
    }
    pub fn extend_definitions<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Definition>,
    {
        self.definitions.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.definitions().all(|d| d.is_complete())
    }

    // --------------------------------------------------------------------------------------------

    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.imports()
            .flat_map(|stmt| stmt.imported_modules())
            .collect()
    }

    pub fn imported_types(&self) -> HashSet<&QualifiedIdentifier> {
        self.imports()
            .flat_map(|stmt| stmt.imported_types())
            .collect()
    }

    pub fn defined_names(&self) -> HashSet<&Identifier> {
        self.definitions().map(|def| def.name()).collect()
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.definitions()
            .flat_map(|def| def.referenced_types())
            .collect()
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.definitions()
            .flat_map(|def| def.referenced_annotations())
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
