use crate::syntax::{KW_TYPE_UNKNOWN, KW_ORDERING_ORDERED, KW_ORDERING_UNORDERED, KW_UNIQUENESS_UNIQUE, KW_UNIQUENESS_NONUNIQUE};

use super::{
    AnnotationOnlyBody, Identifier, IdentifierReference, ModelElement, QualifiedIdentifier, Span,
};
use std::{collections::HashSet, fmt::{Debug, Display}};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Identity
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `identify_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IdentityMember {
    span: Option<Span>,
    name: Identifier,
    inner: IdentityMemberInner,
}

/// Corresponds to the choice component within grammar rule `identity_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum IdentityMemberInner {
    PropertyRole(Identifier),
    Defined(IdentityMemberDef),
}

/// Corresponds to the definition component within grammar rule `identity_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IdentityMemberDef {
    target_type: TypeReference,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ ByValue
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `by_value_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ByValueMember {
    span: Option<Span>,
    name: Identifier,
    inner: ByValueMemberInner,
}

/// Corresponds to the choice component within grammar rule `by_value_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ByValueMemberInner {
    PropertyRole(Identifier),
    Defined(ByValueMemberDef),
}

/// Corresponds to the definition component within grammar rule `by_value_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ByValueMemberDef {
    target_type: TypeReference,
    target_cardinality: Cardinality,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ ByReference
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ByReferenceMember {
    span: Option<Span>,
    name: Identifier,
    inner: ByReferenceMemberInner,
}

/// Corresponds to the choice component within grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ByReferenceMemberInner {
    PropertyRole(Identifier),
    Defined(ByReferenceMemberDef),
}

/// Corresponds to the definition component within grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ByReferenceMemberDef {
    target_type: TypeReference,
    inverse_name: Option<Identifier>,
    target_cardinality: Cardinality,
    body: Option<AnnotationOnlyBody>,
}

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
    ordering: Option<Ordering>,
    uniqueness: Option<Uniqueness>,
    span: Option<Span>,
    min: u32,
    max: Option<u32>,
}

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
// Implementations ❱ Members ❱ Identity
// ------------------------------------------------------------------------------------------------

member_model_element_impl!(IdentityMember);

member_impl!(IdentityMember, IdentityMemberInner, IdentityMemberDef);

member_inner_impl!(IdentityMemberInner, IdentityMemberDef);

impl IdentityMemberDef {
    pub fn new(target_type: TypeReference) -> Self {
        Self {
            target_type,
            body: None,
        }
    }
    pub fn new_named(target_type: IdentifierReference) -> Self {
        Self {
            target_type: target_type.into(),
            body: None,
        }
    }
    pub fn new_unknown() -> Self {
        Self {
            target_type: TypeReference::Unknown,
            body: None,
        }
    }
    member_def_impl!();
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ ByValue
// ------------------------------------------------------------------------------------------------

member_model_element_impl!(ByValueMember);

member_impl!(ByValueMember, ByValueMemberInner, ByValueMemberDef);

member_inner_impl!(ByValueMemberInner, ByValueMemberDef);

impl ByValueMemberDef {
    pub fn new(target_type: TypeReference) -> Self {
        Self {
            target_type,
            target_cardinality: DEFAULT_BY_VALUE_CARDINALITY,
            body: None,
        }
    }
    pub fn new_named(target_type: IdentifierReference) -> Self {
        Self {
            target_type: target_type.into(),
            target_cardinality: DEFAULT_BY_VALUE_CARDINALITY,
            body: None,
        }
    }
    pub fn new_unknown() -> Self {
        Self {
            target_type: TypeReference::Unknown,
            target_cardinality: DEFAULT_BY_VALUE_CARDINALITY,
            body: None,
        }
    }

    pub fn target_cardinality(&self) -> &Cardinality {
        &self.target_cardinality
    }

    pub fn set_target_cardinality(&mut self, target_cardinality: Cardinality) {
        self.target_cardinality = target_cardinality;
    }

