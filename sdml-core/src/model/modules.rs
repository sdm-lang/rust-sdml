/*!
Provide the Rust types that implement *module*-related components of the SDML Grammar.
*/
use crate::load::ModuleLoader;
use crate::model::definitions::{
    DatatypeDef, EntityDef, EnumDef, EventDef, PropertyDef, StructureDef, UnionDef,
};
use crate::model::References;
use crate::model::{
    annotations::{Annotation, HasAnnotations},
    check::{MaybeIncomplete, Validate},
    definitions::{Definition, RdfDef, TypeClassDef},
    identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
    HasBody, HasName, HasSourceSpan, Span,
};
use crate::store::{InMemoryModuleCache, ModuleStore};
use sdml_errors::diagnostics::functions::{
    definition_not_found, imported_module_not_found, library_definition_not_allowed,
    module_is_incomplete, module_version_info_empty, module_version_mismatch,
    module_version_not_found, IdentifierCaseConvention,
};
use sdml_errors::{Error, FileId};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::path::PathBuf;
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
    #[cfg_attr(feature = "serde", serde(skip))]
    source_file: Option<PathBuf>,
    #[cfg_attr(feature = "serde", serde(skip))]
    file_id: Option<FileId>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    base_uri: Option<HeaderValue<Url>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    version_info: Option<HeaderValue<String>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    version_uri: Option<HeaderValue<Url>>,
    body: ModuleBody,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct HeaderValue<T> {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    value: T,
}

///
/// Corresponds the grammar rule `module_body`.
///
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ModuleBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    file_id: Option<FileId>, // <- to report errors
    is_library: bool,        // <- to catch errors
    imports: Vec<ImportStatement>,
    annotations: Vec<Annotation>,
    definitions: HashMap<Identifier, Definition>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

///
/// Corresponds the grammar rule `import_statement`.
///
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ImportStatement {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    imports: Vec<Import>,
}

///
/// Corresponds the grammar rule `import`.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Import {
    /// Corresponds to the grammar rule `module_import`.
    Module(ModuleImport),
    /// Corresponds to the grammar rule `member_import`.
    Member(QualifiedIdentifier),
}

///
/// Corresponds the grammar rule `module_import`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ModuleImport {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    version_uri: Option<HeaderValue<Url>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for Module {
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

impl HasName for Module {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasBody for Module {
    type Body = ModuleBody;

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn body_mut(&mut self) -> &mut Self::Body {
        &mut self.body
    }

    fn set_body(&mut self, body: Self::Body) {
        let mut body_mut = body;
        body_mut.file_id = self.file_id;
        body_mut.set_library_status(self.name());
        self.body = body_mut;
    }
}

impl References for Module {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body.referenced_types(names);
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body.referenced_annotations(names);
    }
}

impl Module {
    // --------------------------------------------------------------------------------------------
    // Module :: Constructors
    // --------------------------------------------------------------------------------------------

    pub fn empty(name: Identifier) -> Self {
        Self::new(name, ModuleBody::default())
    }

