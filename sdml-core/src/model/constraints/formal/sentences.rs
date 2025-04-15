use crate::error::Error;
use crate::model::constraints::Term;
use crate::model::identifiers::{Identifier, IdentifierReference};
use crate::model::{HasBody, HasName, HasSourceSpan, Span};
use crate::syntax::{
    OP_LOGICAL_BICONDITIONAL, OP_LOGICAL_BICONDITIONAL_SYMBOL, OP_LOGICAL_CONJUNCTION,
    OP_LOGICAL_CONJUNCTION_SYMBOL, OP_LOGICAL_DISJUNCTION, OP_LOGICAL_DISJUNCTION_SYMBOL,
    OP_LOGICAL_EXCLUSIVE_DISJUNCTION, OP_LOGICAL_EXCLUSIVE_DISJUNCTION_SYMBOL,
    OP_LOGICAL_IMPLICATION, OP_LOGICAL_IMPLICATION_SYMBOL, OP_LOGICAL_NEGATION,
    OP_LOGICAL_NEGATION_SYMBOL, OP_LOGICAL_QUANTIFIER_EXISTS, OP_LOGICAL_QUANTIFIER_EXISTS_SYMBOL,
    OP_LOGICAL_QUANTIFIER_FORALL, OP_LOGICAL_QUANTIFIER_FORALL_SYMBOL, OP_RELATION_GREATER_THAN,
    OP_RELATION_GREATER_THAN_OR_EQUAL, OP_RELATION_GREATER_THAN_OR_EQUAL_SYMBOL,
    OP_RELATION_LESS_THAN, OP_RELATION_LESS_THAN_OR_EQUAL, OP_RELATION_LESS_THAN_OR_EQUAL_SYMBOL,
    OP_RELATION_NOT_EQUAL, OP_RELATION_NOT_EQUAL_SYMBOL,
};
use std::fmt::Display;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Constraints ❱  Sentences
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the grammar rule `constraint_sentence`.
///
/// A `ConstraintSentence` is either a [`SimpleSentence`], a [`BooleanSentence`], or
/// a [`QuantifiedSentence`].
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ConstraintSentence {
    Simple(SimpleSentence),
    Boolean(BooleanSentence),
    Quantified(QuantifiedSentence),
}

///
/// Corresponds to the grammar rule `simple_sentence`.
///
/// A `SimpleSentence` is either an [`AtomicSentence`] or an [`Equation`].
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum SimpleSentence {
    /// Corresponds to the choice `atomic_sentence`.
    Atomic(AtomicSentence),
    /// Corresponds to the choice `equation`.
    Equation(Equation),
    /// Corresponds to the choice `inequation`.
    Inequation(Inequation),
}

