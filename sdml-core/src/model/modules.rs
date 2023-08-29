use crate::error::Error;
use crate::model::{
    annotations::{Annotation, HasAnnotations},
    check::Validate,
    definitions::Definition,
    identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
    HasName, HasSourceSpan, Span,
};
use std::{collections::HashSet, fmt::Debug};
use tracing::info;
use url::Url;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::References;

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
// Public Types ❱ Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

///
/// Corresponds the grammar rule `import_statement`.
///
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ImportStatement {
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
    Module(Identifier),
    /// Corresponds to the grammar rule `member_import`.
    Member(QualifiedIdentifier),
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

impl_has_source_span_for!(Module);

impl_has_name_for!(Module);

impl References for Module {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body.referenced_types(names);
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body.referenced_annotations(names);
    }
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

    // --------------------------------------------------------------------------------------------

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

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> Result<bool, Error> {
        self.body.is_complete(self)
    }

    pub fn is_valid(&self, check_constraints: bool) -> Result<bool, Error> {
        self.body.is_valid(check_constraints, self)
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(ModuleBody);

impl_has_annotations_for!(ModuleBody);

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

impl ModuleBody {
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
}

impl Validate for ModuleBody {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        let failed: Result<Vec<bool>, Error> =
            self.annotations().map(|ann| ann.is_complete(top)).collect();
        Ok(failed?.iter().all(|b| *b))
    }

    fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
        for annotation in self.annotations() {
            if !annotation.is_valid(check_constraints, top)? {
                info!("Annotation {annotation:?} is not valid");
            }
        }
        for definition in self.definitions() {
            if !definition.is_valid(check_constraints, top)? {
                info!("Definition {} is not valid", definition.name());
            }
        }
        Ok(true)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

impl FromIterator<Import> for ImportStatement {
    fn from_iter<T: IntoIterator<Item = Import>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl_has_source_span_for!(ImportStatement);

impl ImportStatement {
    pub fn new(imports: Vec<Import>) -> Self {
        Self {
            span: None,
            imports,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_imports_empty(&self) -> bool {
        self.imports.is_empty()
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
    pub fn add_to_imports(&mut self, value: Import) {
        self.imports.push(value)
    }
    pub fn extend_imports<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Import>,
    {
        self.imports.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn as_slice(&self) -> &[Import] {
        self.imports.as_slice()
    }

    // --------------------------------------------------------------------------------------------

    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.imports()
            .map(|imp| match imp {
                Import::Module(v) => v,
                Import::Member(v) => v.module(),
            })
            .collect()
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

impl From<Identifier> for Import {
    fn from(v: Identifier) -> Self {
        Self::Module(v)
    }
}

impl From<QualifiedIdentifier> for Import {
    fn from(v: QualifiedIdentifier) -> Self {
        Self::Member(v)
    }
}

enum_display_impl!(Import => Module, Member);

impl Import {
    pub fn has_source_span(&self) -> bool {
        match self {
            Self::Module(v) => v.has_source_span(),
            Self::Member(v) => v.has_source_span(),
        }
    }

    pub fn source_span(&self) -> Option<&Span> {
        match self {
            Self::Module(v) => v.source_span(),
            Self::Member(v) => v.source_span(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
