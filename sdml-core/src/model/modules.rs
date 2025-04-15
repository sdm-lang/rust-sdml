/*!
Provide the Rust types that implement *module*-related components of the SDML Grammar.
 */
use crate::config::{
    is_rdf_definition_allowed_in_module, is_typeclass_definition_allowed_in_module,
};
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
    HasName, HasSourceSpan, Span,
};
use crate::store::{InMemoryModuleCache, ModuleStore};
use crate::syntax::PC_MODULE_PATH_SEPARATOR;
use sdml_errors::diagnostics::functions::{
    definition_not_found, imported_module_not_found, invalid_module_base_uri,
    invalid_module_version_uri, library_definition_not_allowed_in, module_is_incomplete,
    module_version_info_empty, module_version_mismatch, module_version_not_found,
    IdentifierCaseConvention,
};
use sdml_errors::{Error, FileId};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::hash::Hash;
use std::path::PathBuf;
use std::str::FromStr;
use std::{collections::BTreeSet, fmt::Debug};
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
    imports: Vec<ImportStatement>,
    annotations: Vec<Annotation>,
    definitions: BTreeMap<Identifier, Definition>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct HeaderValue<T> {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    value: T,
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    from_clause: Option<ModulePath>,
    imports: Vec<Import>,
}

///
/// Corresponds the embedded choice in grammar rule `from_clause`:
///
/// ```js
/// choice(
///     $.module_path_absolute,
///     $.module_path_relative,
///     $.module_path_root_only
/// )
/// ```
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ModulePath {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    is_absolute: bool,
    segments: Vec<Identifier>,
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
    Member(MemberImport),
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    renamed_as: Option<Identifier>,
}

///
/// Corresponds the grammar rule `member_import`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MemberImport {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: QualifiedIdentifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    renamed_as: Option<Identifier>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules ❱ Module
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

impl HasAnnotations for Module {
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

impl References for Module {
    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.definitions()
            .for_each(|def| def.referenced_types(names))
    }

    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.definitions()
            .for_each(|def| def.referenced_annotations(names));
    }
}

