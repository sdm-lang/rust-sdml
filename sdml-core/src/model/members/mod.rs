use crate::model::{Identifier, IdentifierReference, QualifiedIdentifier, Span};
use crate::syntax::{
    KW_ORDERING_ORDERED, KW_ORDERING_UNORDERED, KW_TYPE_UNKNOWN, KW_UNIQUENESS_NONUNIQUE,
    KW_UNIQUENESS_UNIQUE,
};
use std::fmt::{Debug, Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Type Reference
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `type_reference`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TypeReference {
    Unknown,
    Reference(IdentifierReference),
    // builtin_simple_type is converted into a reference
    MappingType(MappingType),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Mapping Type
// ------------------------------------------------------------------------------------------------

/// Corresponds to the definition component within grammar rule `mapping_type`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MappingType {
    domain: Box<TypeReference>,
    range: Box<TypeReference>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `cardinality`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Cardinality {
    span: Option<Span>,
    ordering: Option<Ordering>,
    uniqueness: Option<Uniqueness>,
    min: u32,
    max: Option<u32>,
}

pub const DEFAULT_BY_REFERENCE_CARDINALITY: Cardinality = Cardinality::one();
pub const DEFAULT_BY_VALUE_CARDINALITY: Cardinality = Cardinality::zero_or_one();

pub const TYPE_BAG_CARDINALITY: Cardinality = Cardinality::zero_or_more();
pub const TYPE_LIST_CARDINALITY: Cardinality =
    Cardinality::zero_or_more().with_ordering(Ordering::Ordered);
pub const TYPE_SET_CARDINALITY: Cardinality =
    Cardinality::zero_or_more().with_uniqueness(Uniqueness::Unique);
pub const TYPE_ORDERED_SET_CARDINALITY: Cardinality = Cardinality::zero_or_more()
    .with_ordering(Ordering::Ordered)
    .with_uniqueness(Uniqueness::Unique);
pub const TYPE_MAYBE_CARDINALITY: Cardinality = Cardinality::zero_or_one();

/// Corresponds to the grammar rule `sequence_ordering`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Ordering {
    Ordered,
    Unordered,
}

/// Corresponds to the grammar rule `sequence_uniqueness`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Uniqueness {
    Unique,
    Nonunique,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PseudoSequenceType {
    Maybe,
    Bag,
    List,
    Set,
    UnorderedSet,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! member_model_element_impl {
    ($type: ty) => {
        impl ModelElement for $type {
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

            delegate!(is_complete, bool, fn inner);
            delegate!(referenced_annotations, HashSet<&IdentifierReference>, fn inner);

            fn referenced_types(&self) -> HashSet<&IdentifierReference> {
                if let Some(target_type) = self.inner().target_type() {
                    [target_type].into_iter().collect()
                } else {
                    Default::default()
                }
            }
        }
    };
}

macro_rules! member_impl {
    ($type: ty, $inner_type: ty, $def_type: ty) => {
        impl $type {
            pub fn new_with_role(name: Identifier, role: Identifier) -> Self {
                Self {
                    span: None,
                    name,
                    inner: role.into(),
                }
            }

            pub fn new_with_definition(name: Identifier, def: $def_type) -> Self {
                Self {
                    span: None,
                    name,
                    inner: def.into(),
                }
            }

            pub fn with_ts_span(self, ts_span: Span) -> Self {
                Self {
                    span: Some(ts_span),
                    ..self
                }
            }

            pub fn inner(&self) -> &$inner_type {
                &self.inner
            }
            pub fn set_inner(&mut self, inner: $inner_type) {
                self.inner = inner;
            }

            delegate!(pub is_property_role, bool, fn inner);
            delegate!(pub as_property_role, Option<&Identifier>, fn inner);

            delegate!(pub is_defined, bool, fn inner);
            delegate!(pub as_defined, Option<&$def_type>, fn inner);

            delegate!(pub target_type, Option<&IdentifierReference>, fn inner);
        }
    };
}

macro_rules! member_inner_impl {
    ($inner_type: ty, $def_type: ty) => {
        impl From<Identifier> for $inner_type {
            fn from(value: Identifier) -> Self {
                Self::PropertyRole(value)
            }
        }

        impl From<$def_type> for $inner_type {
            fn from(value: $def_type) -> Self {
                Self::Defined(value)
            }
        }

        impl $inner_type {
            pub fn is_property_role(&self) -> bool {
                matches!(self, Self::PropertyRole(_))
            }
            pub fn as_property_role(&self) -> Option<&Identifier> {
                match self {
                    Self::PropertyRole(v) => Some(v),
                    _ => None,
                }
            }

            pub fn is_defined(&self) -> bool {
                matches!(self, Self::Defined(_))
            }
            pub fn as_defined(&self) -> Option<&$def_type> {
                match self {
                    Self::Defined(v) => Some(v),
                    _ => None,
                }
            }

            pub fn target_type(&self) -> Option<&IdentifierReference> {
                if let Self::Defined(defined) = self {
                    defined.target_type().as_reference()
                } else {
                    // TODO: lookup the property role to check.
                    None
                }
            }

            pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
                if let Self::Defined(defined) = self {
                    defined.referenced_annotations()
                } else {
                    // TODO: lookup the property role to check.
                    Default::default()
                }
            }

            pub fn is_complete(&self) -> bool {
                if let Self::Defined(defined) = self {
                    defined.is_complete()
                } else {
                    // TODO: lookup the property role to check.
                    true
                }
            }
        }
    };
}

macro_rules! member_def_impl {
    () => {
        pub fn target_type(&self) -> &TypeReference {
            &self.target_type
        }
        pub fn set_target_type(&mut self, target_type: TypeReference) {
            self.target_type = target_type;
        }

        pub fn body(&self) -> Option<&AnnotationOnlyBody> {
            self.body.as_ref()
        }
        pub fn set_body(&mut self, body: AnnotationOnlyBody) {
            self.body = Some(body);
        }
        pub fn unset_body(&mut self) {
            self.body = None;
        }

        pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
            self.body()
                .map(|b| b.referenced_annotations())
                .unwrap_or_default()
        }

        pub fn is_complete(&self) -> bool {
            self.target_type().is_complete()
        }
    };
}

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
                TypeReference::Reference(v) => v.to_string(),
                TypeReference::MappingType(v) => v.to_string(),
            }
        )
    }
}

