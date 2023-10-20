use std::fmt::Display;
use std::str::FromStr;
use crate::error::Error;
use crate::model::constraints::Term;
use crate::model::identifiers::Identifier;
use crate::model::Span;
use crate::syntax::{
    KW_QUANTIFIER_EXISTS, KW_QUANTIFIER_EXISTS_SYMBOL, KW_QUANTIFIER_FORALL,
    KW_QUANTIFIER_FORALL_SYMBOL, KW_RELATION_GREATER_THAN, KW_RELATION_GREATER_THAN_OR_EQUAL,
    KW_RELATION_GREATER_THAN_OR_EQUAL_SYMBOL, KW_RELATION_LESS_THAN,
    KW_RELATION_LESS_THAN_OR_EQUAL, KW_RELATION_LESS_THAN_OR_EQUAL_SYMBOL, KW_RELATION_NOT_EQUAL,
    KW_RELATION_NOT_EQUAL_SYMBOL,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱  Sentences
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the grammar rule `constraint_sentence`.
///
/// A `ConstraintSentence` is either a [`SimpleSentence`], a [`BooleanSentence`], or
/// a [`QuantitySentence`].
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
    span: Option<Span>,
    predicate: Term,
    arguments: Vec<Term>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Equation {
    span: Option<Span>,
    left_operand: Term,
    right_operand: Term,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Inequation {
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
    span: Option<Span>,
    binding: QuantifiedVariableBinding,
    body: Box<ConstraintSentence>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct QuantifiedVariableBinding {
    span: Option<Span>,
    quantifier: Quantifier,
    // None = `self`
    binding: Option<QuantifiedVariable>,
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
/// A `QuantifiedVariable` is a *name* and *source* pair.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct QuantifiedVariable {
    span: Option<Span>,
    name: Identifier,
    source: Term,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Formal Constraints ❱  Sentences
// ------------------------------------------------------------------------------------------------

impl From<SimpleSentence> for ConstraintSentence {
    fn from(v: SimpleSentence) -> Self {
        Self::Simple(v)
    }
}

impl From<BooleanSentence> for ConstraintSentence {
    fn from(v: BooleanSentence) -> Self {
        Self::Boolean(v)
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

    is_as_variant!(Simple (SimpleSentence) => is_simple, as_simple);

    is_as_variant!(Boolean (BooleanSentence) => is_boolean, as_boolean);

    is_as_variant!(Quantified (QuantifiedSentence) => is_quantified, as_quantified);
}

// ------------------------------------------------------------------------------------------------

impl From<AtomicSentence> for SimpleSentence {
    fn from(v: AtomicSentence) -> Self {
        Self::Atomic(v)
    }
}

impl From<Equation> for SimpleSentence {
    fn from(v: Equation) -> Self {
        Self::Equation(v)
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

    is_as_variant!(Atomic (AtomicSentence) => is_atomic, as_atomic);

    is_as_variant!(Equation (Equation) => is_equation, as_equation);

    is_as_variant!(Inequation (Inequation) => is_inequation, as_inequation);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(AtomicSentence);

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

    get_and_set!(pub predicate, set_predicate => Term);

    get_and_set_vec!(
        pub
        has has_arguments,
        arguments_len,
        arguments,
        arguments_mut,
        add_to_arguments,
        extend_arguments
            => arguments, Term
    );
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(Equation);

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

    get_and_set!(pub left_operand, set_left_operand => Term);

    get_and_set!(pub right_operand, set_right_operand => Term);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(Inequation);

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

    get_and_set!(pub left_operand, set_left_operand => Term);

    get_and_set!(pub relation, set_relation => InequalityRelation);

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

    get_and_set!(pub right_operand, set_right_operand => Term);
}

// ------------------------------------------------------------------------------------------------

impl Display for InequalityRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (Self::NotEqual, true) => KW_RELATION_NOT_EQUAL_SYMBOL,
                (Self::NotEqual, false) => KW_RELATION_NOT_EQUAL,
                (Self::LessThan, _) => KW_RELATION_LESS_THAN,
                (Self::LessThanOrEqual, true) => KW_RELATION_LESS_THAN_OR_EQUAL_SYMBOL,
                (Self::LessThanOrEqual, false) => KW_RELATION_LESS_THAN_OR_EQUAL,
                (Self::GreaterThan, _) => KW_RELATION_GREATER_THAN,
                (Self::GreaterThanOrEqual, true) => KW_RELATION_GREATER_THAN_OR_EQUAL,
                (Self::GreaterThanOrEqual, false) => KW_RELATION_GREATER_THAN_OR_EQUAL_SYMBOL,
            }
        )
    }
}

impl FromStr for InequalityRelation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            KW_RELATION_NOT_EQUAL | KW_RELATION_NOT_EQUAL_SYMBOL => Ok(Self::NotEqual),
            KW_RELATION_LESS_THAN => Ok(Self::LessThan),
            KW_RELATION_LESS_THAN_OR_EQUAL | KW_RELATION_LESS_THAN_OR_EQUAL_SYMBOL => {
                Ok(Self::LessThanOrEqual)
            }
            KW_RELATION_GREATER_THAN => Ok(Self::GreaterThan),
            KW_RELATION_GREATER_THAN_OR_EQUAL | KW_RELATION_GREATER_THAN_OR_EQUAL_SYMBOL => {
                Ok(Self::GreaterThanOrEqual)
            }
            // TODO: a real error.
            _ => panic!(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<UnaryBooleanSentence> for BooleanSentence {
    fn from(v: UnaryBooleanSentence) -> Self {
        Self::Unary(v)
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

    is_as_variant!(Unary (UnaryBooleanSentence) => is_unary, as_unary);

    is_as_variant!(Binary (BinaryBooleanSentence) => is_binary, as_binary);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(UnaryBooleanSentence);

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

    #[inline(always)]
    pub fn operator(&self) -> ConnectiveOperator {
        ConnectiveOperator::Negation
    }

    get_and_set!(pub operand, set_operand => boxed ConstraintSentence);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(BinaryBooleanSentence);

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

    get_and_set!(pub left_operand, set_left_operand => boxed ConstraintSentence);

    get_and_set!(pub operator, set_operator => ConnectiveOperator);

    get_and_set!(pub right_operand, set_right_operand => boxed ConstraintSentence);
}

// ------------------------------------------------------------------------------------------------

impl_has_body_for!(QuantifiedSentence, boxed ConstraintSentence);

impl_has_source_span_for!(QuantifiedSentence);

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

    get_and_set!(pub binding, set_binding => QuantifiedVariableBinding);
}

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
                (Self::Existential, true) => KW_QUANTIFIER_EXISTS_SYMBOL,
                (Self::Existential, false) => KW_QUANTIFIER_EXISTS,
                (Self::Universal, true) => KW_QUANTIFIER_FORALL,
                (Self::Universal, false) => KW_QUANTIFIER_FORALL_SYMBOL,
            }
        )
    }
}

