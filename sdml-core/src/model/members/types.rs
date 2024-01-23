use crate::cache::ModuleCache;
use crate::error::Error;
use crate::model::check::{find_definition, Validate};
use crate::model::definitions::Definition;
use crate::model::identifiers::IdentifierReference;
use crate::model::modules::Module;
use crate::model::{References, Span};
use crate::syntax::KW_TYPE_UNKNOWN;
use std::collections::HashSet;
use std::fmt::{Debug, Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tracing::trace;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
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

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Type Reference
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `type_reference`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TypeReference {
    Unknown,
    // `builtin_simple_type` is converted into a `IdentifierReference`
    Type(IdentifierReference),
    FeatureSet(IdentifierReference),
    MappingType(MappingType),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Mapping Type
// ------------------------------------------------------------------------------------------------

/// Corresponds to the definition component within grammar rule `mapping_type`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MappingType {
    span: Option<Span>,
    domain: Box<TypeReference>,
    range: Box<TypeReference>,
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
// Implementations ❱ Members ❱ Type Reference
// ------------------------------------------------------------------------------------------------

impl Display for TypeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TypeReference::Unknown => KW_TYPE_UNKNOWN.to_string(),
                TypeReference::Type(v) => v.to_string(),
                TypeReference::FeatureSet(v) => v.to_string(),
                TypeReference::MappingType(v) => v.to_string(),
            }
        )
    }
}

impl IdentifierReference {
    pub fn into_type_reference(self) -> TypeReference {
        TypeReference::Type(self)
    }

    pub fn into_featureset_reference(self) -> TypeReference {
        TypeReference::FeatureSet(self)
    }
}

impl References for TypeReference {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match self {
            TypeReference::Unknown => {}
            TypeReference::Type(v) => {
                names.insert(v);
            }
            TypeReference::FeatureSet(v) => {
                names.insert(v);
            }
            TypeReference::MappingType(v) => {
                v.referenced_types(names);
            }
        }
    }
}

impl Validate for TypeReference {
    fn is_complete(&self, top: &Module, cache: &ModuleCache) -> Result<bool, Error> {
        trace!("TypeReference::is_complete");
        match self {
            TypeReference::Unknown => Ok(false),
            TypeReference::Type(_) => Ok(true),
            TypeReference::FeatureSet(_) => Ok(true),
            TypeReference::MappingType(v) => v.is_complete(top, cache),
        }
    }

    fn is_valid(
        &self,
        check_constraints: bool,
        top: &Module,
        cache: &ModuleCache,
    ) -> Result<bool, Error> {
        trace!("TypeReference::is_valid");
        match self {
            TypeReference::Unknown => Ok(true),
            TypeReference::Type(name) => Ok(find_definition(name, top, cache)
                .map(|defn| {
                    !matches!(defn, Definition::Property(_)) && !matches!(defn, Definition::Rdf(_))
                })
                .unwrap_or_default()),
            TypeReference::FeatureSet(name) => Ok(find_definition(name, top, cache)
                .map(|defn| matches!(defn, Definition::Union(_)))
                .unwrap_or_default()),
            TypeReference::MappingType(v) => v.is_valid(check_constraints, top, cache),
        }
    }
}

impl TypeReference {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Type (IdentifierReference) => is_reference, as_reference);

    is_as_variant!(FeatureSet (IdentifierReference) => is_featureset, as_featureset);

    is_as_variant!(MappingType (MappingType) => is_mapping_type, as_mapping_type);

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Mapping Type
// ------------------------------------------------------------------------------------------------

impl Display for MappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.domain, self.range)
    }
}

impl_has_source_span_for!(MappingType);

impl References for MappingType {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.domain.referenced_types(names);
        self.range.referenced_types(names);
    }
}

impl Validate for MappingType {
    fn is_complete(&self, top: &Module, cache: &ModuleCache) -> Result<bool, Error> {
        trace!("MappingType::is_complete");
        Ok(self.domain.is_complete(top, cache)? && self.range.is_complete(top, cache)?)
    }

    fn is_valid(
        &self,
        check_constraints: bool,
        top: &Module,
        cache: &ModuleCache,
    ) -> Result<bool, Error> {
        trace!("MappingType::is_valid");
        Ok(self.domain().is_valid(check_constraints, top, cache)?
            && self.range().is_valid(check_constraints, top, cache)?)
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

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub domain, set_domain => boxed into TypeReference);

    get_and_set!(pub range, set_range => boxed into TypeReference);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