impl Module {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name: Identifier) -> Self {
        Self {
            source_file: Default::default(),
            file_id: Default::default(),
            span: Default::default(),
            name: name,
            base_uri: Default::default(),
            version_info: Default::default(),
            version_uri: Default::default(),
            imports: Default::default(),
            annotations: Default::default(),
            definitions: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields ❱ source_file
    // --------------------------------------------------------------------------------------------

    pub fn with_source_file(self, source_file: PathBuf) -> Self {
        let mut self_mut = self;
        self_mut.source_file = Some(source_file);
        self_mut
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

    // --------------------------------------------------------------------------------------------
    // Fields ❱ file_id
    // --------------------------------------------------------------------------------------------

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
    // Fields ❱ base_uri
    // --------------------------------------------------------------------------------------------

    pub fn with_base_uri(self, base_uri: Url) -> Self {
        let mut self_mut = self;
        self_mut.base_uri = Some(base_uri.into());
        self_mut
    }

    pub const fn has_base_uri(&self) -> bool {
        self.base_uri.is_some()
    }

    pub const fn base_uri(&self) -> Option<&HeaderValue<Url>> {
        self.base_uri.as_ref()
    }

    pub fn set_base_uri(&mut self, base_uri: HeaderValue<Url>) {
        // TODO: validate
        self.base_uri = Some(base_uri);
    }

    pub fn unset_base_uri(&mut self) {
        self.base_uri = None;
    }

    // --------------------------------------------------------------------------------------------
    // Fields ❱ version_info
    // --------------------------------------------------------------------------------------------

    pub fn with_version_info<S>(self, version_info: S) -> Self
    where
        S: Into<String>,
    {
        let mut self_mut = self;
        self_mut.version_info = Some(HeaderValue::from(version_info.into()));
        self_mut
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

    // --------------------------------------------------------------------------------------------
    // Fields ❱ version_uri
    // --------------------------------------------------------------------------------------------

    pub fn with_version_uri(self, version_uri: Url) -> Self {
        let mut self_mut = self;
        self_mut.version_uri = Some(version_uri.into());
        self_mut
    }

    pub const fn has_version_uri(&self) -> bool {
        self.version_uri.is_some()
    }

    pub const fn version_uri(&self) -> Option<&HeaderValue<Url>> {
        self.version_uri.as_ref()
    }

    pub fn set_version_uri(&mut self, version_uri: HeaderValue<Url>) {
        // TODO: validate
        self.version_uri = Some(version_uri);
    }

    pub fn unset_version_uri(&mut self) {
        self.version_uri = None;
    }

    // --------------------------------------------------------------------------------------------
    // Fields ❱  imports
    // --------------------------------------------------------------------------------------------

    pub fn with_imports<I>(self, import_statements: I) -> Self
    where
        I: IntoIterator<Item = ImportStatement>,
    {
        let mut self_mut = self;
        self_mut.extend_imports(import_statements);
        self_mut
    }

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
    // Fields ❱  definitions
    // --------------------------------------------------------------------------------------------

    pub fn with_definitions<I>(self, definitions: I) -> Self
    where
        I: IntoIterator<Item = Definition>,
    {
        let mut self_mut = self;
        self_mut.extend_definitions(definitions).unwrap();
        self_mut
    }

    pub fn has_definitions(&self) -> bool {
        !self.definitions.is_empty()
    }

    pub fn definition_count(&self) -> usize {
        self.definitions.len()
    }

    pub fn contains_definition(&self, name: &Identifier) -> bool {
        self.definitions.contains_key(name)
    }

    pub fn definition(&self, name: &Identifier) -> Option<&Definition> {
        self.definitions.get(name)
    }

    pub fn definition_mut(&mut self, name: &Identifier) -> Option<&mut Definition> {
        self.definitions.get_mut(name)
    }

    pub fn definitions(&self) -> impl Iterator<Item = &Definition> {
        self.definitions.values()
    }

    pub fn definitions_mut(&mut self) -> impl Iterator<Item = &mut Definition> {
        self.definitions.values_mut()
    }

    pub fn definition_names(&self) -> impl Iterator<Item = &Identifier> {
        self.definitions.keys()
    }

    pub fn add_to_definitions<I>(&mut self, value: I) -> Result<(), Error>
    where
        I: Into<Definition>,
    {
        let definition = value.into();

        if (matches!(definition, Definition::Rdf(_))
            && !is_rdf_definition_allowed_in_module(&self.name))
            || (matches!(definition, Definition::TypeClass(_))
                && !is_typeclass_definition_allowed_in_module(&self.name))
        {
            Err(library_definition_not_allowed_in(
                self.file_id.unwrap_or_default(),
                definition.source_span().map(|s| s.into()),
                definition.name(),
                self.name(),
            )
            .into())
        } else {
            self.definitions
                .insert(definition.name().clone(), definition);
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
    // Pseudo-Validate
    // --------------------------------------------------------------------------------------------

    fn is_incomplete(&self, cache: &impl ModuleStore) -> bool {
        if !self.is_library_module() {
            self.definitions()
                .any(|elem| elem.is_incomplete(self, cache))
        } else {
            false // library modules are complete
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
        fn is_namespace_safe(uri: &Url) -> bool {
            uri.fragment() == Some("") || (uri.path().ends_with("/") && uri.query().is_none())
        }
        if !self.is_library_module() {
            self.name
                .validate(self, loader, Some(IdentifierCaseConvention::Module));
            if let Some(base_uri) = self.base_uri() {
                let uri = base_uri.value();
                if !is_namespace_safe(uri) {
                    loader
                        .report(&invalid_module_base_uri(
                            self.file_id().copied().unwrap_or_default(),
                            base_uri.source_span().map(|span| span.byte_range()),
                            uri.to_string(),
                        ))
                        .unwrap();
                }
            }
            if let Some(version_uri) = self.version_uri() {
                let uri = version_uri.value();
                if !is_namespace_safe(uri) {
                    loader
                        .report(&invalid_module_version_uri(
                            self.file_id().copied().unwrap_or_default(),
                            version_uri.source_span().map(|span| span.byte_range()),
                            uri.to_string(),
                        ))
                        .unwrap();
                }
            }
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
            self.validate(cache, loader, check_constraints);
            if self.is_incomplete(cache) {
                loader
                    .report(&module_is_incomplete(
                        self.file_id().copied().unwrap_or_default(),
                        self.source_span().map(|span| span.byte_range()),
                        self.name(),
                    ))
                    .unwrap()
            }
            self.imports()
                .for_each(|imp| imp.validate(self, cache, loader, check_constraints));
            // TODO: check that no module is loaded multiple times with different versions.
            self.annotations()
                .for_each(|ann| ann.validate(self, cache, loader, check_constraints));
            self.definitions()
                .for_each(|def| def.validate(self, cache, loader, check_constraints));
        }
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub const fn is_versioned(&self) -> bool {
        self.has_version_info() || self.has_version_uri()
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_library_module(&self) -> bool {
        Identifier::is_library_module_name(self.name().as_ref())
    }

    pub fn resolve_local(&self, name: &Identifier) -> Option<&Definition> {
        self.definitions().find(|def| def.name() == name)
    }

    // --------------------------------------------------------------------------------------------

    pub fn imported_modules(&self) -> BTreeSet<&Identifier> {
        self.imports()
            .flat_map(|stmt| stmt.imported_modules())
            .collect()
    }

    pub fn imported_module_versions(&self) -> BTreeMap<&Identifier, Option<&HeaderValue<Url>>> {
        self.imports()
            .flat_map(|stmt| stmt.imported_module_versions())
            .collect()
    }

    pub fn imported_types(&self) -> BTreeSet<&QualifiedIdentifier> {
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

    pub fn defined_names(&self) -> BTreeSet<&Identifier> {
        self.definitions().map(|def| def.name()).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules ❱ HeaderValue
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
// Implementations ❱ Modules ❱ ImportStatement
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
                    module_ref.effective_name().validate(
                        top,
                        loader,
                        Some(IdentifierCaseConvention::Module),
                    );
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
                Import::Member(member_ref) => {
                    let id_ref = member_ref.name();
                    //let id_ref = member_ref.effective_name();
                    // TODO: check if this ir correct: id_ref.validate(top, loader);
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
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(imports: Vec<Import>) -> Self {
        Self {
            span: None,
            from_clause: None,
            imports,
        }
    }

    pub fn new_module(import: Identifier) -> Self {
        Self {
            span: None,
            from_clause: None,
            imports: vec![Import::from(ModuleImport::from(import))],
        }
    }

    pub fn new_module_with_version_uri(import: Identifier, version_uri: Url) -> Self {
        Self {
            span: None,
            from_clause: None,
            imports: vec![Import::from(
                ModuleImport::from(import).with_version_uri(version_uri.into()),
            )],
        }
    }

    pub fn new_member(import: QualifiedIdentifier) -> Self {
        Self {
            span: None,
            from_clause: None,
            imports: vec![Import::from(import)],
        }
    }

    // TODO: with_from__path
    // TODO: with_imports

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn has_from_module_path(&self) -> bool {
        self.from_clause.is_some()
    }

    pub fn from_module_path(&self) -> Option<&ModulePath> {
        self.from_clause.as_ref()
    }

    pub fn set_from_module_path(&mut self, path: ModulePath) {
        self.from_clause = Some(path);
    }

    pub fn unset_from_module_path(&mut self) {
        self.from_clause = None;
    }

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

    pub fn imported_modules(&self) -> BTreeSet<&Identifier> {
        self.imports()
            .map(|imp| match imp {
                Import::Module(v) => v.name(),
                Import::Member(v) => v.module(),
            })
            .collect()
    }

    pub fn imported_module_versions(&self) -> BTreeMap<&Identifier, Option<&HeaderValue<Url>>> {
        BTreeMap::from_iter(self.imports().map(|imp| match imp {
            Import::Module(v) => (v.name(), v.version_uri()),
            Import::Member(v) => (v.module(), None),
        }))
    }

    pub fn imported_types(&self) -> BTreeSet<&QualifiedIdentifier> {
        self.imports()
            .filter_map(|imp| {
                if let Import::Member(imp) = imp {
                    Some(imp.name())
                } else {
                    None
                }
            })
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if self.is_absolute() {
                PC_MODULE_PATH_SEPARATOR
            } else {
                ""
            },
            self.segments()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(PC_MODULE_PATH_SEPARATOR),
        )
    }
}

impl FromStr for ModulePath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const PREFIX_OFFSET: usize = PC_MODULE_PATH_SEPARATOR.len();
        if s.is_empty() {
            Ok(ModulePath::default())
        } else if s == PC_MODULE_PATH_SEPARATOR {
            Ok(ModulePath::root())
        } else {
            let (absolute, modules) = if s.starts_with(PC_MODULE_PATH_SEPARATOR) {
                (true, s[PREFIX_OFFSET..].split(PC_MODULE_PATH_SEPARATOR))
            } else {
                (false, s.split(PC_MODULE_PATH_SEPARATOR))
            };
            Ok(ModulePath::new(
                absolute,
                modules
                    .map(Identifier::from_str)
                    .collect::<Result<Vec<Identifier>, Self::Err>>()?,
            ))
        }
    }
}

impl HasSourceSpan for ModulePath {
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

impl Validate for ModulePath {
    fn validate(
        &self,
        _top: &Module,
        _cache: &impl ModuleStore,
        _loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        // TODO: ensure the hierarchy is followed.
        todo!()
    }
}

impl ModulePath {
    pub fn new<T>(is_absolute: bool, segments: T) -> Self
    where
        T: Into<Vec<Identifier>>,
    {
        Self {
            span: None,
            is_absolute,
            segments: segments.into(),
        }
    }

    pub fn new_unchecked(is_absolute: bool, segments: &[&str]) -> Self {
        Self {
            span: None,
            is_absolute,
            segments: segments
                .iter()
                .map(|id| Identifier::new_unchecked(id))
                .collect(),
        }
    }

    pub fn root() -> Self {
        Self {
            span: None,
            is_absolute: true,
            segments: Vec::default(),
        }
    }

    pub fn absolute<T>(modules: T) -> Self
    where
        T: Into<Vec<Identifier>>,
    {
        Self {
            span: None,
            is_absolute: true,
            segments: modules.into(),
        }
    }

    pub fn absolute_one(module: Identifier) -> Self {
        Self::absolute(vec![module])
    }

    pub fn relative<T>(modules: T) -> Self
    where
        T: Into<Vec<Identifier>>,
    {
        Self {
            span: None,
            is_absolute: false,
            segments: modules.into(),
        }
    }

    pub fn relative_one(module: Identifier) -> Self {
        Self::relative(vec![module])
    }

    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    pub fn set_absolute(&mut self, is_absolute: bool) {
        self.is_absolute = is_absolute;
    }

    pub fn is_root(&self) -> bool {
        self.is_absolute() && !self.has_segments()
    }

    pub fn has_segments(&self) -> bool {
        !self.segments.is_empty()
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    pub fn segments(&self) -> impl Iterator<Item = &Identifier> {
        self.segments.iter()
    }

    pub fn segments_mut(&mut self) -> impl Iterator<Item = &mut Identifier> {
        self.segments.iter_mut()
    }

    pub fn add_to_segments<I>(&mut self, value: I)
    where
        I: Into<Identifier>,
    {
        self.segments.push(value.into())
    }

    pub fn extend_segments<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Identifier>,
    {
        self.segments.extend(extension)
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
        Self::Member(MemberImport::from(v.clone()))
    }
}

impl From<QualifiedIdentifier> for Import {
    fn from(v: QualifiedIdentifier) -> Self {
        Self::Member(MemberImport::from(v))
    }
}

impl From<&MemberImport> for Import {
    fn from(v: &MemberImport) -> Self {
        Self::Member(v.clone())
    }
}

impl From<MemberImport> for Import {
    fn from(v: MemberImport) -> Self {
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
// Implementations ❱ Modules ❱ ModuleImport
// ------------------------------------------------------------------------------------------------

impl From<&Identifier> for ModuleImport {
    fn from(value: &Identifier) -> Self {
        Self::new(value.clone())
    }
}

impl From<Identifier> for ModuleImport {
    fn from(value: Identifier) -> Self {
        Self::new(value)
    }
}

impl PartialEq for ModuleImport {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.version_uri == other.version_uri
            && self.renamed_as == other.renamed_as
    }
}

impl Eq for ModuleImport {}

impl Hash for ModuleImport {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // ignore: self.span.hash(state);
        self.name.hash(state);
        self.version_uri.hash(state);
        self.renamed_as.hash(state);
    }
}

impl Display for ModuleImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if let Some(version_uri) = self.version_uri() {
                format!("{} version {}", self.name(), version_uri)
            } else {
                self.name().to_string()
            },
            if let Some(renamed_as) = &self.renamed_as {
                format!(" as {renamed_as}")
            } else {
                String::new()
            }
        )
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
    // Constructors
    // --------------------------------------------------------------------------------------------
    pub const fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            version_uri: None,
            renamed_as: None,
        }
    }

    pub fn with_version_uri(self, version_uri: HeaderValue<Url>) -> Self {
        let mut self_mut = self;
        self_mut.version_uri = Some(version_uri);
        self_mut
    }

    pub fn with_rename(self, renamed_as: Identifier) -> Self {
        let mut self_mut = self;
        self_mut.renamed_as = Some(renamed_as);
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn effective_name(&self) -> Identifier {
        if let Some(rename) = self.renamed_as() {
            rename.clone()
        } else {
            self.name().clone()
        }
    }

    // --------------------------------------------------------------------------------------------

    pub const fn has_been_renamed(&self) -> bool {
        self.renamed_as.is_some()
    }

    pub const fn renamed_as(&self) -> Option<&Identifier> {
        self.renamed_as.as_ref()
    }

    pub fn set_rename_as(&mut self, renamed_as: Identifier) {
        self.renamed_as = Some(renamed_as);
    }

    pub fn unset_rename_as(&mut self) {
        self.renamed_as = None;
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span
            && self.name == other.name
            && self.version_uri == other.version_uri
            && self.renamed_as == other.renamed_as
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules ❱ MemberImport
// ------------------------------------------------------------------------------------------------

impl From<&QualifiedIdentifier> for MemberImport {
    fn from(value: &QualifiedIdentifier) -> Self {
        Self::new(value.clone())
    }
}

impl From<QualifiedIdentifier> for MemberImport {
    fn from(value: QualifiedIdentifier) -> Self {
        Self::new(value)
    }
}

impl PartialEq for MemberImport {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.renamed_as == other.renamed_as
    }
}

impl Eq for MemberImport {}

impl Hash for MemberImport {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // ignore: self.span.hash(state);
        self.name.hash(state);
        self.renamed_as.hash(state);
    }
}

impl Display for MemberImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.name(),
            if let Some(renamed_as) = &self.renamed_as {
                format!(" as {renamed_as}")
            } else {
                String::new()
            }
        )
    }
}

impl HasSourceSpan for MemberImport {
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

impl MemberImport {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------
    pub const fn new(name: QualifiedIdentifier) -> Self {
        Self {
            span: None,
            name,
            renamed_as: None,
        }
    }

    pub fn with_rename(self, renamed_as: Identifier) -> Self {
        let mut self_mut = self;
        self_mut.renamed_as = Some(renamed_as);
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn name(&self) -> &QualifiedIdentifier {
        &self.name
    }

    pub fn set_name(&mut self, name: QualifiedIdentifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    pub const fn module(&self) -> &Identifier {
        self.name().module()
    }

    pub const fn member(&self) -> &Identifier {
        self.name().member()
    }

    // --------------------------------------------------------------------------------------------

    pub fn effective_name(&self) -> IdentifierReference {
        if let Some(rename) = self.renamed_as() {
            rename.clone().into()
        } else {
            self.name().clone().into()
        }
    }

    // --------------------------------------------------------------------------------------------

    pub const fn has_been_renamed(&self) -> bool {
        self.renamed_as.is_some()
    }

    pub const fn renamed_as(&self) -> Option<&Identifier> {
        self.renamed_as.as_ref()
    }

    pub fn set_rename_as(&mut self, renamed_as: Identifier) {
        self.renamed_as = Some(renamed_as);
    }

    pub fn unset_rename_as(&mut self) {
        self.renamed_as = None;
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn eq_with_span(&self, other: &Self) -> bool {
        self.span == other.span && self.name == other.name && self.renamed_as == other.renamed_as
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_path_from_str_default() {
        let path = ModulePath::from_str("").unwrap();
        assert!(!path.is_root());
        assert!(!path.is_absolute());
        assert!(!path.has_segments());
        assert_eq!(path.segment_count(), 0);
    }

    #[test]
    fn test_module_path_from_str_root() {
        let path = ModulePath::from_str("::").unwrap();
        assert!(path.is_root());
        assert!(path.is_absolute());
        assert!(!path.has_segments());
        assert_eq!(path.segment_count(), 0);
    }

    #[test]
    fn test_module_path_from_str_absolute() {
        let path = ModulePath::from_str("::one").unwrap();
        assert!(!path.is_root());
        assert!(path.is_absolute());
        assert!(path.has_segments());
        assert_eq!(path.segment_count(), 1);
    }

    #[test]
    fn test_module_path_from_str_absolute_two() {
        let path = ModulePath::from_str("::one::two").unwrap();
        assert!(!path.is_root());
        assert!(path.is_absolute());
        assert!(path.has_segments());
        assert_eq!(path.segment_count(), 2);
    }

    #[test]
    fn test_module_path_from_str_relative() {
        let path = ModulePath::from_str("one").unwrap();
        assert!(!path.is_root());
        assert!(!path.is_absolute());
        assert!(path.has_segments());
        assert_eq!(path.segment_count(), 1);
    }

    #[test]
    fn test_module_path_from_str_relative_two() {
        let path = ModulePath::from_str("one::two").unwrap();
        assert!(!path.is_root());
        assert!(!path.is_absolute());
        assert!(path.has_segments());
        assert_eq!(path.segment_count(), 2);
    }

    #[test]
    fn test_module_path_display_default() {
        let path = ModulePath::default();
        assert_eq!(&path.to_string(), "");
    }

    #[test]
    fn test_module_path_display_root() {
        let path = ModulePath::root();
        assert_eq!(&path.to_string(), "::");
    }

    #[test]
    fn test_module_path_display_absolute() {
        let path = ModulePath::absolute(vec![Identifier::from_str("one").unwrap()]);
        assert_eq!(&path.to_string(), "::one");
    }

    #[test]
    fn test_module_path_display_absolute_two() {
        let path = ModulePath::absolute(vec![
            Identifier::from_str("one").unwrap(),
            Identifier::from_str("two").unwrap(),
        ]);
        assert_eq!(&path.to_string(), "::one::two");
    }
    #[test]
    fn test_module_path_display_relative() {
        let path = ModulePath::relative(vec![Identifier::from_str("one").unwrap()]);
        assert_eq!(&path.to_string(), "one");
    }

    #[test]
    fn test_module_path_display_relative_two() {
        let path = ModulePath::relative(vec![
            Identifier::from_str("one").unwrap(),
            Identifier::from_str("two").unwrap(),
        ]);
        assert_eq!(&path.to_string(), "one::two");
    }
}