///
/// Corresponds to the grammar rule `atomic_sentence`.
///
/// An `AtomicSentence` has a *predicate* term and an ordered list of terms corresponding
/// to the predicate *arguments*.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AtomicSentence {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    predicate: Term,
    arguments: Vec<Term>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Equation {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    left_operand: Term,
    right_operand: Term,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Inequation {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    left_operand: Term,
    relation: InequalityRelation,
    right_operand: Term,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InequalityRelation {
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

///
/// Corresponds to the grammar rule `boolean_sentence`.
///
/// Boolean sentences are those that are constructed with the boolean operations negation (not),
/// conjunction (and), disjunction (or), exclusive disjunction (xor), implication, or
/// biconditional.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum BooleanSentence {
    Unary(UnaryBooleanSentence),
    Binary(BinaryBooleanSentence),
}

///
/// Holds the *left* and *right* operands in the rules `conjunction`, `disjunction`,
/// `exclusive_disjunction`, `implication`, and `biconditional`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UnaryBooleanSentence {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    operand: Box<ConstraintSentence>,
}

///
/// Holds the *left* and *right* operands in the rules `conjunction`, `disjunction`,
/// `exclusive_disjunction`, `implication`, and `biconditional`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BinaryBooleanSentence {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    left_operand: Box<ConstraintSentence>,
    operator: ConnectiveOperator,
    right_operand: Box<ConstraintSentence>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ConnectiveOperator {
    /// Corresponds to the grammar rule `negation`. Uses the prefix keyword **`not`**
    /// or the operator $\lnot$.
    Negation,
    /// Corresponds to the grammar rule `conjunction`. Uses the infix keyword **`and`**
    /// or the operator $\land$.
    Conjunction,
    /// Corresponds to the grammar rule `disjunction`. Uses the infix keyword **`or`**
    /// or the operator $\lor$.
    Disjunction,
    /// Corresponds to the grammar rule `exclusive_disjunction`. Uses the infix keyword **`xor`**
    /// or the operator $\veebar$. Note that this operation is not a part of ISO Common Logic but
    /// $a \veebar b$ can be rewritten as $\lnot (a \iff b)$
    ExclusiveDisjunction,
    /// Corresponds to the grammar rule `implication`. Uses the infix keyword **`implies`**
    /// or the operator $\implies$.
    Implication,
    /// Corresponds to the grammar rule `biconditional`. Uses the infix keyword **`iff`**
    /// or the operator $\iff$.
    Biconditional,
}

///
/// Corresponds to the grammar rule `quantified_sentence`.
///
/// Such a sentence may be either *universally* or *existentially* quantified.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct QuantifiedSentence {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    binding: QuantifiedVariableBinding,
    body: Box<ConstraintSentence>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct QuantifiedVariableBinding {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    quantifier: Quantifier,
    binding: QuantifiedVariable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Quantifier {
    /// Corresponds to the grammar rule `universal`. Introduced with the keyword **`forall`**
    /// or the operator $\forall$.
    Universal,
    /// Corresponds to the grammar rule `existential`. Introduced with the keyword **`exists`**
    /// or the operator $\exists$.
    Existential,
}

///
/// Corresponds to the grammar rule `quantified_variable`.
///
/// A `QuantifiedVariable` is a *variable* and *source* pair.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct QuantifiedVariable {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    variable: Variable,
    source: Term,
}

///
/// Corresponds to the grammar rule `variable`.
///
/// A `QuantifiedVariable` is a *name* and *source* pair.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Variable {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    range: Option<IdentifierReference>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ ConstraintSentence
// ------------------------------------------------------------------------------------------------

impl From<&SimpleSentence> for ConstraintSentence {
    fn from(v: &SimpleSentence) -> Self {
        Self::Simple(v.clone())
    }
}

impl From<SimpleSentence> for ConstraintSentence {
    fn from(v: SimpleSentence) -> Self {
        Self::Simple(v)
    }
}

impl From<&BooleanSentence> for ConstraintSentence {
    fn from(v: &BooleanSentence) -> Self {
        Self::Boolean(v.clone())
    }
}

impl From<BooleanSentence> for ConstraintSentence {
    fn from(v: BooleanSentence) -> Self {
        Self::Boolean(v)
    }
}

impl From<&QuantifiedSentence> for ConstraintSentence {
    fn from(v: &QuantifiedSentence) -> Self {
        Self::Quantified(v.clone())
    }
}

impl From<QuantifiedSentence> for ConstraintSentence {
    fn from(v: QuantifiedSentence) -> Self {
        Self::Quantified(v)
    }
}

impl ConstraintSentence {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_simple(&self) -> bool {
        matches!(self, Self::Simple(_))
    }

    pub const fn as_simple(&self) -> Option<&SimpleSentence> {
        match self {
            Self::Simple(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub const fn as_boolean(&self) -> Option<&BooleanSentence> {
        match self {
            Self::Boolean(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_quantified(&self) -> bool {
        matches!(self, Self::Quantified(_))
    }

    pub const fn as_quantified(&self) -> Option<&QuantifiedSentence> {
        match self {
            Self::Quantified(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ SimpleSentence
// ------------------------------------------------------------------------------------------------

impl From<&AtomicSentence> for SimpleSentence {
    fn from(v: &AtomicSentence) -> Self {
        Self::Atomic(v.clone())
    }
}

impl From<AtomicSentence> for SimpleSentence {
    fn from(v: AtomicSentence) -> Self {
        Self::Atomic(v)
    }
}

impl From<&Equation> for SimpleSentence {
    fn from(v: &Equation) -> Self {
        Self::Equation(v.clone())
    }
}

impl From<Equation> for SimpleSentence {
    fn from(v: Equation) -> Self {
        Self::Equation(v)
    }
}

impl From<&Inequation> for SimpleSentence {
    fn from(v: &Inequation) -> Self {
        Self::Inequation(v.clone())
    }
}

impl From<Inequation> for SimpleSentence {
    fn from(v: Inequation) -> Self {
        Self::Inequation(v)
    }
}

impl SimpleSentence {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_atomic(&self) -> bool {
        matches!(self, Self::Atomic(_))
    }

    pub const fn as_atomic(&self) -> Option<&AtomicSentence> {
        match self {
            Self::Atomic(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_equation(&self) -> bool {
        matches!(self, Self::Equation(_))
    }

    pub const fn as_equation(&self) -> Option<&Equation> {
        match self {
            Self::Equation(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_inequation(&self) -> bool {
        matches!(self, Self::Inequation(_))
    }

    pub const fn as_inequation(&self) -> Option<&Inequation> {
        match self {
            Self::Inequation(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ AtomicSentence
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for AtomicSentence {
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

impl AtomicSentence {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<T>(predicate: T) -> Self
    where
        T: Into<Term>,
    {
        Self {
            span: Default::default(),
            predicate: predicate.into(),
            arguments: Default::default(),
        }
    }

    pub fn new_with_arguments<T, I>(predicate: T, arguments: I) -> Self
    where
        T: Into<Term>,
        I: IntoIterator<Item = Term>,
    {
        Self {
            span: Default::default(),
            predicate: predicate.into(),
            arguments: Vec::from_iter(arguments),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn predicate(&self) -> &Term {
        &self.predicate
    }

    pub fn set_predicate(&mut self, predicate: Term) {
        self.predicate = predicate;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_arguments(&self) -> bool {
        !self.arguments.is_empty()
    }

    pub fn arguments_len(&self) -> usize {
        self.arguments.len()
    }

    pub fn arguments(&self) -> impl Iterator<Item = &Term> {
        self.arguments.iter()
    }

    pub fn arguments_mut(&mut self) -> impl Iterator<Item = &mut Term> {
        self.arguments.iter_mut()
    }

    pub fn add_to_arguments<I>(&mut self, value: I)
    where
        I: Into<Term>,
    {
        self.arguments.push(value.into())
    }

    pub fn extend_arguments<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Term>,
    {
        self.arguments.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ Equation
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for Equation {
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

impl Equation {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<Term>,
        R: Into<Term>,
    {
        Self {
            span: Default::default(),
            left_operand: left_operand.into(),
            right_operand: right_operand.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn left_operand(&self) -> &Term {
        &self.left_operand
    }

    pub fn set_left_operand(&mut self, left_operand: Term) {
        self.left_operand = left_operand;
    }

    // --------------------------------------------------------------------------------------------

    pub const fn right_operand(&self) -> &Term {
        &self.right_operand
    }

    pub fn set_right_operand(&mut self, right_operand: Term) {
        self.right_operand = right_operand;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ Inequation
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for Inequation {
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

impl Inequation {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<L, R>(left_operand: L, relation: InequalityRelation, right_operand: R) -> Self
    where
        L: Into<Term>,
        R: Into<Term>,
    {
        Self {
            span: Default::default(),
            left_operand: left_operand.into(),
            relation,
            right_operand: right_operand.into(),
        }
    }

    #[inline(always)]
    pub fn not_equal<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<Term>,
        R: Into<Term>,
    {
        Self::new(left_operand, InequalityRelation::NotEqual, right_operand)
    }

    #[inline(always)]
    pub fn less_than<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<Term>,
        R: Into<Term>,
    {
        Self::new(left_operand, InequalityRelation::LessThan, right_operand)
    }

    #[inline(always)]
    pub fn less_than_or_greater<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<Term>,
        R: Into<Term>,
    {
        Self::new(
            left_operand,
            InequalityRelation::LessThanOrEqual,
            right_operand,
        )
    }

    #[inline(always)]
    pub fn greater_than<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<Term>,
        R: Into<Term>,
    {
        Self::new(left_operand, InequalityRelation::GreaterThan, right_operand)
    }

    #[inline(always)]
    pub fn greater_than_or_equal<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<Term>,
        R: Into<Term>,
    {
        Self::new(
            left_operand,
            InequalityRelation::GreaterThanOrEqual,
            right_operand,
        )
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn left_operand(&self) -> &Term {
        &self.left_operand
    }

    pub fn set_left_operand(&mut self, left_operand: Term) {
        self.left_operand = left_operand;
    }

    // --------------------------------------------------------------------------------------------

    pub const fn relation(&self) -> &InequalityRelation {
        &self.relation
    }

    pub fn set_relation(&mut self, relation: InequalityRelation) {
        self.relation = relation;
    }

    #[inline(always)]
    pub fn is_not_equal(&self) -> bool {
        self.relation == InequalityRelation::NotEqual
    }

    #[inline(always)]
    pub fn is_less_than(&self) -> bool {
        self.relation == InequalityRelation::LessThan
    }

    #[inline(always)]
    pub fn is_greater_than(&self) -> bool {
        self.relation == InequalityRelation::GreaterThan
    }

    #[inline(always)]
    pub fn is_less_than_or_equal(&self) -> bool {
        self.relation == InequalityRelation::LessThanOrEqual
    }

    #[inline(always)]
    pub fn is_greater_than_or_equal(&self) -> bool {
        self.relation == InequalityRelation::GreaterThanOrEqual
    }

    // --------------------------------------------------------------------------------------------

    pub const fn right_operand(&self) -> &Term {
        &self.right_operand
    }

    pub fn set_right_operand(&mut self, right_operand: Term) {
        self.right_operand = right_operand;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ InequalityRelation
// ------------------------------------------------------------------------------------------------

impl FromStr for InequalityRelation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            OP_RELATION_NOT_EQUAL | OP_RELATION_NOT_EQUAL_SYMBOL => Ok(Self::NotEqual),
            OP_RELATION_LESS_THAN => Ok(Self::LessThan),
            OP_RELATION_LESS_THAN_OR_EQUAL | OP_RELATION_LESS_THAN_OR_EQUAL_SYMBOL => {
                Ok(Self::LessThanOrEqual)
            }
            OP_RELATION_GREATER_THAN => Ok(Self::GreaterThan),
            OP_RELATION_GREATER_THAN_OR_EQUAL | OP_RELATION_GREATER_THAN_OR_EQUAL_SYMBOL => {
                Ok(Self::GreaterThanOrEqual)
            }
            // TODO: a real error.
            _ => panic!(),
        }
    }
}

impl Display for InequalityRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (Self::NotEqual, true) => OP_RELATION_NOT_EQUAL_SYMBOL,
                (Self::NotEqual, false) => OP_RELATION_NOT_EQUAL,
                (Self::LessThan, _) => OP_RELATION_LESS_THAN,
                (Self::LessThanOrEqual, true) => OP_RELATION_LESS_THAN_OR_EQUAL_SYMBOL,
                (Self::LessThanOrEqual, false) => OP_RELATION_LESS_THAN_OR_EQUAL,
                (Self::GreaterThan, _) => OP_RELATION_GREATER_THAN,
                (Self::GreaterThanOrEqual, true) => OP_RELATION_GREATER_THAN_OR_EQUAL,
                (Self::GreaterThanOrEqual, false) => OP_RELATION_GREATER_THAN_OR_EQUAL_SYMBOL,
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ BooleanSentence
// ------------------------------------------------------------------------------------------------

impl From<&UnaryBooleanSentence> for BooleanSentence {
    fn from(v: &UnaryBooleanSentence) -> Self {
        Self::Unary(v.clone())
    }
}

impl From<UnaryBooleanSentence> for BooleanSentence {
    fn from(v: UnaryBooleanSentence) -> Self {
        Self::Unary(v)
    }
}

impl From<&BinaryBooleanSentence> for BooleanSentence {
    fn from(v: &BinaryBooleanSentence) -> Self {
        Self::Binary(v.clone())
    }
}

impl From<BinaryBooleanSentence> for BooleanSentence {
    fn from(v: BinaryBooleanSentence) -> Self {
        Self::Binary(v)
    }
}

impl BooleanSentence {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_unary(&self) -> bool {
        matches!(self, Self::Unary(_))
    }

    pub const fn as_unary(&self) -> Option<&UnaryBooleanSentence> {
        match self {
            Self::Unary(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    pub const fn as_binary(&self) -> Option<&BinaryBooleanSentence> {
        match self {
            Self::Binary(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ UnaryBooleanSentence
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for UnaryBooleanSentence {
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

impl UnaryBooleanSentence {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<R>(operand: R) -> Self
    where
        R: Into<ConstraintSentence>,
    {
        Self {
            span: Default::default(),
            operand: Box::new(operand.into()),
        }
    }

    #[inline(always)]
    pub fn negate<R>(operand: R) -> Self
    where
        R: Into<ConstraintSentence>,
    {
        Self::new(operand)
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn operator(&self) -> ConnectiveOperator {
        ConnectiveOperator::Negation
    }

    // --------------------------------------------------------------------------------------------

    pub const fn operand(&self) -> &ConstraintSentence {
        &self.operand
    }

    pub fn set_operand(&mut self, operand: ConstraintSentence) {
        self.operand = Box::new(operand);
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ BinaryBooleanSentence
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for BinaryBooleanSentence {
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

impl BinaryBooleanSentence {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<L, R>(left_operand: L, operator: ConnectiveOperator, right_operand: R) -> Self
    where
        L: Into<ConstraintSentence>,
        R: Into<ConstraintSentence>,
    {
        assert!(operator != ConnectiveOperator::Negation);
        Self {
            span: Default::default(),
            left_operand: Box::new(left_operand.into()),
            operator,
            right_operand: Box::new(right_operand.into()),
        }
    }

    #[inline(always)]
    pub fn and<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<ConstraintSentence>,
        R: Into<ConstraintSentence>,
    {
        Self::new(left_operand, ConnectiveOperator::Conjunction, right_operand)
    }

    #[inline(always)]
    pub fn or<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<ConstraintSentence>,
        R: Into<ConstraintSentence>,
    {
        Self::new(left_operand, ConnectiveOperator::Disjunction, right_operand)
    }

    #[inline(always)]
    pub fn xor<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<ConstraintSentence>,
        R: Into<ConstraintSentence>,
    {
        Self::new(
            left_operand,
            ConnectiveOperator::ExclusiveDisjunction,
            right_operand,
        )
    }

    #[inline(always)]
    pub fn implies<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<ConstraintSentence>,
        R: Into<ConstraintSentence>,
    {
        Self::new(left_operand, ConnectiveOperator::Implication, right_operand)
    }

    #[inline(always)]
    pub fn iff<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<ConstraintSentence>,
        R: Into<ConstraintSentence>,
    {
        Self::new(
            left_operand,
            ConnectiveOperator::Biconditional,
            right_operand,
        )
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn left_operand(&self) -> &ConstraintSentence {
        &self.left_operand
    }

    pub fn set_left_operand(&mut self, left_operand: ConstraintSentence) {
        self.left_operand = Box::new(left_operand);
    }

    // --------------------------------------------------------------------------------------------

    pub const fn operator(&self) -> &ConnectiveOperator {
        &self.operator
    }

    pub fn set_operator(&mut self, operator: ConnectiveOperator) {
        self.operator = operator;
    }

    // --------------------------------------------------------------------------------------------

    pub const fn right_operand(&self) -> &ConstraintSentence {
        &self.right_operand
    }

    pub fn set_right_operand(&mut self, right_operand: ConstraintSentence) {
        self.right_operand = Box::new(right_operand);
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ ConnectiveOperator
// ------------------------------------------------------------------------------------------------

impl Display for ConnectiveOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (ConnectiveOperator::Negation, true) => OP_LOGICAL_NEGATION,
                (ConnectiveOperator::Negation, false) => OP_LOGICAL_NEGATION_SYMBOL,
                (ConnectiveOperator::Conjunction, true) => OP_LOGICAL_CONJUNCTION,
                (ConnectiveOperator::Conjunction, false) => OP_LOGICAL_CONJUNCTION_SYMBOL,
                (ConnectiveOperator::Disjunction, true) => OP_LOGICAL_DISJUNCTION,
                (ConnectiveOperator::Disjunction, false) => OP_LOGICAL_DISJUNCTION_SYMBOL,
                (ConnectiveOperator::ExclusiveDisjunction, true) =>
                    OP_LOGICAL_EXCLUSIVE_DISJUNCTION,
                (ConnectiveOperator::ExclusiveDisjunction, false) =>
                    OP_LOGICAL_EXCLUSIVE_DISJUNCTION_SYMBOL,
                (ConnectiveOperator::Implication, true) => OP_LOGICAL_IMPLICATION,
                (ConnectiveOperator::Implication, false) => OP_LOGICAL_IMPLICATION_SYMBOL,
                (ConnectiveOperator::Biconditional, true) => OP_LOGICAL_BICONDITIONAL,
                (ConnectiveOperator::Biconditional, false) => OP_LOGICAL_BICONDITIONAL_SYMBOL,
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ QuantifiedSentence
// ------------------------------------------------------------------------------------------------

impl HasBody for QuantifiedSentence {
    type Body = ConstraintSentence;

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn body_mut(&mut self) -> &mut Self::Body {
        &mut self.body
    }

    fn set_body(&mut self, body: Self::Body) {
        self.body = Box::new(body);
    }
}

impl HasSourceSpan for QuantifiedSentence {
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

impl QuantifiedSentence {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<S>(binding: QuantifiedVariableBinding, body: S) -> Self
    where
        S: Into<ConstraintSentence>,
    {
        Self {
            span: Default::default(),
            binding,
            body: Box::new(body.into()),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn binding(&self) -> &QuantifiedVariableBinding {
        &self.binding
    }

    pub fn set_binding(&mut self, binding: QuantifiedVariableBinding) {
        self.binding = binding;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ Quantifier
// ------------------------------------------------------------------------------------------------

impl Default for Quantifier {
    fn default() -> Self {
        Self::Universal
    }
}

impl Display for Quantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (Self::Existential, true) => OP_LOGICAL_QUANTIFIER_EXISTS_SYMBOL,
                (Self::Existential, false) => OP_LOGICAL_QUANTIFIER_EXISTS,
                (Self::Universal, true) => OP_LOGICAL_QUANTIFIER_FORALL,
                (Self::Universal, false) => OP_LOGICAL_QUANTIFIER_FORALL_SYMBOL,
            }
        )
    }
}

impl FromStr for Quantifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            OP_LOGICAL_QUANTIFIER_EXISTS => Ok(Self::Existential),
            OP_LOGICAL_QUANTIFIER_EXISTS_SYMBOL => Ok(Self::Existential),
            OP_LOGICAL_QUANTIFIER_FORALL => Ok(Self::Universal),
            OP_LOGICAL_QUANTIFIER_FORALL_SYMBOL => Ok(Self::Universal),
            // TODO: need an error here.
            _ => panic!("Invalid Quantifier value {:?}", s),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ QuantifiedVariableBinding
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for QuantifiedVariableBinding {
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

impl QuantifiedVariableBinding {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(quantifier: Quantifier, binding: QuantifiedVariable) -> Self {
        Self {
            span: Default::default(),
            quantifier,
            binding: binding,
        }
    }

    pub fn new_existential(binding: QuantifiedVariable) -> Self {
        Self::new(Quantifier::Existential, binding)
    }

    pub fn new_universal(binding: QuantifiedVariable) -> Self {
        Self::new(Quantifier::Universal, binding)
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn quantifier(&self) -> &Quantifier {
        &self.quantifier
    }

    pub fn set_quantifier(&mut self, quantifier: Quantifier) {
        self.quantifier = quantifier;
    }

    #[inline(always)]
    pub const fn is_existential(&self) -> bool {
        matches!(self.quantifier, Quantifier::Existential)
    }

    #[inline(always)]
    pub const fn is_universal(&self) -> bool {
        matches!(self.quantifier, Quantifier::Universal)
    }

    // --------------------------------------------------------------------------------------------

    pub const fn binding(&self) -> &QuantifiedVariable {
        &self.binding
    }

    pub fn set_binding(&mut self, binding: QuantifiedVariable) {
        self.binding = binding;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ QuantifiedVariable
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for QuantifiedVariable {
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

impl QuantifiedVariable {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<T>(variable: Variable, source: T) -> Self
    where
        T: Into<Term>,
    {
        Self {
            span: Default::default(),
            variable,
            source: source.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn variable(&self) -> &Variable {
        &self.variable
    }

    pub fn set_variable(&mut self, variable: Variable) {
        self.variable = variable;
    }

    // --------------------------------------------------------------------------------------------

    pub const fn source(&self) -> &Term {
        &self.source
    }

    pub fn set_source<T>(&mut self, source: T)
    where
        T: Into<Term>,
    {
        self.source = source.into();
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ Variable
// ------------------------------------------------------------------------------------------------

impl HasName for Variable {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasSourceSpan for Variable {
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

impl Variable {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            range: None,
        }
    }

    pub fn with_range<I>(self, range: I) -> Self
    where
        I: Into<IdentifierReference>,
    {
        let mut self_mut = self;
        self_mut.range = Some(range.into());
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn range(&self) -> Option<&IdentifierReference> {
        self.range.as_ref()
    }

    pub fn set_range<I>(&mut self, range: I)
    where
        I: Into<IdentifierReference>,
    {
        self.range = Some(range.into());
    }

    pub fn unset_range(&mut self) {
        self.range = None
    }
}
