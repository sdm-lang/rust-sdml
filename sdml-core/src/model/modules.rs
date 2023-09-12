use crate::error::Error;
use crate::model::{
    annotations::{Annotation, HasAnnotations},
    check::Validate,
    definitions::Definition,
    identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
    HasName, Span,
};
use std::{collections::HashSet, fmt::Debug};
use tracing::info;
use url::Url;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::definitions::{
    DatatypeDef, EntityDef, EnumDef, EventDef, FeatureSetDef, PropertyDef, StructureDef, UnionDef,
};
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

impl_has_body_for!(Module, ModuleBody);

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
    // Constructors
    // --------------------------------------------------------------------------------------------

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
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn with_base(self, base: Url) -> Self {
        Self {
            base: Some(base),
            ..self
        }
    }

    get_and_set!(pub base, set_base, unset_base => optional has_base, Url);

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
    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set_vec!(
        pub
        has has_imports,
        imports_len,
        imports,
        imports_mut,
        add_to_imports,
        extend_imports
            => imports, ImportStatement
    );

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

    // --------------------------------------------------------------------------------------------

    get_and_set_vec!(
        pub
        has has_definitions,
        definitions_len,
        definitions,
        definitions_mut,
        add_to_definitions,
        extend_definitions
            => definitions, Definition
    );

    #[inline]
    pub fn datatype_definitions(&self) -> impl Iterator<Item = &DatatypeDef> {
        self.definitions.iter().filter_map(|d| match d {
            Definition::Datatype(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn entity_definitions(&self) -> impl Iterator<Item = &EntityDef> {
        self.definitions.iter().filter_map(|d| match d {
            Definition::Entity(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn enum_definitions(&self) -> impl Iterator<Item = &EnumDef> {
        self.definitions.iter().filter_map(|d| match d {
            Definition::Enum(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn event_definitions(&self) -> impl Iterator<Item = &EventDef> {
        self.definitions.iter().filter_map(|d| match d {
            Definition::Event(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn feature_definitions(&self) -> impl Iterator<Item = &FeatureSetDef> {
        self.definitions.iter().filter_map(|d| match d {
            Definition::FeatureSet(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn property_definitions(&self) -> impl Iterator<Item = &PropertyDef> {
        self.definitions.iter().filter_map(|d| match d {
            Definition::Property(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn structure_definitions(&self) -> impl Iterator<Item = &StructureDef> {
        self.definitions.iter().filter_map(|d| match d {
            Definition::Structure(v) => Some(v),
            _ => None,
        })
    }

    #[inline]
    pub fn union_definitions(&self) -> impl Iterator<Item = &UnionDef> {
        self.definitions.iter().filter_map(|d| match d {
            Definition::Union(v) => Some(v),
            _ => None,
        })
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
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(imports: Vec<Import>) -> Self {
        Self {
            span: None,
            imports,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set_vec!(
        pub
        has has_imports,
        imports_len,
        imports,
        imports_mut,
        add_to_imports,
        extend_imports
            => imports, Import
    );

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

impl_has_source_span_for!(Import => variants Module, Member);

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