    pub fn new(name: Identifier, body: ModuleBody) -> Self {
        let mut body = body;
        body.set_library_status(&name);
        Self {
            source_file: None,
            file_id: None,
            span: None,
            name,
            base_uri: None,
            version_info: None,
            version_uri: None,
            body,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Module :: Fields
    // --------------------------------------------------------------------------------------------

    pub fn with_source_file(self, source_file: PathBuf) -> Self {
        Self {
            source_file: Some(source_file),
            ..self
        }
    }

    pub fn with_base_uri(self, base_uri: Url) -> Self {
        Self {
            base_uri: Some(base_uri.into()),
            ..self
        }
    }

    pub fn with_version_info<S>(self, version_info: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            version_info: Some(HeaderValue::from(version_info.into())),
            ..self
        }
    }

    pub fn with_version_uri(self, version_uri: Url) -> Self {
        Self {
            version_uri: Some(version_uri.into()),
            ..self
        }
    }

    pub const fn has_source_file(&self) -> bool {
        self.source_file.is_some()
    }
    pub const fn source_file(&self) -> Option<&PathBuf> {
        self.source_file.as_ref()
    }
    pub fn set_source_file(&mut self, source_file: PathBuf) {
        self.source_file = Some(source_file);
    }
    pub fn unset_source_file(&mut self) {
        self.source_file = None;
    }

    pub const fn has_base_uri(&self) -> bool {
        self.base_uri.is_some()
    }
    pub const fn base_uri(&self) -> Option<&HeaderValue<Url>> {
        self.base_uri.as_ref()
    }
    pub fn set_base_uri(&mut self, base_uri: HeaderValue<Url>) {
        self.base_uri = Some(base_uri);
    }
    pub fn unset_base_uri(&mut self) {
        self.base_uri = None;
    }

    pub const fn has_version_info(&self) -> bool {
        self.version_info.is_some()
    }
    pub const fn version_info(&self) -> Option<&HeaderValue<String>> {
        self.version_info.as_ref()
    }
    pub fn set_version_info(&mut self, version_info: HeaderValue<String>) {
        self.version_info = Some(version_info);
    }
    pub fn unset_version_info(&mut self) {
        self.version_info = None;
    }

    pub const fn has_version_uri(&self) -> bool {
        self.version_uri.is_some()
    }
    pub const fn version_uri(&self) -> Option<&HeaderValue<Url>> {
        self.version_uri.as_ref()
    }
    pub fn set_version_uri(&mut self, version_uri: HeaderValue<Url>) {
        self.version_uri = Some(version_uri);
    }
    pub fn unset_version_uri(&mut self) {
        self.version_uri = None;
    }

    pub const fn has_file_id(&self) -> bool {
        self.file_id.is_some()
    }
    pub const fn file_id(&self) -> Option<&FileId> {
        self.file_id.as_ref()
    }
    pub fn set_file_id(&mut self, file_id: FileId) {
        self.file_id = Some(file_id);
    }
    pub fn unset_file_id(&mut self) {
        self.file_id = None;
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.body.imported_modules()
    }

    #[inline(always)]
    pub fn imported_module_versions(&self) -> HashMap<&Identifier, Option<&HeaderValue<Url>>> {
        self.body.imported_module_versions()
    }

    #[inline(always)]
    pub fn imported_types(&self) -> HashSet<&QualifiedIdentifier> {
        self.body.imported_types()
    }

    #[inline(always)]
    pub fn defined_names(&self) -> HashSet<&Identifier> {
        self.body.defined_names()
    }

    // --------------------------------------------------------------------------------------------
    // Module :: Pseudo-Validate
    // --------------------------------------------------------------------------------------------

    pub fn is_incomplete(&self, cache: &InMemoryModuleCache) -> bool {
        if !self.is_library_module() {
            self.body.is_incomplete(self, cache)
        } else {
            false
        }
    }

    ///
    /// # Checks
    ///
    /// 1. name is valid [`Identifier`]
    /// 1. base URI is absolute [`Url`]
    /// 1. version info string is not empty (warning)
    /// 1. version URI is absolute [`Url`]
    /// 1. body is valid
    ///
    pub fn validate(
        &self,
        cache: &InMemoryModuleCache,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        if !self.is_library_module() {
            self.name
                .validate(self, loader, Some(IdentifierCaseConvention::Module));
            if let Some(version_info) = self.version_info() {
                if version_info.as_ref().is_empty() {
                    loader
                        .report(&module_version_info_empty(
                            self.file_id().copied().unwrap_or_default(),
                            version_info.source_span().map(|span| span.byte_range()),
                        ))
                        .unwrap();
                }
            }
            self.body.validate(self, cache, loader, check_constraints);
            if self.is_incomplete(cache) {
                loader
                    .report(&module_is_incomplete(
                        self.file_id().copied().unwrap_or_default(),
                        self.source_span().map(|span| span.byte_range()),
                        self.name(),
                    ))
                    .unwrap()
            }
        }
    }

    // --------------------------------------------------------------------------------------------
    // Module :: Helpers
    // --------------------------------------------------------------------------------------------

    pub fn is_library_module(&self) -> bool {
        Identifier::is_library_module_name(self.name().as_ref())
    }

    pub fn resolve_local(&self, name: &Identifier) -> Option<&Definition> {
        self.body().definitions().find(|def| def.name() == name)
    }
}

// ------------------------------------------------------------------------------------------------

impl<T> AsRef<T> for HeaderValue<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> From<T> for HeaderValue<T> {
    fn from(value: T) -> Self {
        Self { span: None, value }
    }
}

impl<T: Display> Display for HeaderValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: PartialEq> PartialEq for HeaderValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: PartialEq> PartialEq<T> for HeaderValue<T> {
    fn eq(&self, other: &T) -> bool {
        self.value == *other
    }
}

impl<T: Eq> Eq for HeaderValue<T> {}

impl<T: Hash> Hash for HeaderValue<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // ignore: self.span.hash(state);
        self.value.hash(state);
    }
}