impl FromStr for Quantifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            KW_QUANTIFIER_EXISTS => Ok(Self::Existential),
            KW_QUANTIFIER_EXISTS_SYMBOL => Ok(Self::Existential),
            KW_QUANTIFIER_FORALL => Ok(Self::Universal),
            KW_QUANTIFIER_FORALL_SYMBOL => Ok(Self::Universal),
            // TODO: need an error here.
            _ => panic!("Invalid Quantifier value {:?}", s),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(QuantifiedVariableBinding);

impl QuantifiedVariableBinding {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(quantifier: Quantifier, binding: QuantifiedVariable) -> Self
    {
        Self {
            span: Default::default(),
            quantifier,
            binding: Some(binding),
        }
    }

    pub fn new_self(quantifier: Quantifier) -> Self
    {
        Self {
            span: Default::default(),
            quantifier,
            binding: None,
        }
    }

    pub fn new_existential(binding: QuantifiedVariable) -> Self
    {
        Self::new(Quantifier::Existential, binding)
    }

    pub fn new_universal(binding: QuantifiedVariable) -> Self
    {
        Self::new(Quantifier::Universal, binding)
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub quantifier, set_quantifier => Quantifier);

    #[inline(always)]
    pub fn is_existential(&self) -> bool {
        self.quantifier == Quantifier::Existential
    }

    #[inline(always)]
    pub fn is_universal(&self) -> bool {
        self.quantifier == Quantifier::Universal
    }

    pub fn binding(&self) -> Option<&QuantifiedVariable> {
        self.binding.as_ref()
    }

    pub fn is_bound_to_variable(&self) -> bool {
        self.binding.is_some()
    }

    pub fn set_binding_to_variable(&mut self, binding: QuantifiedVariable) {
        self.binding = Some(binding);
    }

    pub fn is_bound_to_self(&self) -> bool {
        self.binding.is_none()
    }

    pub fn set_binding_to_self(&mut self) {
        self.binding = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(QuantifiedVariable);

impl_has_name_for!(QuantifiedVariable);

impl QuantifiedVariable {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<T>(name: Identifier, source: T) -> Self
    where
        T: Into<Term>
    {
        Self {
            span: Default::default(),
            name,
            source: source.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub source, set_source => into Term);
}
