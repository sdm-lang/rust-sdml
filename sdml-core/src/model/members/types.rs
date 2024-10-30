use crate::load::ModuleLoader;
use crate::model::check::{find_definition, MaybeIncomplete, Validate};
use crate::model::definitions::Definition;
use crate::model::identifiers::IdentifierReference;
use crate::model::modules::Module;
use crate::model::{HasSourceSpan, References, Span};
use crate::stdlib::is_builtin_type_name;
use crate::store::ModuleStore;
use crate::syntax::KW_TYPE_UNKNOWN;
use sdml_errors::diagnostics::functions::{
    property_incompatible_usage, rdf_definition_incompatible_usage, type_class_incompatible_usage,
    type_definition_not_found,
};
use std::collections::HashSet;
use std::fmt::{Debug, Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Types
// ------------------------------------------------------------------------------------------------

pub trait HasType {
    fn target_type(&self) -> &TypeReference;
    fn set_target_type(&mut self, target_type: TypeReference);
    fn is_unknown_type(&self) -> bool {
        matches!(self.target_type(), TypeReference::Unknown)
    }
    fn is_named_type(&self) -> bool {
        matches!(self.target_type(), TypeReference::Type(_))
    }
    fn is_mapping_type(&self) -> bool {
        matches!(self.target_type(), TypeReference::MappingType(_))
    }
}

/// Corresponds to the grammar rule `type_reference`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TypeReference {
    Unknown,
    /// `builtin_simple_type` is converted into a `IdentifierReference`
    Type(IdentifierReference),
    MappingType(MappingType),
}

/// Corresponds to the definition component within grammar rule `mapping_type`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MappingType {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    domain: Box<TypeReference>,
    range: Box<TypeReference>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ TypeReference
// ------------------------------------------------------------------------------------------------

impl From<&IdentifierReference> for TypeReference {
    fn from(value: &IdentifierReference) -> Self {
        Self::Type(value.clone())
    }
}

impl From<IdentifierReference> for TypeReference {
    fn from(value: IdentifierReference) -> Self {
        Self::Type(value)
    }
}

impl From<&MappingType> for TypeReference {
    fn from(value: &MappingType) -> Self {
        Self::MappingType(value.clone())
    }
}

impl From<MappingType> for TypeReference {
    fn from(value: MappingType) -> Self {
        Self::MappingType(value)
    }
}

impl Display for TypeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TypeReference::Unknown => KW_TYPE_UNKNOWN.to_string(),
                TypeReference::Type(v) => v.to_string(),
                TypeReference::MappingType(v) => v.to_string(),
            }
        )
    }
}

impl IdentifierReference {
    pub fn into_type_reference(self) -> TypeReference {
        TypeReference::Type(self)
    }
}

impl References for TypeReference {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match self {
            TypeReference::Unknown => {}
            TypeReference::Type(v) => {
                names.insert(v);
            }
            TypeReference::MappingType(v) => {
                v.referenced_types(names);
            }
        }
    }
}

impl MaybeIncomplete for TypeReference {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        match self {
            TypeReference::Unknown => true,
            TypeReference::MappingType(v) => v.is_incomplete(top, cache),
            _ => false,
        }
    }
}

impl Validate for TypeReference {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        match self {
            TypeReference::Unknown => {}
            TypeReference::Type(name) => match find_definition(name, top, cache) {
                None => {
                    if !name
                        .as_identifier()
                        .map(is_builtin_type_name)
                        .unwrap_or_default()
                    {
                        loader
                            .report(&type_definition_not_found(
                                top.file_id().copied().unwrap_or_default(),
                                name.source_span().as_ref().map(|span| (*span).into()),
                                name,
                            ))
                            .unwrap()
                    }
                }
                Some(Definition::TypeClass(_)) => loader
                    .report(&type_class_incompatible_usage(
                        top.file_id().copied().unwrap_or_default(),
                        name.source_span().as_ref().map(|span| (*span).into()),
                        name,
                    ))
                    .unwrap(),
                Some(Definition::Property(_)) => loader
                    .report(&property_incompatible_usage(
                        top.file_id().copied().unwrap_or_default(),
                        name.source_span().as_ref().map(|span| (*span).into()),
                        name,
                    ))
                    .unwrap(),
                Some(Definition::Rdf(defn)) => {
                    if !(defn.is_datatype() || defn.is_class()) {
                        loader
                            .report(&rdf_definition_incompatible_usage(
                                top.file_id().copied().unwrap_or_default(),
                                name.source_span().as_ref().map(|span| (*span).into()),
                                name,
                            ))
                            .unwrap()
                    }
                }
                _ => {}
            },
            TypeReference::MappingType(v) => v.validate(top, cache, loader, check_constraints),
        };
    }
}

impl TypeReference {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_reference(&self) -> bool {
        match self {
            Self::Type(_) => true,
            _ => false,
        }
    }

    pub const fn as_reference(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Type(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_mapping_type(&self) -> bool {
        match self {
            Self::MappingType(_) => true,
            _ => false,
        }
    }

    pub const fn as_mapping_type(&self) -> Option<&MappingType> {
        match self {
            Self::MappingType(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ MappingType
// ------------------------------------------------------------------------------------------------

impl Display for MappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.domain, self.range)
    }
}

impl HasSourceSpan for MappingType {
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

impl References for MappingType {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.domain.referenced_types(names);
        self.range.referenced_types(names);
    }
}

impl MaybeIncomplete for MappingType {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        self.domain.is_incomplete(top, cache) || self.range.is_incomplete(top, cache)
    }
}

impl Validate for MappingType {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.domain()
            .validate(top, cache, loader, check_constraints);
        self.range().validate(top, cache, loader, check_constraints);
    }
}

impl MappingType {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<T1, T2>(domain: T1, range: T2) -> Self
    where
        T1: Into<TypeReference>,
        T2: Into<TypeReference>,
    {
        Self {
            span: Default::default(),
            domain: Box::new(domain.into()),
            range: Box::new(range.into()),
        }
    }

    pub fn with_domain<T>(self, domain: T) -> Self
    where
        T: Into<TypeReference>,
    {
        let mut self_mut = self;
        self_mut.domain = Box::new(domain.into());
        self_mut
    }

    pub fn with_range<T>(self, range: T) -> Self
    where
        T: Into<TypeReference>,
    {
        let mut self_mut = self;
        self_mut.range = Box::new(range.into());
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn domain(&self) -> &TypeReference {
        &self.domain
    }

    pub fn set_domain<T>(&mut self, domain: T)
    where
        T: Into<TypeReference>,
    {
        self.domain = Box::new(domain.into());
    }

    // --------------------------------------------------------------------------------------------

    pub const fn range(&self) -> &TypeReference {
        &self.range
    }

    pub fn set_range<T>(&mut self, range: T)
    where
        T: Into<TypeReference>,
    {
        self.range = Box::new(range.into());
    }
}