impl<T> HasSourceSpan for HeaderValue<T> {
    fn with_source_span(self, span: Span) -> Self {
        Self {
            span: Some(span),
            ..self
        }
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

impl<T: PartialEq> HeaderValue<T> {
    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.value == other.value
    }
}

impl<T> HeaderValue<T> {
    pub const fn value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for ModuleBody {
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

impl HasAnnotations for ModuleBody {
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

impl References for ModuleBody {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.definitions
            .iter()
            .for_each(|def| def.referenced_types(names))
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.definitions
            .iter()
            .for_each(|def| def.referenced_annotations(names));
    }
}

impl MaybeIncomplete for ModuleBody {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        self.definitions()
            .any(|elem| elem.is_incomplete(top, cache))
    }
}

impl Validate for ModuleBody {
    ///
    /// # Checks
    ///
    /// 1. All import statements are valid [`ImportStatement`]s
    /// 1. All annotations are valid [`Annotation`]s
    /// 1. All definitions are valid [`Definition`]s
    ///
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.imports()
            .for_each(|imp| imp.validate(top, cache, loader, check_constraints));
        self.annotations()
            .for_each(|ann| ann.validate(top, cache, loader, check_constraints));
        self.definitions()
            .for_each(|def| def.validate(top, cache, loader, check_constraints));
    }
}

impl ModuleBody {
    pub fn set_library_status(&mut self, module_name: &Identifier) {
        self.is_library = Identifier::is_library_module_name(module_name);
    }

    // --------------------------------------------------------------------------------------------
    // ModuleBody :: Fields
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

    pub fn add_to_imports<I>(&mut self, value: I)
    where
        I: Into<ImportStatement>,
    {
        self.imports.push(value.into())
    }

