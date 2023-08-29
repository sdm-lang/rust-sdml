use crate::model::constraints::ConstraintSentence;
use crate::model::identifiers::{Identifier, IdentifierReference};
use crate::model::members::{MappingType, Ordering, Uniqueness, CardinalityRange};
use crate::model::Span;
use crate::error::Error;
use crate::model::modules::Module;
use crate::model::check::Validate;
use std::fmt::Display;
use crate::syntax::KW_WILDCARD;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱ Environments ❱ Functions
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionDef {
    span: Option<Span>,
    signature: FunctionSignature,
    body: ConstraintSentence,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionSignature {
    span: Option<Span>,
    parameters: Vec<FunctionParameter>,
    target_type: FunctionType,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionParameter {
    span: Option<Span>,
    name: Identifier,
    target_type: FunctionType,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionType {
    span: Option<Span>,
    target_cardinality: FunctionCardinality,
    target_type: FunctionTypeReference,
}

/// Corresponds to the grammar rule `cardinality`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionCardinality {
    span: Option<Span>,
    ordering: Option<Ordering>,
    uniqueness: Option<Uniqueness>,
    range: Option<CardinalityRange>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum FunctionTypeReference {
    Wildcard,
    Reference(IdentifierReference),
    // builtin_simple_type is converted into a reference
    MappingType(MappingType),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Formal Constraints ❱ Environments ❱ Functions
// ------------------------------------------------------------------------------------------------

impl_has_body_for!(FunctionDef, ConstraintSentence);

impl_has_source_span_for!(FunctionDef);

impl FunctionDef {
    pub fn new(signature: FunctionSignature, body: ConstraintSentence) -> Self {
        Self {
            span: Default::default(),
            signature,
            body,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn signature(&self) -> &FunctionSignature {
        &self.signature
    }
    pub fn set_signature(&mut self, signature: FunctionSignature) {
        self.signature = signature;
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(FunctionSignature);

impl FunctionSignature {
    pub fn new(parameters: Vec<FunctionParameter>, target_type: FunctionType) -> Self {
        Self {
            span: Default::default(),
            parameters,
            target_type,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }
    pub fn parameters_len(&self) -> usize {
        self.parameters.len()
    }
    pub fn parameters(&self) -> impl Iterator<Item = &FunctionParameter> {
        self.parameters.iter()
    }
    pub fn parameters_mut(&mut self) -> impl Iterator<Item = &mut FunctionParameter> {
        self.parameters.iter_mut()
    }
    pub fn add_to_parameters(&mut self, value: FunctionParameter) {
        self.parameters.push(value)
    }
    pub fn extend_parameters<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = FunctionParameter>,
    {
        self.parameters.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn target_type(&self) -> &FunctionType {
        &self.target_type
    }
    pub fn set_target_type(&mut self, target_type: FunctionType) {
        self.target_type = target_type;
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_name_for!(FunctionParameter);

impl_has_source_span_for!(FunctionParameter);

impl FunctionParameter {
    pub fn new(name: Identifier, target_type: FunctionType) -> Self {
        Self {
            span: Default::default(),
            name,
            target_type,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn target_type(&self) -> &FunctionType {
        &self.target_type
    }
    pub fn set_target_type(&mut self, target_type: FunctionType) {
        self.target_type = target_type;
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(FunctionType);

impl FunctionType {
    pub fn new(
        target_cardinality: FunctionCardinality,
        target_type: FunctionTypeReference,
    ) -> Self {
        Self {
            span: Default::default(),
            target_cardinality,
            target_type,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_wildcard_cardinality(self) -> Self {
        Self {
            target_cardinality: FunctionCardinality::new_wildcard(),
            ..self
        }
    }

    pub fn with_target_cardinality(self, target_cardinality: FunctionCardinality) -> Self {
        Self {
            target_cardinality,
            ..self
        }
    }

    pub fn target_cardinality(&self) -> &FunctionCardinality {
        &self.target_cardinality
    }

    pub fn set_target_cardinality(&mut self, target_cardinality: FunctionCardinality) {
        self.target_cardinality = target_cardinality;
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_target_type(self, target_type: FunctionTypeReference) -> Self {
        Self {
            target_type,
            ..self
        }
    }

    pub fn target_type(&self) -> &FunctionTypeReference {
        &self.target_type
    }

    pub fn set_target_type(&mut self, target_type: FunctionTypeReference) {
        self.target_type = target_type;
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for FunctionCardinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}{}{}}}",
            self.ordering.map(|c| format!("{} ", c)).unwrap_or_default(),
            self.uniqueness
                .map(|c| format!("{} ", c))
                .unwrap_or_default(),
            if let Some(range) = &self.range {
                range.to_string()
            } else {
                KW_WILDCARD.to_string()
            }
        )
    }
}

impl_has_source_span_for!(FunctionCardinality);

impl Validate for FunctionCardinality {
    fn is_complete(&self, top: &Module) -> Result<bool, Error> {
        if let Some(range) = &self.range {
            range.is_complete(top)
        } else {
            Ok(true)
        }
    }

    fn is_valid(&self, check_constraints: bool, top: &Module) -> Result<bool, Error> {
        if let Some(range) = &self.range {
            range.is_valid(check_constraints, top)
        } else {
            Ok(true)
        }
    }
}

impl FunctionCardinality {
    pub const fn new(ordering: Option<Ordering>, uniqueness: Option<Uniqueness>, range: Option<CardinalityRange>) -> Self {
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
            range: Some(CardinalityRange::new_range(min, max)),
        }
    }

    pub const fn new_unbounded(min: u32) -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            range: Some(CardinalityRange::new_unbounded(min)),
        }
    }

    pub const fn new_single(min_and_max: u32) -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            range: Some(CardinalityRange::new_single(min_and_max)),
        }
    }

    pub const fn new_wildcard() -> Self {
        Self {
            span: None,
            ordering: None,
            uniqueness: None,
            range: None,
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

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    pub fn is_wildcard(&self) -> bool {
        self.range.is_none()
    }

    pub fn range(&self) -> Option<&CardinalityRange> {
        self.range.as_ref()
    }

    pub fn set_range(&mut self, range: CardinalityRange) {
        self.range = Some(range);
    }
}

// ------------------------------------------------------------------------------------------------

impl From<IdentifierReference> for FunctionTypeReference {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl From<MappingType> for FunctionTypeReference {
    fn from(value: MappingType) -> Self {
        Self::MappingType(value)
    }
}

impl FunctionTypeReference {
    pub fn is_type_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub fn as_type_reference(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Reference(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_mapping_type(&self) -> bool {
        matches!(self, Self::MappingType(_))
    }

    pub fn as_mapping_type(&self) -> Option<&MappingType> {
        match self {
            Self::MappingType(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_wildcard(&self) -> bool {
        matches!(self, Self::Wildcard)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
