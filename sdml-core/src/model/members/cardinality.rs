use crate::error::Error;
use crate::load::ModuleLoader;
use crate::model::check::Validate;
use crate::model::modules::Module;
use crate::model::Span;
use crate::store::ModuleStore;
use crate::syntax::{
    KW_ORDERING_ORDERED, KW_ORDERING_UNORDERED, KW_UNIQUENESS_NONUNIQUE, KW_UNIQUENESS_UNIQUE,
};
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

pub trait HasCardinality {
    fn target_cardinality(&self) -> &Cardinality;

    fn set_target_cardinality(&mut self, target_cardinality: Cardinality);
}

/// Corresponds to the grammar rule `cardinality`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Cardinality {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    ordering: Option<Ordering>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    uniqueness: Option<Uniqueness>,
    range: CardinalityRange,
}

pub const DEFAULT_CARDINALITY: Cardinality = Cardinality::one();

pub const TYPE_BAG_CARDINALITY: Cardinality = Cardinality::zero_or_more();
pub const TYPE_LIST_CARDINALITY: Cardinality =
    Cardinality::zero_or_more().with_ordering(Some(Ordering::Ordered));
pub const TYPE_SET_CARDINALITY: Cardinality =
    Cardinality::zero_or_more().with_uniqueness(Some(Uniqueness::Unique));
pub const TYPE_ORDERED_SET_CARDINALITY: Cardinality = Cardinality::zero_or_more()
    .with_ordering(Some(Ordering::Ordered))
    .with_uniqueness(Some(Uniqueness::Unique));
pub const TYPE_MAYBE_CARDINALITY: Cardinality = Cardinality::zero_or_one();

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CardinalityRange {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    min: u32,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    max: Option<u32>,
}

pub const DEFAULT_CARDINALITY_RANGE: CardinalityRange = CardinalityRange::one();

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
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.range.validate(top, cache, loader, check_constraints);
    }
}

impl Cardinality {
    // --------------------------------------------------------------------------------------------
    // Cardinality :: Constructors
    // --------------------------------------------------------------------------------------------

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
    // Cardinality :: Fields
    // --------------------------------------------------------------------------------------------

    pub const fn with_ordering(self, ordering: Option<Ordering>) -> Self {
        Self { ordering, ..self }
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
    pub const fn with_uniqueness(self, uniqueness: Option<Uniqueness>) -> Self {
        Self { uniqueness, ..self }
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
    // Cardinality :: Helpers
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
    fn validate(
        &self,
        _: &Module,
        _: &impl ModuleStore,
        _loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        if let Some(max) = self.max {
            if max < self.min {
                panic!();
            }
        }
    }
}

impl CardinalityRange {
    // --------------------------------------------------------------------------------------------
    // Cardinality :: Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new_range(min: u32, max: u32) -> Self {
        assert!(
            max > 0 && max > min,
            "Zero, or negative, cardinality range is not allowed."
        );
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
        assert!(
            min_and_max != 0,
            "Zero width cardinality range is not allowed."
        );
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
    // Cardinality :: Fields
    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub const fn min_occurs(&self) -> u32 {
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
    pub const fn max_occurs(&self) -> Option<u32> {
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
    // Cardinality :: Helpers
    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub const fn is_optional(&self) -> bool {
        self.min_occurs() == 0
    }

    #[inline(always)]
    pub const fn is_required(&self) -> bool {
        !self.is_optional()
    }

    #[inline(always)]
    pub fn is_range(&self) -> bool {
        self.max.map(|i| i != self.min).unwrap_or(true)
    }

    #[inline(always)]
    pub const fn is_unbounded(&self) -> bool {
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
