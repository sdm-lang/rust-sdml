use super::{Identifier, QualifiedIdentifier, Span};
use std::{collections::HashSet, fmt::Debug, hash::Hash};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Imports
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
// Implementations ❱ Imports
// ------------------------------------------------------------------------------------------------

impl FromIterator<Import> for ImportStatement {
    fn from_iter<T: IntoIterator<Item = Import>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl ImportStatement {
    pub fn new(imports: Vec<Import>) -> Self {
        Self {
            span: None,
            imports,
        }
    }

    // --------------------------------------------------------------------------------------------

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
    pub fn has_ts_span(&self) -> bool {
        match self {
            Self::Module(v) => v.has_ts_span(),
            Self::Member(v) => v.has_ts_span(),
        }
    }

    pub fn ts_span(&self) -> Option<&Span> {
        match self {
            Self::Module(v) => v.ts_span(),
            Self::Member(v) => v.ts_span(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
