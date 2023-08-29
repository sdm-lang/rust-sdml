use crate::error::Error;
use crate::model::check::Validate;
use crate::model::identifiers::{Identifier, IdentifierReference, QualifiedIdentifier};
use crate::model::modules::Module;
use crate::model::{References, Span};
use crate::syntax::{
    KW_ORDERING_ORDERED, KW_ORDERING_UNORDERED, KW_TYPE_UNKNOWN, KW_UNIQUENESS_NONUNIQUE,
    KW_UNIQUENESS_UNIQUE,
};
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasCardinality {
    fn target_cardinality(&self) -> &Cardinality;

    fn set_target_cardinality(&mut self, target_cardinality: Cardinality);
}

pub trait HasType {
    fn target_type(&self) -> &TypeReference;
    fn set_target_type(&mut self, target_type: TypeReference);
    fn is_unknown_type(&self) -> bool {
        matches!(self.target_type(), TypeReference::Unknown)
    }
    fn is_named_type(&self) -> bool {
        matches!(self.target_type(), TypeReference::Reference(_))
    }
    fn is_mapping_type(&self) -> bool {
        matches!(self.target_type(), TypeReference::MappingType(_))
    }
}

pub trait Member<'a, D: 'a> {
    fn kind(&'a self) -> &'a MemberKind<D>;

    fn set_kind(&mut self, kind: MemberKind<D>);

    fn is_property_reference(&'a self) -> bool {
        matches!(self.kind(), MemberKind::PropertyReference(_))
    }

    fn as_property_reference(&'a self) -> Option<&'a IdentifierReference> {
        if let MemberKind::PropertyReference(v) = self.kind() {
            Some(v)
        } else {
            None
        }
    }

    fn is_definition(&'a self) -> bool {
        matches!(self.kind(), MemberKind::Definition(_))
    }

    fn as_definition(&'a self) -> Option<&'a D> {
        if let MemberKind::Definition(v) = self.kind() {
            Some(v)
        } else {
            None
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ MemberKind
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum MemberKind<D> {
    PropertyReference(IdentifierReference),
    Definition(D),
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
// Public Types ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `cardinality`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Cardinality {
    span: Option<Span>,
    ordering: Option<Ordering>,
    uniqueness: Option<Uniqueness>,
    range: CardinalityRange,
}

pub const BY_IDENTITY_CARDINALITY: Cardinality = Cardinality::one();
pub const DEFAULT_BY_REFERENCE_CARDINALITY: Cardinality = Cardinality::zero_or_one();
pub const DEFAULT_BY_VALUE_CARDINALITY: Cardinality = Cardinality::one();

pub const TYPE_BAG_CARDINALITY: Cardinality = Cardinality::zero_or_more();
pub const TYPE_LIST_CARDINALITY: Cardinality =
    Cardinality::zero_or_more().with_ordering(Ordering::Ordered);
pub const TYPE_SET_CARDINALITY: Cardinality =
    Cardinality::zero_or_more().with_uniqueness(Uniqueness::Unique);
pub const TYPE_ORDERED_SET_CARDINALITY: Cardinality = Cardinality::zero_or_more()
    .with_ordering(Ordering::Ordered)
    .with_uniqueness(Uniqueness::Unique);
pub const TYPE_MAYBE_CARDINALITY: Cardinality = Cardinality::zero_or_one();

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CardinalityRange {
    span: Option<Span>,
    min: u32,
    max: Option<u32>,
}

pub const BY_IDENTITY_CARDINALITY_RANGE: CardinalityRange = CardinalityRange::one();
pub const DEFAULT_BY_REFERENCE_CARDINALITY_RANGE: CardinalityRange =
    CardinalityRange::zero_or_one();
pub const DEFAULT_BY_VALUE_CARDINALITY_RANGE: CardinalityRange = CardinalityRange::one();

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
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Member Kind
// ------------------------------------------------------------------------------------------------

impl<D> Into<MemberKind<D>> for IdentifierReference {
    fn into(self) -> MemberKind<D> {
        MemberKind::PropertyReference(self)
    }
}

impl<D> Into<MemberKind<D>> for Identifier {
    fn into(self) -> MemberKind<D> {
        MemberKind::PropertyReference(self.into())
    }
}

impl<D> Into<MemberKind<D>> for QualifiedIdentifier {
    fn into(self) -> MemberKind<D> {
        MemberKind::PropertyReference(self.into())
    }
}

impl<D> References for MemberKind<D>
where
    D: References,
{
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        if let Self::Definition(defininition) = self {
            defininition.referenced_annotations(names)
        }
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        if let Self::Definition(defininition) = self {
            defininition.referenced_types(names)
        }
    }
}

impl<D> Validate for MemberKind<D>
where
    D: Validate,
{
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        match self {
            MemberKind::PropertyReference(_) => Ok(true),
            MemberKind::Definition(v) => v.is_complete(top),
        }
    }

    fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
        match self {
            MemberKind::PropertyReference(_) => Ok(true),
            MemberKind::Definition(v) => v.is_valid(check_constraints, top),
        }
    }
}

impl<D> MemberKind<D>
where
    D: HasType,
{
    pub fn target_type(&self) -> Option<&IdentifierReference> {
        if let Self::Definition(defininition) = self {
            defininition.target_type().as_reference()
        } else {
            None
        }
    }
}

impl<D> MemberKind<D> {
    pub fn is_property_reference(&self) -> bool {
        matches!(self, Self::PropertyReference(_))
    }

    pub fn as_property_reference(&self) -> Option<&IdentifierReference> {
        if let Self::PropertyReference(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn is_definition(&self) -> bool {
        matches!(self, Self::Definition(_))
    }

    pub fn as_definition(&self) -> Option<&D> {
        if let Self::Definition(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

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

impl References for TypeReference {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match self {
            TypeReference::Unknown => {}
            TypeReference::Reference(v) => {
                names.insert(v);
            }
            TypeReference::MappingType(v) => {
                v.referenced_types(names);
            }
        }
    }
}

impl Validate for TypeReference {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        match self {
            TypeReference::Unknown => Ok(false),
            TypeReference::Reference(_) => Ok(true),
            TypeReference::MappingType(v) => v.is_complete(top),
        }
    }

    fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
        match self {
            TypeReference::Unknown => Ok(true),
            TypeReference::Reference(_) => todo!(),
            TypeReference::MappingType(v) => v.is_valid(check_constraints, top),
        }
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

    pub fn is_mapping_type(&self) -> bool {
        matches!(self, Self::MappingType(_))
    }

    pub fn as_mapping_type(&self) -> Option<&MappingType> {
        match self {
            Self::MappingType(v) => Some(v),
            _ => None,
        }
    }

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
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        Ok(self.domain.is_complete(top)? && self.range.is_complete(top)?)
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        // TODO: check type references exist
        todo!()
    }
}

impl MappingType {
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

    pub fn domain(&self) -> &TypeReference {
        &self.domain
    }

    pub fn set_domain<T>(&mut self, domain: T)
    where
        T: Into<TypeReference>,
    {
        self.domain = Box::new(domain.into());
    }

    // --------------------------------------------------------------------------------------------

    pub fn range(&self) -> &TypeReference {
        &self.range
    }

    pub fn set_range<T>(&mut self, range: T)
    where
        T: Into<TypeReference>,
    {
        self.range = Box::new(range.into());
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

impl From<CardinalityRange> for Cardinality {
    fn from(range: CardinalityRange) -> Self {
        Self {
            span: Default::default(),
            ordering: Default::default(),
            uniqueness: Default::default(),
            range,
        }
    }
}

impl_has_source_span_for!(Cardinality);

impl Validate for Cardinality {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        self.range.is_complete(top)
    }

    fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
        self.range.is_valid(check_constraints, top)
    }
}

impl Cardinality {
    pub const fn new(
        ordering: Option<Ordering>,
        uniqueness: Option<Uniqueness>,
        range: CardinalityRange,
    ) -> Self {
        Self {
            span: None,
            ordering,
            uniqueness,
            range,
        }
    }

    pub const fn new_range(min: u32, max: u32) -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            range: CardinalityRange::new_range(min, max),
        }
    }

    pub const fn new_unbounded(min: u32) -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            range: CardinalityRange::new_unbounded(min),
        }
    }

    pub const fn new_single(min_and_max: u32) -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            range: CardinalityRange::new_single(min_and_max),
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

    pub fn range(&self) -> &CardinalityRange {
        &self.range
    }

    pub fn set_range(&mut self, range: CardinalityRange) {
        self.range = range;
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn min_occurs(&self) -> u32 {
        self.range.min_occurs()
    }

    #[inline(always)]
    pub fn set_min_occurs(&mut self, min: u32) {
        self.range.set_min_occurs(min);
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn max_occurs(&self) -> Option<u32> {
        self.range.max_occurs()
    }

    #[inline(always)]
    pub fn set_max_occurs(&mut self, max: u32) {
        self.range.set_max_occurs(max);
    }

    #[inline(always)]
    pub fn unset_max_occurs(&mut self) {
        self.range.unset_max_occurs();
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn is_optional(&self) -> bool {
        self.range.is_optional()
    }

    #[inline(always)]
    pub fn is_required(&self) -> bool {
        !self.range.is_optional()
    }

    #[inline(always)]
    pub fn is_range(&self) -> bool {
        self.range.is_range()
    }

    #[inline(always)]
    pub fn is_unbounded(&self) -> bool {
        self.range.is_unbounded()
    }

    #[inline(always)]
    pub fn is_exactly(&self, value: u32) -> bool {
        self.range.is_exactly(value)
    }

    // --------------------------------------------------------------------------------------------

    pub fn sequence_type(&self) -> PseudoSequenceType {
        match (
            self.is_ordered(),
            self.is_unique(),
            self.range.min_occurs(),
            self.range.max_occurs().unwrap_or(self.range.min_occurs()),
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
        format!(
            "{}{}{}",
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
            self.range
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for CardinalityRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}..{}",
            self.min,
            self.max.map(|i| i.to_string()).unwrap_or_default()
        )
    }
}

impl From<u32> for CardinalityRange {
    fn from(value: u32) -> Self {
        Self::new_single(value)
    }
}

impl_has_source_span_for!(CardinalityRange);

impl Validate for CardinalityRange {
    fn is_complete(&self, _top: &Module) -> Result<bool, Error> {
        Ok(true)
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        Ok(if let Some(max) = self.max {
            max >= self.min
        } else {
            true
        })
    }
}

impl CardinalityRange {
    pub const fn new_range(min: u32, max: u32) -> Self {
        assert!(max > 0 && max >= min);
        Self {
            span: None,
            min,
            max: Some(max),
        }
    }

    pub const fn new_unbounded(min: u32) -> Self {
        Self {
            span: None,
            min,
            max: None,
        }
    }

    pub const fn new_single(min_and_max: u32) -> Self {
        assert!(min_and_max > 0);
        Self {
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

    #[inline(always)]
    pub fn min_occurs(&self) -> u32 {
        self.min
    }

    #[inline(always)]
    pub fn set_min_occurs(&mut self, min: u32) {
        if let Some(max) = self.max {
            assert!(min <= max);
        }
        self.min = min;
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn max_occurs(&self) -> Option<u32> {
        self.max
    }

    #[inline(always)]
    pub fn set_max_occurs(&mut self, max: u32) {
        assert!(max > 0 && max >= self.min);
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
        write!(
            f,
            "{}",
            match self {
                Self::Ordered => KW_ORDERING_ORDERED,
                Self::Unordered => KW_ORDERING_UNORDERED,
            }
        )
    }
}

impl FromStr for Ordering {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            KW_ORDERING_ORDERED => Ok(Self::Ordered),
            KW_ORDERING_UNORDERED => Ok(Self::Unordered),
            _ => panic!(),
        }
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
                Self::Unique => KW_UNIQUENESS_UNIQUE,
                Self::Nonunique => KW_UNIQUENESS_NONUNIQUE,
            }
        )
    }
}

impl FromStr for Uniqueness {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            KW_UNIQUENESS_UNIQUE => Ok(Self::Unique),
            KW_UNIQUENESS_NONUNIQUE => Ok(Self::Nonunique),
            _ => panic!(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! impl_member_outer_for {
    ($type: ty, $deftype: ty) => {
        impl $type {
            pub fn new_with_role(role: Identifier, property: IdentifierReference) -> Self {
                Self {
                    span: None,
                    name: role,
                    kind: property.into(),
                }
            }

            pub fn new_with_definition(name: Identifier, definition: $deftype) -> Self {
                Self {
                    span: None,
                    name,
                    kind: definition.into(),
                }
            }

            // --------------------------------------------------------------------------------------------

            delegate!(pub target_type, Option<&IdentifierReference>, kind);
        }
    };
}

macro_rules! impl_member_def_references_for {
    ($type: ty) => {
        impl References for $type {
            fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
                self.body
                    .as_ref()
                    .map(|b| b.referenced_annotations(names))
                    .unwrap_or_default()
            }

            fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
                self.target_type.referenced_types(names);
            }
        }
    };
}
// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod by_reference;
pub use by_reference::{ByReferenceMember, ByReferenceMemberDef};

mod by_value;
pub use by_value::{ByValueMember, ByValueMemberDef};

mod identity;
pub use identity::{IdentityMember, IdentityMemberDef};
