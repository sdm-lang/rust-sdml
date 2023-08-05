use std::{fmt::Display, str::FromStr};

use crate::{
    error::invalid_language_tag_error,
    model::{Identifier, IdentifierReference, SimpleValue},
};

use lazy_static::lazy_static;
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Span;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `constraint`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Constraint {
    span: Option<Span>,
    name: Option<Identifier>,
    body: ConstraintBody,
}

///
/// Corresponds to the field `body` in the grammar rule `constraint`.
///
/// # Semantics
///
/// The domain of discourse, $\mathbb{D}$, is the set of all definitions present in the current
/// module and the set of modules transitively imported by it.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ConstraintBody {
    /// Corresponds to the grammar rule `informal_constraint`.
    Informal(ControlledLanguageString),
    /// Corresponds to the grammar rule `formal_constraint`.
    Formal(ConstraintSentence),
}

///
/// Corresponds to the grammar rule `informal_constraint`.
///
/// This structure captures an informal, or semi-formal constraint as a natural language string
/// string.
///
/// 1. `"some cars have manual transmissions"` is an informal constraint in some unidentified
///    natural language.
/// 2. `"some cars have manual transmissions"@en` is an informal constraint in English.
/// 3. `"there is a car that has a a:manual transmission."@en-ACE` is a semi-formal constraint in
///    Attempto Controlled English (ACE).
///
/// We classify the last example as *semi-formal*, even though ACE is formally defined,
/// because SDML does not expect (although does not prohibit) the translation from this form into
/// the logical structure of a [`ConstraintSentence`].
///
/// In the last example above the prefix `a:` on manual identifies the term *manual* it as an
/// adjective applied to the word term *transmission*.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ControlledLanguageString {
    span: Option<Span>,
    /// Corresponds to the grammar rule `quoted_string`.
    value: String,
    language: Option<ControlledLanguageTag>,
}

///
/// Corresponds to the grammar rule `controlled_language_tag`.
///
/// 1. Required natural language identifier, either a 2 or 3 character
///    code from ISO-639.
/// 2. An optional identifier representing the controlled language scheme.
///
/// There is no registry for controlled language schemes, and SDML makes no requirement
/// for the support of any particular scheme. The following are commonly used schemes
/// and their identifiers:
///
/// - **CLCE**: Common Logic Controlled English (see [Sowa, 2004](http://www.jfsowa.com/clce/specs.htm)).
/// - **ACE** or **Attempto**: Attempto Controlled English (ACE) (see
///   [attempto.ifi.uzh.ch](http://attempto.ifi.uzh.ch/site/)).
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ControlledLanguageTag {
    span: Option<Span>,
    value: String,
}

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
    Atomic(AtomicSentence),
    /// Corresponds to the grammar rule `equation`.
    Equation(BinaryOperation),
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
    /// Corresponds to the grammar rule `negation`. Uses the prefix keyword **`not`**
    /// or the operator $\lnot$.
    Negation(Box<ConstraintSentence>),
    /// Corresponds to the grammar rule `conjunction`. Uses the infix keyword **`and`**
    /// or the operator $\land$.
    Conjunction(BinaryOperation),
    /// Corresponds to the grammar rule `disjunction`. Uses the infix keyword **`or`**
    /// or the operator $\lor$.
    Disjunction(BinaryOperation),
    /// Corresponds to the grammar rule `exclusive_disjunction`. Uses the infix keyword **`xor`**
    /// or the operator $\veebar$. Note that this operation is not a part of ISO Common Logic but
    /// $a \veebar b$ can be rewritten as $\lnot (a \iff b)$
    ExclusiveDisjunction(BinaryOperation),
    /// Corresponds to the grammar rule `implication`. Uses the infix keyword **`implies`**
    /// or the operator $\implies$.
    Implication(BinaryOperation),
    /// Corresponds to the grammar rule `biconditional`. Uses the infix keyword **`iff`**
    /// or the operator $\iff$.
    Biconditional(BinaryOperation),
}

///
/// Holds the *left* and *right* operands in the rules `conjunction`, `disjunction`,
/// `exclusive_disjunction`, `implication`, and `biconditional`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BinaryOperation {
    span: Option<Span>,
    left_operand: Box<ConstraintSentence>,
    right_operand: Box<ConstraintSentence>,
}