    pub fn extend_imports<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = ImportStatement>,
    {
        self.imports.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    fn has_definitions(&self) -> bool {
        !self.definitions.is_empty()
    }

    fn definition_count(&self) -> usize {
        self.definitions.len()
    }

    fn contains_definition(&self, name: &Identifier) -> bool {
        self.definitions.contains_key(name)
    }

    fn definition(&self, name: &Identifier) -> Option<&Definition> {
        self.definitions.get(name)
    }

    fn definition_mut(&mut self, name: &Identifier) -> Option<&mut Definition> {
        self.definitions.get_mut(name)
    }

    fn definitions(&self) -> impl Iterator<Item = &Definition> {
        self.definitions.values()
    }

    fn definitions_mut(&mut self) -> impl Iterator<Item = &mut Definition> {
        self.definitions.values_mut()
    }

    fn definition_names(&self) -> impl Iterator<Item = &Identifier> {
        self.definitions.keys()
    }

    pub fn add_to_definitions<I>(&mut self, value: I) -> Result<(), Error>
    where
        I: Into<Definition>,
    {
        let definition = value.into();
        if !self.is_library && matches!(definition, Definition::Rdf(_) | Definition::TypeClass(_)) {
            Err(library_definition_not_allowed(
                self.file_id.unwrap_or_default(),
                definition.source_span().map(|s| s.into()),
                definition.name(),
            )
            .into())
        } else {
            self.definitions.push(definition);
            Ok(())
        }
    }

    pub fn extend_definitions<I>(&mut self, extension: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = Definition>,
    {
        // we do this manually to ensure the library rules in ModuleBody::push
        for definition in extension.into_iter() {
            self.add_to_definitions(definition)?;
        }
        Ok(())
    }

    // --------------------------------------------------------------------------------------------
    // ModuleBody :: Helpers
    // --------------------------------------------------------------------------------------------

    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.imports()
            .flat_map(|stmt| stmt.imported_modules())
            .collect()
    }

    pub fn imported_module_versions(&self) -> HashMap<&Identifier, Option<&HeaderValue<Url>>> {
        self.imports()
            .flat_map(|stmt| stmt.imported_module_versions())
            .collect()
    }

    pub fn imported_types(&self) -> HashSet<&QualifiedIdentifier> {
        self.imports()
            .flat_map(|stmt| stmt.imported_types())
            .collect()
    }

    // --------------------------------------------------------------------------------------------

    #[inline]
    pub fn get_definition(&self, name: &Identifier) -> Option<&Definition> {
        self.definitions().find(|d| d.name() == name)
    }

    #[inline]
    pub fn datatype_definitions(&self) -> impl Iterator<Item = &DatatypeDef> {
        self.definitions().filter_map(|d| match d {
            Definition::Datatype(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn entity_definitions(&self) -> impl Iterator<Item = &EntityDef> {
        self.definitions().filter_map(|d| match d {
            Definition::Entity(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn enum_definitions(&self) -> impl Iterator<Item = &EnumDef> {
        self.definitions().filter_map(|d| match d {
            Definition::Enum(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn event_definitions(&self) -> impl Iterator<Item = &EventDef> {
        self.definitions().filter_map(|d| match d {
            Definition::Event(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn property_definitions(&self) -> impl Iterator<Item = &PropertyDef> {
        self.definitions().filter_map(|d| match d {
            Definition::Property(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn rdf_definitions(&self) -> impl Iterator<Item = &RdfDef> {
        self.definitions().filter_map(|d| match d {
            Definition::Rdf(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn structure_definitions(&self) -> impl Iterator<Item = &StructureDef> {
        self.definitions().filter_map(|d| match d {
            Definition::Structure(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn type_class_definitions(&self) -> impl Iterator<Item = &TypeClassDef> {
        self.definitions().filter_map(|d| match d {
            Definition::TypeClass(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn union_definitions(&self) -> impl Iterator<Item = &UnionDef> {
        self.definitions().filter_map(|d| match d {
            Definition::Union(v) => Some(v),
            _ => None,
        })
    }

    pub fn defined_names(&self) -> HashSet<&Identifier> {
        self.definitions().map(|def| def.name()).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

impl From<Import> for ImportStatement {
    fn from(value: Import) -> Self {
        Self::new(vec![value])
    }
}

impl From<Vec<Import>> for ImportStatement {
    fn from(value: Vec<Import>) -> Self {
        Self::new(value)
    }
}

impl FromIterator<Import> for ImportStatement {
    fn from_iter<T: IntoIterator<Item = Import>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl From<ImportStatement> for Vec<Import> {
    fn from(value: ImportStatement) -> Self {
        value.imports
    }
}

impl HasSourceSpan for ImportStatement {
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

impl Validate for ImportStatement {
    ///
    /// # Checks
    ///
    /// - For each [`Import`]:
    ///   - If module import:
    ///     1. Ensure it is in the cache
    ///     1. If the import has a version URI ensure the imported module has a matching one
    ///     1.
    ///     1.
    ///     1.
    ///
    ///
    ///
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        _: bool,
    ) {
        for import in self.imports() {
            match import {
                Import::Module(module_ref) => {
                    module_ref
                        .name()
                        .validate(top, loader, Some(IdentifierCaseConvention::Module));
                    if let Some(actual_module) = cache.get(module_ref.name()) {
                        match (module_ref.version_uri(), actual_module.version_uri()) {
                            (None, _) => {}
                            (Some(expected), Some(actual)) => {
                                if actual != expected {
                                    loader
                                        .report(&module_version_mismatch(
                                            top.file_id().copied().unwrap_or_default(),
                                            expected.source_span().map(|s| s.byte_range()),
                                            expected.as_ref().to_string(),
                                            actual_module.file_id().copied().unwrap_or_default(),
                                            actual.source_span().map(|s| s.byte_range()),
                                            actual.as_ref().to_string(),
                                        ))
                                        .unwrap();
                                }
                            }
                            (Some(expected), None) => {
                                loader
                                    .report(&module_version_not_found(
                                        top.file_id().copied().unwrap_or_default(),
                                        module_ref.source_span().map(|s| s.byte_range()),
                                        expected.as_ref().to_string(),
                                        actual_module.file_id().copied().unwrap_or_default(),
                                        actual_module.source_span().map(|s| s.byte_range()),
                                        actual_module.name(),
                                    ))
                                    .unwrap();
                            }
                        }
                    } else {
                        loader
                            .report(&imported_module_not_found(
                                top.file_id().copied().unwrap_or_default(),
                                module_ref.source_span().map(|s| s.byte_range()),
                                module_ref.name(),
                            ))
                            .unwrap();
                    }
                }
                Import::Member(id_ref) => {
                    id_ref.validate(top, loader);
                    if let Some(actual_module) = cache.get(id_ref.module()) {
                        if actual_module.resolve_local(id_ref.member()).is_none() {
                            loader
                                .report(&definition_not_found(
                                    top.file_id().copied().unwrap_or_default(),
                                    id_ref.source_span().map(|s| s.byte_range()),
                                    id_ref,
                                ))
                                .unwrap();
                        }
                    } else {
                        loader
                            .report(&imported_module_not_found(
                                top.file_id().copied().unwrap_or_default(),
                                id_ref.source_span().map(|s| s.byte_range()),
                                id_ref,
                            ))
                            .unwrap();
                    }
                }
            }
        }
    }
}

impl ImportStatement {
    // --------------------------------------------------------------------------------------------
    // ImportStatement :: Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(imports: Vec<Import>) -> Self {
        Self {
            span: None,
            imports,
        }
    }

    pub fn new_module(import: Identifier) -> Self {
        Self {
            span: None,
            imports: vec![Import::from(ModuleImport::from(import))],
        }
    }

    pub fn new_module_with_version_uri(import: Identifier, version_uri: Url) -> Self {
        Self {
            span: None,
            imports: vec![Import::from(
                ModuleImport::from(import).with_version_uri(version_uri.into()),
            )],
        }
    }

    pub fn new_member(import: QualifiedIdentifier) -> Self {
        Self {
            span: None,
            imports: vec![Import::from(import)],
        }
    }

    // --------------------------------------------------------------------------------------------
    // ImportStatement :: Fields
    // --------------------------------------------------------------------------------------------

    pub fn has_imports(&self) -> bool {
        !self.imports.is_empty()
    }

    pub fn imports_len(&self) -> usize {
        self.imports.len()
    }

    pub fn imports(&self) -> impl Iterator<Item = &Import> {
        self.imports.iter()
    }

    pub fn imports_mut(&mut self) -> impl Iterator<Item = &mut Import> {
        self.imports.iter_mut()
    }

    pub fn add_to_imports<I>(&mut self, value: I)
    where
        I: Into<Import>,
    {
        self.imports.push(value.into())
    }

    pub fn extend_imports<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Import>,
    {
        self.imports.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn as_slice(&self) -> &[Import] {
        self.imports.as_slice()
    }

    // --------------------------------------------------------------------------------------------

    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.imports()
            .map(|imp| match imp {
                Import::Module(v) => v.name(),
                Import::Member(v) => v.module(),
            })
            .collect()
    }

    pub fn imported_module_versions(&self) -> HashMap<&Identifier, Option<&HeaderValue<Url>>> {
        HashMap::from_iter(self.imports().map(|imp| match imp {
            Import::Module(v) => (v.name(), v.version_uri()),
            Import::Member(v) => (v.module(), None),
        }))
    }

    pub fn imported_types(&self) -> HashSet<&QualifiedIdentifier> {
        self.imports()
            .filter_map(|imp| {
                if let Import::Member(imp) = imp {
                    Some(imp)
                } else {
                    None
                }
            })
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<&Identifier> for Import {
    fn from(v: &Identifier) -> Self {
        Self::Module(ModuleImport::from(v.clone()))
    }
}

impl From<Identifier> for Import {
    fn from(v: Identifier) -> Self {
        Self::Module(ModuleImport::from(v))
    }
}

impl From<&ModuleImport> for Import {
    fn from(v: &ModuleImport) -> Self {
        Self::Module(v.clone())
    }
}

impl From<ModuleImport> for Import {
    fn from(v: ModuleImport) -> Self {
        Self::Module(v)
    }
}

impl From<&QualifiedIdentifier> for Import {
    fn from(v: &QualifiedIdentifier) -> Self {
        Self::Member(v.clone())
    }
}

impl From<QualifiedIdentifier> for Import {
    fn from(v: QualifiedIdentifier) -> Self {
        Self::Member(v)
    }
}

impl std::fmt::Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Module(v) => v.to_string(),
                Self::Member(v) => v.to_string(),
            }
        )
    }
}

impl HasSourceSpan for Import {
    #[inline]
    fn with_source_span(self, span: Span) -> Self {
        match self {
            Self::Module(v) => Self::Module(v.with_source_span(span)),
            Self::Member(v) => Self::Member(v.with_source_span(span)),
        }
    }

    #[inline]
    fn source_span(&self) -> Option<&Span> {
        match self {
            Self::Module(v) => v.source_span(),
            Self::Member(v) => v.source_span(),
        }
    }

    #[inline]
    fn set_source_span(&mut self, span: Span) {
        match self {
            Self::Module(v) => v.set_source_span(span),
            Self::Member(v) => v.set_source_span(span),
        }
    }

    #[inline]
    fn unset_source_span(&mut self) {
        match self {
            Self::Module(v) => v.unset_source_span(),
            Self::Member(v) => v.unset_source_span(),
        }
    }
}

impl Import {
    pub fn module(&self) -> &Identifier {
        match self {
            Import::Module(v) => v.name(),
            Import::Member(v) => v.module(),
        }
    }
    pub fn member(&self) -> Option<&Identifier> {
        match self {
            Import::Module(_) => None,
            Import::Member(v) => Some(v.member()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ModuleImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if let Some(version_uri) = self.version_uri() {
                format!("{} version {}", self.name(), version_uri)
            } else {
                self.name().to_string()
            }
        )
    }
}

impl From<Identifier> for ModuleImport {
    fn from(value: Identifier) -> Self {
        Self::new(value)
    }
}

impl PartialEq for ModuleImport {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version_uri == other.version_uri
    }
}

impl Eq for ModuleImport {}

impl Hash for ModuleImport {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // ignore: self.span.hash(state);
        self.name.hash(state);
        self.version_uri.hash(state);
    }
}

impl HasSourceSpan for ModuleImport {
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

impl ModuleImport {
    // --------------------------------------------------------------------------------------------
    // ModuleImport :: Constructors
    // --------------------------------------------------------------------------------------------
    pub const fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            version_uri: None,
        }
    }

    pub fn with_version_uri(self, version_uri: HeaderValue<Url>) -> Self {
        Self {
            version_uri: Some(version_uri),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------
    // ModuleImport :: Fields
    // --------------------------------------------------------------------------------------------

    pub const fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    pub const fn has_version_uri(&self) -> bool {
        self.version_uri.is_some()
    }

    pub const fn version_uri(&self) -> Option<&HeaderValue<Url>> {
        self.version_uri.as_ref()
    }

    pub fn set_version_uri(&mut self, version_uri: HeaderValue<Url>) {
        self.version_uri = Some(version_uri);
    }

    pub fn unset_version_uri(&mut self) {
        self.version_uri = None;
    }

    // --------------------------------------------------------------------------------------------
    // ModuleImport :: Helpers
    // --------------------------------------------------------------------------------------------

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.name == other.name && self.version_uri == other.version_uri
    }
}