impl From<IdentifierReference> for TypeReference {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl From<Identifier> for TypeReference {
    fn from(value: Identifier) -> Self {
        Self::Reference(value.into())
    }
}

impl From<QualifiedIdentifier> for TypeReference {
    fn from(value: QualifiedIdentifier) -> Self {
        Self::Reference(value.into())
    }
}

impl TypeReference {
    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }
    pub fn as_reference(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Reference(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    pub fn is_complete(&self) -> bool {
        match self {
            TypeReference::Unknown => false,
            TypeReference::Reference(_) => true,
            TypeReference::MappingType(v) => v.is_complete(),
        }
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

impl MappingType {
    pub fn new<T1, T2>(domain: T1, range: T2) -> Self
    where
        T1: Into<TypeReference>,
        T2: Into<TypeReference>,
    {
        Self {
            domain: Box::new(domain.into()),
            range: Box::new(range.into()),
        }
    }

    pub fn domain(&self) -> &TypeReference {
        &self.domain
    }
    pub fn set_domain<T>(&mut self, domain: T)
    where
        T: Into<TypeReference>,
    {
        self.domain = Box::new(domain.into());
    }

    pub fn range(&self) -> &TypeReference {
        &self.range
    }
    pub fn set_range<T>(&mut self, range: T)
    where
        T: Into<TypeReference>,
    {
        self.range = Box::new(range.into());
    }

    pub fn is_complete(&self) -> bool {
        self.range.is_complete()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

impl Display for Cardinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}{}{}..{}}}",
            self.ordering.map(|c| format!("{} ", c)).unwrap_or_default(),
            self.uniqueness
                .map(|c| format!("{} ", c))
                .unwrap_or_default(),
            self.min_occurs(),
            self.max_occurs().map(|i| i.to_string()).unwrap_or_default()
        )
    }
}

impl From<u32> for Cardinality {
    fn from(value: u32) -> Self {
        Self::new_single(value)
    }
}

impl Cardinality {
    pub const fn new_range(min: u32, max: u32) -> Self {
        Self {
            ordering: None,
            uniqueness: None,
            span: None,
            min,
            max: Some(max),
        }
    }

    pub const fn new_unbounded(min: u32) -> Self {
        Self {
            ordering: None,
            uniqueness: None,
            span: None,
            min,
            max: None,
        }
    }

    pub const fn new_single(min_and_max: u32) -> Self {
        Self {
            ordering: None,
            uniqueness: None,
            span: None,
            min: min_and_max,
            max: Some(min_and_max),
        }
    }

    #[inline(always)]
    pub const fn one() -> Self {
        Self::new_single(1)
    }

    #[inline(always)]
    pub const fn zero_or_one() -> Self {
        Self::new_range(0, 1)
    }

    #[inline(always)]
    pub const fn one_or_more() -> Self {
        Self::new_unbounded(1)
    }