///
/// Corresponds to the grammar rule `quantified_sentence`.
///
/// Such a sentence may be either *universally* or *existentially* quantified.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum QuantifiedSentence {
    /// Corresponds to the grammar rule `universal`. Introduced with the keyword **`forall`**
    /// or the operator $\forall$.
    Universal(BoundSentence),
    /// Corresponds to the grammar rule `existential`. Introduced with the keyword **`exists`**
    /// or the operator $\exists$.
    Existential(BoundSentence),
}

///
/// Corresponds to the inner part of the grammar rule `quantified_sentence`,
/// and the rule `quantified_body`.
///
/// The *body* of a bound sentence is a [`ConstraintSentence`], and the *bindings* are an
/// ordered list of [`QuantifierBinding`]s
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BoundSentence {
    span: Option<Span>,
    bindings: Vec<QuantifierBinding>, // assert!(!is_empty())
    body: Box<ConstraintSentence>,
}

///
/// Corresponds to the grammar rule `quantifier_binding`.
///
/// A `QuantifierBinding` is either the keyword **`self`** or a *name* and *target* pair.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum QuantifierBinding {
    ReservedSelf,
    Named(QuantifierNamedBinding),
}

///
/// Corresponds to the inner part of the grammar rule `quantifier_binding`.
///
/// A `QuantifierBinding` is either the keyword **`self`** or a *name* and *target* pair.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct QuantifierNamedBinding {
    span: Option<Span>,
    name: Identifier,
    target: BindingTarget,
}

///
/// Corresponds to the grammar rule `binding_target`.
///
/// A `BindingTarget` may be either a [`BindingTypeRef`], or a [`BindingSeqIterator`].
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum BindingTarget {
    Type(BindingTypeRef),
    Iterator(BindingSeqIterator),
}

///
/// Corresponds to the grammar rule `binding_seq_iterator`.
///
/// A named binding may target either a [`BindingTypeRef`], or a [`BindingSeqIterator`].
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum BindingSeqIterator {
    Name(NamePath),
    Type(IdentifierReference),
    Comprehension(SequenceComprehension),
}

///
/// Corresponds to the grammar rule `binding_type_reference`.
///
/// A `BindingTypeRef` is either the keyword **`Self`** or an [`IdentifierReference`].
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum BindingTypeRef {
    ReservedSelfType,
    Type(IdentifierReference),
}

/// Corresponds to the grammar rule `term`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Term {
    Name(NamePath),
    /// Corresponds to the grammar rule `predicate_value`.
    Value(Vec<SimpleValue>),
    Function(Box<FunctionalTerm>),
}

///
/// Corresponds to the grammar rule `name_path`.
///
/// # Semantics
///
/// The keywords **`self`** and **`Self`** may ONLY appear as the first element.
///
/// The name path $x.y.z$ is equivalent to $z(y(x))$, or $(z \circ y)(x)$.
///
/// For example:
///
/// `self.name.length` becomes `length(name(self))`
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct NamePath {
    span: Option<Span>,
    subject: Subject, // assert!(!is_empty())
    path: Vec<Identifier>,
}

/// Corresponds to the field `subject` in the grammar rule `name`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Subject {
    /// Corresponds to the grammar rule `reserved_self`, or the keyword **`self`**.
    ReservedSelf,
    /// Corresponds to the grammar rule `reserved_self_type`, or the keyword **`Self`**.
    ReservedSelfType,
    Identifier(Identifier),
}

/// Corresponds to the grammar rule `functional_term`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionalTerm {
    span: Option<Span>,
    function: Term,
    arguments: Vec<Term>,
}

/// Corresponds to the grammar rule `sequence_comprehension`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SequenceComprehension {
    span: Option<Span>,
    returns: Vec<Identifier>,
    body: Expression,
}

/// Corresponds to the grammar rule `expression`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Expression {
    Boolean(BooleanExpression),
    Quantified(QuantifiedExpression),
    Constraint(ConstraintSentence),
}