    member_def_impl!();
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ ByReference
// ------------------------------------------------------------------------------------------------

member_model_element_impl!(ByReferenceMember);

member_impl!(
    ByReferenceMember,
    ByReferenceMemberInner,
    ByReferenceMemberDef
);

member_inner_impl!(ByReferenceMemberInner, ByReferenceMemberDef);

impl ByReferenceMemberDef {
    pub fn new(target_type: TypeReference) -> Self {
        Self {
            target_type,
            target_cardinality: DEFAULT_BY_REFERENCE_CARDINALITY,
            inverse_name: None,
            body: None,
        }
    }
    pub fn new_named(target_type: IdentifierReference) -> Self {
        Self {
            target_type: target_type.into(),
            target_cardinality: DEFAULT_BY_REFERENCE_CARDINALITY,
            inverse_name: None,
            body: None,
        }
    }
    pub fn new_unknown() -> Self {
        Self {
            target_type: TypeReference::Unknown,
            target_cardinality: DEFAULT_BY_REFERENCE_CARDINALITY,
            inverse_name: None,
            body: None,
        }
    }

    pub fn target_cardinality(&self) -> &Cardinality {
        &self.target_cardinality
    }

    pub fn set_target_cardinality(&mut self, target_cardinality: Cardinality) {
        self.target_cardinality = target_cardinality;
    }

    pub fn inverse_name(&self) -> Option<&Identifier> {
        self.inverse_name.as_ref()
    }

    pub fn set_inverse_name(&mut self, inverse_name: Identifier) {
        self.inverse_name = Some(inverse_name);
    }

    pub fn unset_inverse_name(&mut self) {
        self.inverse_name = None;
    }

    member_def_impl!();
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Type Reference
// ------------------------------------------------------------------------------------------------

impl Display for TypeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TypeReference::Unknown => KW_TYPE_UNKNOWN.to_string(),
            TypeReference::Reference(v) => v.to_string(),
            TypeReference::MappingType(v) => v.to_string(),
        })
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
        !self.is_unknown()
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
    pub fn new<T1,T2>(domain: T1, range: T2) -> Self
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
        T: Into<TypeReference>
    {
        self.domain = Box::new(domain.into());
    }

    pub fn range(&self) -> &TypeReference {
        &self.range
    }
    pub fn set_range<T>(&mut self, range: T)
    where
        T: Into<TypeReference>
    {
        self.range = Box::new(range.into());
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

pub const DEFAULT_BY_VALUE_CARDINALITY: Cardinality = Cardinality::new_single(1);

pub const DEFAULT_BY_REFERENCE_CARDINALITY: Cardinality = Cardinality::new_range(0, 1);

impl Display for Cardinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}{}{}..{}}}",
            self.ordering.map(|c|format!("{} ", c)).unwrap_or_default(),
            self.uniqueness.map(|c|format!("{} ", c)).unwrap_or_default(),
            self.min_occurs(),
            self.max_occurs()
                .map(|i| i.to_string())
                .unwrap_or_default()
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

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
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

    pub fn with_ordering(self, ordering: Ordering) -> Self {
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
    pub fn is_ordered(&self) -> Option<bool> {
        self.ordering().map(|o|o == Ordering::Ordered)
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn with_uniqueness(self, uniqueness: Uniqueness) -> Self {
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
    pub fn is_unique(&self) -> Option<bool> {
        self.uniqueness().map(|u|u == Uniqueness::Unique)
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
        match (self.is_ordered(), self.is_unique(), self.min, self.max.unwrap_or(self.min)) {
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
                "{}..{}",
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
        write!(f, "{}", match self {
            Ordering::Ordered => KW_ORDERING_ORDERED,
            Ordering::Unordered => KW_ORDERING_UNORDERED,
        })
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
        write!(f, "{}", match self {
            Uniqueness::Unique => KW_UNIQUENESS_UNIQUE,
            Uniqueness::Nonunique => KW_UNIQUENESS_NONUNIQUE,
        })
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
