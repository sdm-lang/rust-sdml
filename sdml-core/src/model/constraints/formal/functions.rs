use crate::{
    load::ModuleLoader,
    model::{
        check::Validate,
        constraints::{
            BooleanSentence, ConstraintSentence, FunctionComposition, FunctionalTerm,
            PredicateValue, QuantifiedSentence, SequenceBuilder, SimpleSentence, Term,
        },
        identifiers::{Identifier, IdentifierReference},
        members::{CardinalityRange, MappingType, Ordering, Uniqueness},
        modules::Module,
        HasBody, HasName, HasSourceSpan, Span,
    },
    store::ModuleStore,
    syntax::KW_WILDCARD,
};
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Constraints ❱ Functions
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    signature: FunctionSignature,
    body: FunctionBody,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionSignature {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    parameters: Vec<FunctionParameter>,
    target_type: FunctionType,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionParameter {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    target_type: FunctionType,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionType {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    target_cardinality: FunctionCardinality,
    target_type: FunctionTypeReference,
}

/// Corresponds to the grammar rule `cardinality`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionCardinality {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    ordering: Option<Ordering>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    uniqueness: Option<Uniqueness>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum FunctionBody {
    Sentence(ConstraintSentence),
    Term(Term),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ FunctionDef
// ------------------------------------------------------------------------------------------------

impl HasBody for FunctionDef {
    type Body = FunctionBody;

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn body_mut(&mut self) -> &mut Self::Body {
        &mut self.body
    }

    fn set_body(&mut self, body: Self::Body) {
        self.body = body;
    }
}

impl HasSourceSpan for FunctionDef {
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

impl FunctionDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<T>(signature: FunctionSignature, body: T) -> Self
    where
        T: Into<FunctionBody>,
    {
        Self {
            span: None,
            signature,
            body: body.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn name(&self) -> &Identifier {
        self.signature.name()
    }

    pub const fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    pub fn set_signature(&mut self, signature: FunctionSignature) {
        self.signature = signature;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ FunctionSignature
// ------------------------------------------------------------------------------------------------

impl HasName for FunctionSignature {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasSourceSpan for FunctionSignature {
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

impl FunctionSignature {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(
        name: Identifier,
        parameters: Vec<FunctionParameter>,
        target_type: FunctionType,
    ) -> Self {
        Self::named(name, target_type).with_parameters(parameters)
    }

    pub fn named(name: Identifier, target_type: FunctionType) -> Self {
        Self {
            span: Default::default(),
            name,
            parameters: Default::default(),
            target_type,
        }
    }

    pub fn with_parameters<I>(self, parameters: I) -> Self
    where
        I: IntoIterator<Item = FunctionParameter>,
    {
        let mut self_mut = self;
        self_mut.extend_parameters(parameters);
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
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

    pub fn add_to_parameters<I>(&mut self, value: I)
    where
        I: Into<FunctionParameter>,
    {
        self.parameters.push(value.into())
    }

    pub fn extend_parameters<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = FunctionParameter>,
    {
        self.parameters.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub const fn target_type(&self) -> &FunctionType {
        &self.target_type
    }

    pub fn set_target_type(&mut self, target_type: FunctionType) {
        self.target_type = target_type;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ FunctionParameter
// ------------------------------------------------------------------------------------------------

impl HasName for FunctionParameter {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasSourceSpan for FunctionParameter {
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

impl FunctionParameter {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: Identifier, target_type: FunctionType) -> Self {
        Self {
            span: None,
            name,
            target_type,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn target_type(&self) -> &FunctionType {
        &self.target_type
    }

    pub fn set_target_type(&mut self, target_type: FunctionType) {
        self.target_type = target_type;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ FunctionType
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for FunctionType {
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

impl FunctionType {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

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

    pub fn with_wildcard_cardinality(self) -> Self {
        let mut self_mut = self;
        self_mut.target_cardinality = FunctionCardinality::new_wildcard();
        self_mut
    }

    pub fn with_target_cardinality(self, target_cardinality: FunctionCardinality) -> Self {
        let mut self_mut = self;
        self_mut.target_cardinality = target_cardinality;
        self_mut
    }

    pub fn with_target_type(self, target_type: FunctionTypeReference) -> Self {
        let mut self_mut = self;
        self_mut.target_type = target_type;
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn target_cardinality(&self) -> &FunctionCardinality {
        &self.target_cardinality
    }

    pub fn set_target_cardinality(&mut self, target_cardinality: FunctionCardinality) {
        self.target_cardinality = target_cardinality;
    }

    // --------------------------------------------------------------------------------------------

    pub const fn target_type(&self) -> &FunctionTypeReference {
        &self.target_type
    }

    pub fn set_target_type(&mut self, target_type: FunctionTypeReference) {
        self.target_type = target_type;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ FunctionCardinality
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

impl HasSourceSpan for FunctionCardinality {
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

impl Validate for FunctionCardinality {
    fn validate(
        &self,
        _top: &Module,
        _cache: &impl ModuleStore,
        _loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        todo!()
    }
}

impl FunctionCardinality {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(
        ordering: Option<Ordering>,
        uniqueness: Option<Uniqueness>,
        range: Option<CardinalityRange>,
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

    pub const fn with_uniqueness(self, uniqueness: Uniqueness) -> Self {
        let mut self_mut = self;
        self_mut.uniqueness = Some(uniqueness);
        self_mut
    }

    pub const fn with_ordering(self, ordering: Ordering) -> Self {
        let mut self_mut = self;
        self_mut.ordering = Some(ordering);
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn has_ordering(&self) -> bool {
        self.ordering.is_some()
    }

    pub const fn ordering(&self) -> Option<Ordering> {
        self.ordering
    }

    pub fn set_ordering(&mut self, ordering: Ordering) {
        self.ordering = Some(ordering);
    }

    pub fn unset_ordering(&mut self) {
        self.ordering = None;
    }

    #[inline(always)]
    pub fn is_ordered(&self) -> Option<bool> {
        self.ordering().map(|o| o == Ordering::Ordered)
    }

    // --------------------------------------------------------------------------------------------

    pub const fn has_uniqueness(&self) -> bool {
        self.uniqueness.is_some()
    }

    pub const fn uniqueness(&self) -> Option<Uniqueness> {
        self.uniqueness
    }

    pub fn set_uniqueness(&mut self, uniqueness: Uniqueness) {
        self.uniqueness = Some(uniqueness);
    }

    pub fn unset_uniqueness(&mut self) {
        self.uniqueness = None;
    }

    #[inline(always)]
    pub fn is_unique(&self) -> Option<bool> {
        self.uniqueness().map(|u| u == Uniqueness::Unique)
    }

    // --------------------------------------------------------------------------------------------

    pub const fn has_range(&self) -> bool {
        self.range.is_some()
    }

    pub const fn range(&self) -> Option<&CardinalityRange> {
        self.range.as_ref()
    }

    pub fn set_range(&mut self, range: CardinalityRange) {
        self.range = Some(range);
    }

    pub fn unset_range(&mut self) {
        self.range = None;
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn is_default(&self) -> bool {
        self.ordering.is_none() && self.uniqueness.is_none() && self.range.is_none()
    }

    pub fn is_wildcard(&self) -> bool {
        self.range.is_none()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ FunctionTypeReference
// ------------------------------------------------------------------------------------------------

impl From<&IdentifierReference> for FunctionTypeReference {
    fn from(value: &IdentifierReference) -> Self {
        Self::Reference(value.clone())
    }
}

impl From<IdentifierReference> for FunctionTypeReference {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl From<&MappingType> for FunctionTypeReference {
    fn from(value: &MappingType) -> Self {
        Self::MappingType(value.clone())
    }
}

impl From<MappingType> for FunctionTypeReference {
    fn from(value: MappingType) -> Self {
        Self::MappingType(value)
    }
}

impl FunctionTypeReference {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub const fn as_reference(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Reference(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub const fn is_mapping_type(&self) -> bool {
        matches!(self, Self::MappingType(_))
    }

    pub const fn as_mapping_type(&self) -> Option<&MappingType> {
        match self {
            Self::MappingType(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub const fn is_wildcard(&self) -> bool {
        matches!(self, Self::Wildcard)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ FunctionBody
// ------------------------------------------------------------------------------------------------

impl From<ConstraintSentence> for FunctionBody {
    fn from(value: ConstraintSentence) -> FunctionBody {
        FunctionBody::Sentence(value)
    }
}

impl From<SimpleSentence> for FunctionBody {
    fn from(value: SimpleSentence) -> FunctionBody {
        FunctionBody::Sentence(value.into())
    }
}

impl From<BooleanSentence> for FunctionBody {
    fn from(value: BooleanSentence) -> FunctionBody {
        FunctionBody::Sentence(value.into())
    }
}

impl From<QuantifiedSentence> for FunctionBody {
    fn from(value: QuantifiedSentence) -> FunctionBody {
        FunctionBody::Sentence(value.into())
    }
}

impl From<Term> for FunctionBody {
    fn from(value: Term) -> FunctionBody {
        FunctionBody::Term(value)
    }
}

impl From<SequenceBuilder> for FunctionBody {
    fn from(value: SequenceBuilder) -> FunctionBody {
        FunctionBody::Term(Box::new(value).into())
    }
}

impl From<FunctionalTerm> for FunctionBody {
    fn from(value: FunctionalTerm) -> FunctionBody {
        FunctionBody::Term(Box::new(value).into())
    }
}

impl From<FunctionComposition> for FunctionBody {
    fn from(value: FunctionComposition) -> FunctionBody {
        FunctionBody::Term(value.into())
    }
}

impl From<IdentifierReference> for FunctionBody {
    fn from(value: IdentifierReference) -> FunctionBody {
        FunctionBody::Term(value.into())
    }
}

impl From<PredicateValue> for FunctionBody {
    fn from(value: PredicateValue) -> FunctionBody {
        FunctionBody::Term(value.into())
    }
}

impl FunctionBody {
    pub fn is_sentence(&self) -> bool {
        matches!(self, Self::Sentence(_))
    }

    pub fn as_sentence(&self) -> Option<&ConstraintSentence> {
        match &self {
            Self::Sentence(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_term(&self) -> bool {
        matches!(self, Self::Term(_))
    }

    pub fn as_term(&self) -> Option<&Term> {
        match &self {
            Self::Term(value) => Some(value),
            _ => None,
        }
    }
}