    #[inline(always)]
    pub const fn zero_or_more() -> Self {
        Self::new_unbounded(0)
    }

    // --------------------------------------------------------------------------------------------

    pub const fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }
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

    pub const fn with_ordering(self, ordering: Ordering) -> Self {
        Self {
            ordering: Some(ordering),
            ..self
        }
    }

    #[inline(always)]
    pub fn ordering(&self) -> Option<Ordering> {
        self.ordering
    }

    #[inline(always)]
    pub fn set_ordering(&mut self, ordering: Ordering) {
        self.ordering = Some(ordering);
    }

    #[inline(always)]
    pub fn unset_ordering(&mut self) {
        self.ordering = None;
    }

    #[inline(always)]
    pub fn is_ordered(&self) -> Option<bool> {
        self.ordering().map(|o| o == Ordering::Ordered)
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub const fn with_uniqueness(self, uniqueness: Uniqueness) -> Self {
        Self {
            uniqueness: Some(uniqueness),
            ..self
        }
    }

    #[inline(always)]
    pub fn uniqueness(&self) -> Option<Uniqueness> {
        self.uniqueness
    }

    #[inline(always)]
    pub fn set_uniqueness(&mut self, uniqueness: Uniqueness) {
        self.uniqueness = Some(uniqueness);
    }

    #[inline(always)]
    pub fn unset_uniqueness(&mut self) {
        self.uniqueness = None;
    }

    #[inline(always)]
    pub fn is_unique(&self) -> Option<bool> {
        self.uniqueness().map(|u| u == Uniqueness::Unique)
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn min_occurs(&self) -> u32 {
        self.min
    }

    #[inline(always)]
    pub fn set_min_occurs(&mut self, min: u32) {
        self.min = min;
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn max_occurs(&self) -> Option<u32> {
        self.max
    }

    #[inline(always)]
    pub fn set_max_occurs(&mut self, max: u32) {
        self.max = Some(max);
    }

    #[inline(always)]
    pub fn unset_max_occurs(&mut self) {
        self.max = None;
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn is_optional(&self) -> bool {
        self.min_occurs() == 0
    }

    #[inline(always)]
    pub fn is_required(&self) -> bool {
        !self.is_optional()
    }

    #[inline(always)]
    pub fn is_range(&self) -> bool {
        self.max.map(|i| i != self.min).unwrap_or(true)
    }

    #[inline(always)]
    pub fn is_unbounded(&self) -> bool {
        self.max_occurs().is_none()
    }

    #[inline(always)]
    pub fn is_exactly(&self, value: u32) -> bool {
        self.min_occurs() == value && self.max_occurs().map(|i| i == value).unwrap_or(false)
    }

    // --------------------------------------------------------------------------------------------

    pub fn sequence_type(&self) -> PseudoSequenceType {
        match (
            self.is_ordered(),
            self.is_unique(),
            self.min,
            self.max.unwrap_or(self.min),
        ) {
            (_, _, 0, 1) => PseudoSequenceType::Maybe,
            (Some(true), Some(true), _, _) => PseudoSequenceType::UnorderedSet,
            (Some(false), Some(true), _, _) => PseudoSequenceType::Set,
            (Some(true), Some(false), _, _) => PseudoSequenceType::List,
            _ => PseudoSequenceType::Bag,
        }
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn to_uml_string(&self) -> String {
        if self.is_range() {
            format!(
                "{}{}{}..{}",
                if let Some(ordering) = self.ordering() {
                    format!("{{{}}} ", ordering)
                } else {
                    String::new()
                },
                if let Some(uniqueness) = self.uniqueness() {
                    format!("{{{}}} ", uniqueness)
                } else {
                    String::new()
                },
                self.min_occurs(),
                self.max_occurs()
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "*".to_string())
            )
        } else {
            self.min.to_string()
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Ordering {
    fn default() -> Self {
        Self::Unordered
    }
}

impl Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ordering::Ordered => KW_ORDERING_ORDERED,
                Ordering::Unordered => KW_ORDERING_UNORDERED,
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Uniqueness {
    fn default() -> Self {
        Self::Nonunique
    }
}

impl Display for Uniqueness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Uniqueness::Unique => KW_UNIQUENESS_UNIQUE,
                Uniqueness::Nonunique => KW_UNIQUENESS_NONUNIQUE,
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod by_reference;
pub use by_reference::{ByReferenceMember, ByReferenceMemberDef, ByReferenceMemberInner};

mod by_value;
pub use by_value::{ByValueMember, ByValueMemberDef, ByValueMemberInner};

mod identity;
pub use identity::{IdentityMember, IdentityMemberDef, IdentityMemberInner};
