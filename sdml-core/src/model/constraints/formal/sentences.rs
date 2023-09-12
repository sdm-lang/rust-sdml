use std::fmt::Display;
use std::str::FromStr;

use crate::error::Error;
use crate::model::constraints::{FunctionComposition, SequenceBuilder, Term};
use crate::model::identifiers::{Identifier, IdentifierReference};
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
    variable_bindings: Vec<QuantifiedVariableBinding>,
    body: Box<ConstraintSentence>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct QuantifiedVariableBinding {
    span: Option<Span>,
    quantifier: Quantifier,
    bindings: Vec<QuantifiedBinding>, // assert!(!is_empty())
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
/// Corresponds to the grammar rule `quantifier_binding`.
///
/// A `QuantifierBinding` is either the keyword **`self`** or a *name* and *target* pair.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum QuantifiedBinding {
    ReservedSelf,
    Named(QuantifierBoundNames),
}

///
/// Corresponds to the inner part of the grammar rule `quantifier_binding`.
///
/// A `QuantifierBinding` is either the keyword **`self`** or a set of *name* and a *target*.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct QuantifierBoundNames {
    span: Option<Span>,
    names: Vec<Identifier>,
    source: IteratorSource,
}

///
/// Corresponds to the grammar rule `binding_target`.
///
/// A `BindingTarget` may be either a [`BindingTypeRef`], or a [`BindingSeqIterator`].
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum IteratorSource {
    Type(TypeIterator),
    Sequence(SequenceIterator),
}

///
/// Corresponds to the grammar rule `binding_seq_iterator`.
///
/// A named binding may target either a [`BindingTypeRef`], or a [`BindingSeqIterator`].
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum SequenceIterator {
    Call(FunctionComposition),
    Variable(Identifier),
    Builder(SequenceBuilder),
}

///
/// Corresponds to the grammar rule `binding_type_reference`.
///
/// A `BindingTypeRef` is either the keyword **`Self`** or an [`IdentifierReference`].
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TypeIterator {
    SelfType,
    Type(IdentifierReference),
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

    pub fn new<B, S>(bindings: B, body: S) -> Self
    where
        B: Into<Vec<QuantifiedVariableBinding>>,
        S: Into<ConstraintSentence>,
    {
        Self {
            span: Default::default(),
            variable_bindings: bindings.into(),
            body: Box::new(body.into()),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set_vec!(
        pub
        has has_variable_bindings,
        variable_bindings_len,
        variable_bindings,
        variable_bindings_mut,
        add_to_variable_bindings,
        extend_variable_bindings
            => variable_bindings, QuantifiedVariableBinding
    );
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

    pub fn new<B>(quantifier: Quantifier, bindings: B) -> Self
    where
        B: Into<Vec<QuantifiedBinding>>,
    {
        Self {
            span: Default::default(),
            quantifier,
            bindings: bindings.into(),
        }
    }

    pub fn new_existential<B>(bindings: B) -> Self
    where
        B: Into<Vec<QuantifiedBinding>>,
    {
        Self::new(Quantifier::Existential, bindings)
    }

    pub fn new_universal<B>(bindings: B) -> Self
    where
        B: Into<Vec<QuantifiedBinding>>,
    {
        Self::new(Quantifier::Universal, bindings)
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

    get_and_set_vec!(
        pub
        has has_bindings,
        bindings_len,
        bindings,
        bindings_mut,
        add_to_bindings,
        extend_bindings
            => bindings, QuantifiedBinding
    );
}

// ------------------------------------------------------------------------------------------------

impl From<QuantifierBoundNames> for QuantifiedBinding {
    fn from(v: QuantifierBoundNames) -> Self {
        Self::Named(v)
    }
}

impl QuantifiedBinding {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_variant!(ReservedSelf => is_self_instance);

    is_as_variant!(Named (QuantifierBoundNames) => is_named, as_named);

    // --------------------------------------------------------------------------------------------
    // FromStr helper
    // --------------------------------------------------------------------------------------------

    pub fn is_valid_str(s: &str) -> bool {
        s == KW_QUANTIFIER_EXISTS
            || s == KW_QUANTIFIER_EXISTS_SYMBOL
            || s == KW_QUANTIFIER_FORALL
            || s == KW_QUANTIFIER_FORALL_SYMBOL
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(QuantifierBoundNames);

impl QuantifierBoundNames {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<N, B>(names: N, source: B) -> Self
    where
        N: IntoIterator<Item = Identifier>,
        B: Into<IteratorSource>,
    {
        Self {
            span: Default::default(),
            names: Vec::from_iter(names),
            source: source.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub source, set_source => IteratorSource);

    get_and_set_vec!(
        pub
        has has_names,
        names_len,
        names,
        names_mut,
        add_to_names,
        extend_names
            => names, Identifier
    );
}

// ------------------------------------------------------------------------------------------------

impl From<TypeIterator> for IteratorSource {
    fn from(v: TypeIterator) -> Self {
        Self::Type(v)
    }
}

impl From<SequenceIterator> for IteratorSource {
    fn from(v: SequenceIterator) -> Self {
        Self::Sequence(v)
    }
}

impl IteratorSource {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Type (TypeIterator) => is_type_iterator, as_type_iterator);

    is_as_variant!(Sequence (SequenceIterator) => is_sequence_iterator, as_sequence_iterator);
}

// ------------------------------------------------------------------------------------------------

impl From<FunctionComposition> for SequenceIterator {
    fn from(v: FunctionComposition) -> Self {
        Self::Call(v)
    }
}

impl From<Identifier> for SequenceIterator {
    fn from(v: Identifier) -> Self {
        Self::Variable(v)
    }
}

impl From<SequenceBuilder> for SequenceIterator {
    fn from(v: SequenceBuilder) -> Self {
        Self::Builder(v)
    }
}

impl SequenceIterator {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Call (FunctionComposition) => is_call_path, as_call_path);

    is_as_variant!(Variable (Identifier) => is_variable, as_variable);

    is_as_variant!(Builder (SequenceBuilder) => is_sequence_builder, as_sequence_builder);
}

// ------------------------------------------------------------------------------------------------

impl From<IdentifierReference> for TypeIterator {
    fn from(v: IdentifierReference) -> Self {
        Self::Type(v)
    }
}

impl TypeIterator {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub fn is_self_type(&self) -> bool {
        matches!(self, Self::SelfType)
    }

    is_as_variant!(Type (IdentifierReference) => is_type_name, as_type_name);
}
