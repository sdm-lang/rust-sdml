use crate::model::{ConstraintSentence, Identifier, QuantifierBinding, Span};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱  Sequence Comprehensions
// ------------------------------------------------------------------------------------------------

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
// Implementations ❱ Formal Constraints ❱  Sequence Comprehensions
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

impl From<BooleanExpression> for Expression {
    fn from(v: BooleanExpression) -> Self {
        Self::Boolean(v)
    }
}

impl From<QuantifiedExpression> for Expression {
    fn from(v: QuantifiedExpression) -> Self {
        Self::Quantified(v)
    }
}

impl From<ConstraintSentence> for Expression {
    fn from(v: ConstraintSentence) -> Self {
        Self::Constraint(v)
    }
}

impl Expression {
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