///
/// Corresponds to the grammar rule `boolean_expression`.
///
/// Boolean expressions are those that are constructed with the boolean operations negation (not),
/// conjunction (and), disjunction (or), and exclusive disjunction (xor).
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum BooleanExpression {
    /// Corresponds to the grammar rule `expression_negation`. Uses the prefix keyword **`not`**
    /// or the operator $\lnot$.
    Negation(Box<Expression>),
    /// Corresponds to the grammar rule `expression_conjunction`. Uses the infix keyword **`and`**
    /// or the operator $\land$.
    Conjunction(BinaryExpressionOperation),
    /// Corresponds to the grammar rule `expression_disjunction`. Uses the infix keyword **`or`**
    /// or the operator $\lor$.
    Disjunction(BinaryExpressionOperation),
    /// Corresponds to the grammar rule `expression_exclusive_disjunction`. Uses the infix keyword **`xor`**
    /// or the operator $\veebar$. Note that this operation is not a part of ISO Common Logic but
    /// $a \veebar b$ can be rewritten as $\lnot (a \iff b)$
    ExclusiveDisjunction(BinaryExpressionOperation),
}

///
/// Holds the *left* and *right* operands in the rules `conjunction`, `disjunction`,
/// and `exclusive_disjunction`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BinaryExpressionOperation {
    span: Option<Span>,
    left_operand: Box<Expression>,
    right_operand: Box<Expression>,
}

///
/// Corresponds to the grammar rule `quantified_sentence`.
///
/// Such a sentence may be either *universally* or *existentially* quantified.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum QuantifiedExpression {
    /// Corresponds to the grammar rule `universal`. Introduced with the keyword **`forall`**
    /// or the operator $\forall$.
    Universal(BoundExpression),
    /// Corresponds to the grammar rule `existential`. Introduced with the keyword **`exists`**
    /// or the operator $\exists$.
    Existential(BoundExpression),
}

///
/// Corresponds to the inner part of the grammar rule `quantified_expression`,
/// and the rule `quantified_expression_body`.
///
/// The *body* of a bound sentence is a [`Expression`], and the *bindings* are an
/// ordered list of [`QuantifierBinding`]s
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BoundExpression {
    span: Option<Span>,
    bindings: Vec<QuantifierBinding>, // assert!(!is_empty())
    body: Box<Expression>,
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
// Implementations
// ------------------------------------------------------------------------------------------------

impl<B> From<B> for Constraint
where
    B: Into<ConstraintBody>,
{
    fn from(value: B) -> Self {
        Self::anonymous(value)
    }
}

impl Constraint {
    pub fn new<B>(name: Option<Identifier>, body: B) -> Self
    where
        B: Into<ConstraintBody>,
    {
        Self {
            span: None,
            name,
            body: body.into(),
        }
    }

    pub fn named<B>(name: Identifier, body: B) -> Self
    where
        B: Into<ConstraintBody>,
    {
        Self::new(Some(name), body)
    }

    pub fn anonymous<B>(body: B) -> Self
    where
        B: Into<ConstraintBody>,
    {
        Self::new(None, body)
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate!(pub name => option Identifier);

    get_and_mutate!(pub body => ConstraintBody);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(ConstraintBody, Informal, ControlledLanguageString);
impl_from_for_variant!(ConstraintBody, Formal, ConstraintSentence);

impl ConstraintBody {
    is_as_variant!(pub informal => Informal, ControlledLanguageString);
    is_as_variant!(pub formal => Formal, ConstraintSentence);
}

// ------------------------------------------------------------------------------------------------

impl From<String> for ControlledLanguageString {
    fn from(value: String) -> Self {
        Self {
            span: Default::default(),
            value,
            language: Default::default(),
        }
    }
}

impl ControlledLanguageString {
    pub fn new<S>(value: S, language: ControlledLanguageTag) -> Self
    where
        S: Into<String>,
    {
        Self {
            span: Default::default(),
            value: value.into(),
            language: Some(language),
        }
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get_and_mutate!(pub value => String);
    get_and_mutate!(pub language => option ControlledLanguageTag);
}

// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref LANGUAGE_TAG: Regex = Regex::new(r"^[a-z]{2,3}(-[A-Z][A-Za-z]{1,9})?$").unwrap();
}

impl Display for ControlledLanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.value)
    }
}

impl FromStr for ControlledLanguageTag {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self {
                span: None,
                value: s.to_string(),
            })
        } else {
            Err(invalid_language_tag_error(s))
        }
    }
}

