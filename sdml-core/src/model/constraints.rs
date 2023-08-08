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
    name: Identifier,
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

impl Constraint {
    pub fn new<B>(name: Identifier, body: B) -> Self
    where
        B: Into<ConstraintBody>,
    {
        Self {
            span: None,
            name,
            body: body.into(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn name(&self) -> &Identifier {
        &self.name
    }
    pub fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    pub fn body(&self) -> &ConstraintBody {
        &self.body
    }
    pub fn set_body(&mut self, body: ConstraintBody) {
        self.body = body;
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(ConstraintBody, Informal, ControlledLanguageString);
impl_from_for_variant!(ConstraintBody, Formal, ConstraintSentence);

impl ConstraintBody {
    pub fn is_informal(&self) -> bool {
        matches!(self, Self::Informal(_))
    }
    pub fn as_informal(&self) -> Option<&ControlledLanguageString> {
        match self {
            Self::Informal(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_formal(&self) -> bool {
        matches!(self, Self::Formal(_))
    }
    pub fn as_formal(&self) -> Option<&ConstraintSentence> {
        match self {
            Self::Formal(v) => Some(v),
            _ => None,
        }
    }
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

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn value(&self) -> &String {
        &self.value
    }
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    // --------------------------------------------------------------------------------------------

    pub fn language(&self) -> Option<&ControlledLanguageTag> {
        self.language.as_ref()
    }
    pub fn set_language(&mut self, language: ControlledLanguageTag) {
        self.language = Some(language);
    }
    pub fn unset_language(&mut self) {
        self.language = None;
    }
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

impl From<ControlledLanguageTag> for String {
    fn from(value: ControlledLanguageTag) -> Self {
        value.value
    }
}

impl AsRef<str> for ControlledLanguageTag {
    fn as_ref(&self) -> &str {
        self.value.as_str()
    }
}

impl PartialEq for ControlledLanguageTag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for ControlledLanguageTag {}

impl ControlledLanguageTag {
    pub fn new_unchecked(s: &str) -> Self {
        Self {
            span: None,
            value: s.to_string(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn value(&self) -> &String {
        &self.value
    }
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_valid(s: &str) -> bool {
        LANGUAGE_TAG.is_match(s)
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(ConstraintSentence, Simple, into SimpleSentence);
impl_from_for_variant!(ConstraintSentence, Boolean, BooleanSentence);
impl_from_for_variant!(ConstraintSentence, Quantified, QuantifiedSentence);

impl ConstraintSentence {
    pub fn is_simple(&self) -> bool {
        matches!(self, Self::Simple(_))
    }
    pub fn as_simple(&self) -> Option<&SimpleSentence> {
        match self {
            Self::Simple(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }
    pub fn as_boolean(&self) -> Option<&BooleanSentence> {
        match self {
            Self::Boolean(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_quantified(&self) -> bool {
        matches!(self, Self::Quantified(_))
    }
    pub fn as_quantified(&self) -> Option<&QuantifiedSentence> {
        match self {
            Self::Quantified(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(SimpleSentence, Atomic, AtomicSentence);
impl_from_for_variant!(SimpleSentence, Equation, BinaryOperation);

impl SimpleSentence {
    pub fn is_atomic(&self) -> bool {
        matches!(self, Self::Atomic(_))
    }
    pub fn as_atomic(&self) -> Option<&AtomicSentence> {
        match self {
            Self::Atomic(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_equation(&self) -> bool {
        matches!(self, Self::Equation(_))
    }
    pub fn as_equation(&self) -> Option<&BinaryOperation> {
        match self {
            Self::Equation(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl AtomicSentence {
    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn predicate(&self) -> &Term {
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
    pub fn add_to_arguments(&mut self, value: Term) {
        self.arguments.push(value)
    }
    pub fn extend_arguments<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Term>,
    {
        self.arguments.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------

// Box:: impl_from_for_variant!(BooleanSentence, Negation, ConstraintSentence);

impl BooleanSentence {
    // --------------------------------------------------------------------------------------------

    pub fn is_negation(&self) -> bool {
        matches!(self, Self::Negation(_))
    }
    pub fn as_negation(&self) -> Option<&ConstraintSentence> {
        match self {
            Self::Negation(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_conjunction(&self) -> bool {
        matches!(self, Self::Conjunction(_))
    }
    pub fn as_conjunction(&self) -> Option<&BinaryOperation> {
        match self {
            Self::Conjunction(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_disjunction(&self) -> bool {
        matches!(self, Self::Disjunction(_))
    }
    pub fn as_disjunction(&self) -> Option<&BinaryOperation> {
        match self {
            Self::Disjunction(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_exclusive_disjunction(&self) -> bool {
        matches!(self, Self::ExclusiveDisjunction(_))
    }
    pub fn as_exclusive_disjunction(&self) -> Option<&BinaryOperation> {
        match self {
            Self::ExclusiveDisjunction(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_implication(&self) -> bool {
        matches!(self, Self::Implication(_))
    }
    pub fn as_implication(&self) -> Option<&BinaryOperation> {
        match self {
            Self::Implication(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_biconditional(&self) -> bool {
        matches!(self, Self::Biconditional(_))
    }
    pub fn as_biconditional(&self) -> Option<&BinaryOperation> {
        match self {
            Self::Biconditional(v) => Some(v),
            _ => None,
        }
    }
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

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn left_operand(&self) -> &ConstraintSentence {
        &self.left_operand
    }
    pub fn set_left_operand(&mut self, operand: ConstraintSentence) {
        self.left_operand = Box::new(operand);
    }

    // --------------------------------------------------------------------------------------------

    pub fn right_operand(&self) -> &ConstraintSentence {
        &self.right_operand
    }
    pub fn set_right_operand(&mut self, operand: ConstraintSentence) {
        self.right_operand = Box::new(operand);
    }
}

// ------------------------------------------------------------------------------------------------

impl QuantifiedSentence {
    pub fn is_universal(&self) -> bool {
        matches!(self, Self::Universal(_))
    }
    pub fn as_universal(&self) -> Option<&BoundSentence> {
        match self {
            Self::Universal(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_existential(&self) -> bool {
        matches!(self, Self::Existential(_))
    }
    pub fn as_existential(&self) -> Option<&BoundSentence> {
        match self {
            Self::Existential(v) => Some(v),
            _ => None,
        }
    }
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

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn has_bindings(&self) -> bool {
        !self.bindings.is_empty()
    }
    pub fn bindings_len(&self) -> usize {
        self.bindings.len()
    }
    pub fn bindings(&self) -> impl Iterator<Item = &QuantifierBinding> {
        self.bindings.iter()
    }
    pub fn bindings_mut(&mut self) -> impl Iterator<Item = &mut QuantifierBinding> {
        self.bindings.iter_mut()
    }
    pub fn add_to_bindings<I>(&mut self, value: I)
    where
        I: Into<QuantifierBinding>,
    {
        self.bindings.push(value.into())
    }
    pub fn extend_bindings<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = QuantifierBinding>,
    {
        self.bindings.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn body(&self) -> &ConstraintSentence {
        &self.body
    }
    pub fn set_body(&mut self, body: ConstraintSentence) {
        self.body = Box::new(body);
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(QuantifierBinding, Named, QuantifierNamedBinding);

impl QuantifierBinding {
    // --------------------------------------------------------------------------------------------

    pub fn is_self_instance(&self) -> bool {
        matches!(self, Self::ReservedSelf)
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }
    pub fn as_named(&self) -> Option<&QuantifierNamedBinding> {
        match self {
            Self::Named(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(BindingTarget, Type, BindingTypeRef);
impl_from_for_variant!(BindingTarget, Iterator, BindingSeqIterator);

impl BindingTarget {
    // --------------------------------------------------------------------------------------------

    pub fn is_type_ref(&self) -> bool {
        matches!(self, Self::Type(_))
    }
    pub fn as_type_ref(&self) -> Option<&BindingTypeRef> {
        match self {
            Self::Type(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_iterator(&self) -> bool {
        matches!(self, Self::Iterator(_))
    }
    pub fn as_iterator(&self) -> Option<&BindingSeqIterator> {
        match self {
            Self::Iterator(v) => Some(v),
            _ => None,
        }
    }
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

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn name(&self) -> &Identifier {
        &self.name
    }
    pub fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    pub fn target(&self) -> &BindingTarget {
        &self.target
    }
    pub fn set_target(&mut self, target: BindingTarget) {
        self.target = target;
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(Term, Name, NamePath);
impl_from_for_variant!(Term, Value, Vec<SimpleValue>);
impl_from_for_variant!(Term, Function, Box<FunctionalTerm>);

impl Term {
    // --------------------------------------------------------------------------------------------

    pub fn is_name(&self) -> bool {
        matches!(self, Self::Name(_))
    }
    pub fn as_name(&self) -> Option<&NamePath> {
        match self {
            Self::Name(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }
    pub fn as_value(&self) -> Option<&Vec<SimpleValue>> {
        match self {
            Self::Value(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function(_))
    }
    pub fn as_function(&self) -> Option<&FunctionalTerm> {
        match self {
            Self::Function(v) => Some(v),
            _ => None,
        }
    }
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

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn subject(&self) -> &Subject {
        &self.subject
    }
    pub fn set_subject(&mut self, subject: Subject) {
        self.subject = subject;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_path(&self) -> bool {
        !self.path.is_empty()
    }
    pub fn path_len(&self) -> usize {
        self.path.len()
    }
    pub fn path(&self) -> impl Iterator<Item = &Identifier> {
        self.path.iter()
    }
    pub fn path_mut(&mut self) -> impl Iterator<Item = &mut Identifier> {
        self.path.iter_mut()
    }
    pub fn add_to_path(&mut self, value: Identifier) {
        self.path.push(value)
    }
    pub fn extend_path<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Identifier>,
    {
        self.path.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(Subject, Identifier, Identifier);

impl Subject {
    // --------------------------------------------------------------------------------------------

    pub fn is_reserved_self(&self) -> bool {
        matches!(self, Self::ReservedSelf)
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_reserved_self_type(&self) -> bool {
        matches!(self, Self::ReservedSelfType)
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            Self::Identifier(v) => Some(v),
            _ => None,
        }
    }
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

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn function(&self) -> &Term {
        &self.function
    }
    pub fn set_function(&mut self, function: Term) {
        self.function = function;
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

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn has_returns(&self) -> bool {
        !self.returns.is_empty()
    }
    pub fn returns_len(&self) -> usize {
        self.returns.len()
    }
    pub fn returns(&self) -> impl Iterator<Item = &Identifier> {
        self.returns.iter()
    }
    pub fn returns_mut(&mut self) -> impl Iterator<Item = &mut Identifier> {
        self.returns.iter_mut()
    }
    pub fn add_to_returns<I>(&mut self, value: I)
    where
        I: Into<Identifier>,
    {
        self.returns.push(value.into())
    }
    pub fn extend_returns<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Identifier>,
    {
        self.returns.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn body(&self) -> &Expression {
        &self.body
    }
    pub fn set_body(&mut self, body: Expression) {
        self.body = body;
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(Expression, Boolean, BooleanExpression);
impl_from_for_variant!(Expression, Quantified, QuantifiedExpression);
impl_from_for_variant!(Expression, Constraint, ConstraintSentence);

impl Expression {
    // --------------------------------------------------------------------------------------------

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }
    pub fn as_boolean(&self) -> Option<&BooleanExpression> {
        match self {
            Self::Boolean(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_quantified(&self) -> bool {
        matches!(self, Self::Quantified(_))
    }
    pub fn as_quantified(&self) -> Option<&QuantifiedExpression> {
        match self {
            Self::Quantified(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_constraint(&self) -> bool {
        matches!(self, Self::Constraint(_))
    }
    pub fn as_constraint(&self) -> Option<&ConstraintSentence> {
        match self {
            Self::Constraint(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl BooleanExpression {
    pub fn is_negation(&self) -> bool {
        matches!(self, Self::Negation(_))
    }
    pub fn as_negation(&self) -> Option<&Expression> {
        match self {
            Self::Negation(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_conjunction(&self) -> bool {
        matches!(self, Self::Conjunction(_))
    }
    pub fn as_conjunction(&self) -> Option<&BinaryExpressionOperation> {
        match self {
            Self::Conjunction(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_disjunction(&self) -> bool {
        matches!(self, Self::Disjunction(_))
    }
    pub fn as_disjunction(&self) -> Option<&BinaryExpressionOperation> {
        match self {
            Self::Disjunction(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_exclusive_disjunction(&self) -> bool {
        matches!(self, Self::ExclusiveDisjunction(_))
    }
    pub fn as_exclusive_disjunction(&self) -> Option<&BinaryExpressionOperation> {
        match self {
            Self::ExclusiveDisjunction(v) => Some(v),
            _ => None,
        }
    }
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

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn left_operand(&self) -> &Expression {
        &self.left_operand
    }
    pub fn set_left_operand(&mut self, left_operand: Expression) {
        self.left_operand = Box::new(left_operand);
    }

    // --------------------------------------------------------------------------------------------

    pub fn right_operand(&self) -> &Expression {
        &self.right_operand
    }
    pub fn set_right_operand(&mut self, right_operand: Expression) {
        self.right_operand = Box::new(right_operand);
    }
}

// ------------------------------------------------------------------------------------------------

impl QuantifiedExpression {
    pub fn is_universal(&self) -> bool {
        matches!(self, Self::Universal(_))
    }
    pub fn as_universal(&self) -> Option<&BoundExpression> {
        match self {
            Self::Universal(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_existential(&self) -> bool {
        matches!(self, Self::Existential(_))
    }
    pub fn as_existential(&self) -> Option<&BoundExpression> {
        match self {
            Self::Existential(v) => Some(v),
            _ => None,
        }
    }
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

    // --------------------------------------------------------------------------------------------

    pub fn with_ts_span(self, ts_span: Span) -> Self {
        Self {
            span: Some(ts_span),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------

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

    pub fn has_bindings(&self) -> bool {
        !self.bindings.is_empty()
    }
    pub fn bindings_len(&self) -> usize {
        self.bindings.len()
    }
    pub fn bindings(&self) -> impl Iterator<Item = &QuantifierBinding> {
        self.bindings.iter()
    }
    pub fn bindings_mut(&mut self) -> impl Iterator<Item = &mut QuantifierBinding> {
        self.bindings.iter_mut()
    }
    pub fn add_to_bindings(&mut self, value: QuantifierBinding) {
        self.bindings.push(value)
    }
    pub fn extend_bindings<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = QuantifierBinding>,
    {
        self.bindings.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn body(&self) -> &Expression {
        &self.body
    }
    pub fn set_body(&mut self, body: Expression) {
        self.body = Box::new(body);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