into_string_impl!(ControlledLanguageTag, value);
as_str_impl!(ControlledLanguageTag, value);

impl PartialEq for ControlledLanguageTag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for ControlledLanguageTag {}

impl ControlledLanguageTag {
    #[allow(dead_code)]
    pub fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    pub fn is_valid(s: &str) -> bool {
        LANGUAGE_TAG.is_match(s)
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get!(pub value => String);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(ConstraintSentence, Simple, into SimpleSentence);
impl_from_for_variant!(ConstraintSentence, Boolean, BooleanSentence);
impl_from_for_variant!(ConstraintSentence, Quantified, QuantifiedSentence);

impl ConstraintSentence {
    is_as_variant!(pub simple => Simple, SimpleSentence);
    is_as_variant!(pub boolean => Boolean, BooleanSentence);
    is_as_variant!(pub quantified => Quantified, QuantifiedSentence);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(SimpleSentence, Atomic, AtomicSentence);
impl_from_for_variant!(SimpleSentence, Equation, BinaryOperation);

impl SimpleSentence {
    is_as_variant!(pub simple => Atomic, AtomicSentence);
    is_as_variant!(pub boolean => Equation, BinaryOperation);
}

// ------------------------------------------------------------------------------------------------

impl AtomicSentence {
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get_and_mutate!(pub predicate => Term);
    get_and_mutate_collection_of!(pub arguments => Vec, Term);
}

// ------------------------------------------------------------------------------------------------

impl BooleanSentence {
    is_as_variant!(pub negation => Negation, ConstraintSentence);
    is_as_variant!(pub conjunction => Conjunction, BinaryOperation);
    is_as_variant!(pub disjunction => Disjunction, BinaryOperation);
    is_as_variant!(pub exclusive_disjunction => ExclusiveDisjunction, BinaryOperation);
    is_as_variant!(pub implication => Implication, BinaryOperation);
    is_as_variant!(pub biconditional => Biconditional, BinaryOperation);
}

// ------------------------------------------------------------------------------------------------

impl BinaryOperation {
    pub fn new<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<ConstraintSentence>,
        R: Into<ConstraintSentence>,
    {
        Self {
            span: Default::default(),
            left_operand: Box::new(left_operand.into()),
            right_operand: Box::new(right_operand.into()),
        }
    }
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get_and_mutate!(pub left_operand => boxed ConstraintSentence);
    get_and_mutate!(pub right_operand => boxed ConstraintSentence);
}

// ------------------------------------------------------------------------------------------------

impl QuantifiedSentence {
    is_as_variant!(pub universal => Universal, BoundSentence);
    is_as_variant!(pub existential => Existential, BoundSentence);
}

// ------------------------------------------------------------------------------------------------

impl BoundSentence {
    pub fn new<B, S>(bindings: B, body: S) -> Self
    where
        B: Into<Vec<QuantifierBinding>>,
        S: Into<ConstraintSentence>,
    {
        Self {
            span: Default::default(),
            bindings: bindings.into(),
            body: Box::new(body.into()),
        }
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get_and_mutate_collection_of!(pub bindings => Vec, QuantifierBinding);
    get_and_mutate!(pub body => boxed ConstraintSentence);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(QuantifierBinding, Named, QuantifierNamedBinding);

impl QuantifierBinding {
    is_variant!(pub self_instance => empty ReservedSelf);
    is_as_variant!(pub named => Named, QuantifierNamedBinding);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(BindingTarget, Type, BindingTypeRef);
impl_from_for_variant!(BindingTarget, Iterator, BindingSeqIterator);

impl BindingTarget {
    is_as_variant!(pub type_ref => Type, BindingTypeRef);
    is_as_variant!(pub iterator => Iterator, BindingSeqIterator);
}

// ------------------------------------------------------------------------------------------------

impl QuantifierNamedBinding {
    pub fn new<B>(name: Identifier, target: B) -> Self
    where
        B: Into<BindingTarget>,
    {
        Self {
            span: Default::default(),
            name,
            target: target.into(),
        }
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get_and_mutate!(pub name => Identifier);
    get_and_mutate!(pub target => BindingTarget);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(Term, Name, NamePath);
impl_from_for_variant!(Term, Value, Vec<SimpleValue>);
impl_from_for_variant!(Term, Function, Box<FunctionalTerm>);

impl Term {
    is_as_variant!(pub name => Name, NamePath);
    is_as_variant!(pub value => Value, Vec<SimpleValue>);
    is_as_variant!(pub function => Function, Box<FunctionalTerm>);
}

// ------------------------------------------------------------------------------------------------

impl NamePath {
    pub fn new<S, P>(subject: S, path: P) -> Self
    where
        S: Into<Subject>,
        P: Into<Vec<Identifier>>,
    {
        Self {
            span: Default::default(),
            subject: subject.into(),
            path: path.into(),
        }
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get_and_mutate!(pub subject => Subject);
    get_and_mutate_collection_of!(pub path => Vec, Identifier);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(Subject, Identifier, Identifier);

impl Subject {
    is_variant!(pub reserved_self => empty ReservedSelf);
    is_variant!(pub reserved_self_type => empty ReservedSelfType);
    is_as_variant!(pub identifier => Identifier, Identifier);
}

// ------------------------------------------------------------------------------------------------

impl FunctionalTerm {
    pub fn new<T>(function: T) -> Self
    where
        T: Into<Term>,
    {
        Self {
            span: Default::default(),
            function: function.into(),
            arguments: Default::default(),
        }
    }

    pub fn new_with_arguments<T, A>(function: T, arguments: A) -> Self
    where
        T: Into<Term>,
        A: Into<Vec<Term>>,
    {
        Self {
            span: Default::default(),
            function: function.into(),
            arguments: arguments.into(),
        }
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get_and_mutate!(pub function => Term);
    get_and_mutate_collection_of!(pub arguments => Vec, Term);
}

// ------------------------------------------------------------------------------------------------

impl SequenceComprehension {
    pub fn new<I, E>(returns: I, body: E) -> Self
    where
        I: Into<Vec<Identifier>>,
        E: Into<Expression>,
    {
        Self {
            span: Default::default(),
            returns: returns.into(),
            body: body.into(),
        }
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get_and_mutate_collection_of!(pub returns => Vec, Identifier);
    get_and_mutate!(pub body => Expression);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(Expression, Boolean, BooleanExpression);
impl_from_for_variant!(Expression, Quantified, QuantifiedExpression);
impl_from_for_variant!(Expression, Constraint, ConstraintSentence);

impl Expression {
    is_as_variant!(pub boolean => Boolean, BooleanExpression);
    is_as_variant!(pub quantified => Quantified, QuantifiedExpression);
    is_as_variant!(pub constraint => Constraint, ConstraintSentence);
}

// ------------------------------------------------------------------------------------------------

impl BooleanExpression {
    is_as_variant!(pub negation => Negation, Expression);
    is_as_variant!(pub conjunction => Conjunction, BinaryExpressionOperation);
    is_as_variant!(pub disjunction => Disjunction, BinaryExpressionOperation);
    is_as_variant!(pub exclusive_disjunction => ExclusiveDisjunction, BinaryExpressionOperation);
}

// ------------------------------------------------------------------------------------------------

impl BinaryExpressionOperation {
    pub fn new<L, R>(left_operand: L, right_operand: R) -> Self
    where
        L: Into<Expression>,
        R: Into<Expression>,
    {
        Self {
            span: Default::default(),
            left_operand: Box::new(left_operand.into()),
            right_operand: Box::new(right_operand.into()),
        }
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get_and_mutate!(pub left_operand => boxed Expression);
    get_and_mutate!(pub right_operand => boxed Expression);
}

// ------------------------------------------------------------------------------------------------

impl QuantifiedExpression {
    is_as_variant!(pub universal => Universal, BoundExpression);
    is_as_variant!(pub existential => Existential, BoundExpression);
}

// ------------------------------------------------------------------------------------------------

impl BoundExpression {
    pub fn new<B, E>(bindings: B, body: E) -> Self
    where
        B: Into<Vec<QuantifierBinding>>,
        E: Into<Expression>,
    {
        Self {
            span: Default::default(),
            bindings: bindings.into(),
            body: Box::new(body.into()),
        }
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);
    get_and_mutate_collection_of!(pub bindings => Vec, QuantifierBinding);
    get_and_mutate!(pub body => boxed Expression);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
