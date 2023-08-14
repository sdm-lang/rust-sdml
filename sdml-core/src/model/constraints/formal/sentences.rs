use crate::model::{Identifier, IdentifierReference, NamePath, SequenceComprehension, Span, Term};

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
    target: IteratorTarget,
}

///
/// Corresponds to the grammar rule `binding_target`.
///
/// A `BindingTarget` may be either a [`BindingTypeRef`], or a [`BindingSeqIterator`].
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum IteratorTarget {
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
    Call(NamePath),
    Variable(Identifier),
    Comprehension(SequenceComprehension),
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

impl From<AtomicSentence> for SimpleSentence {
    fn from(v: AtomicSentence) -> Self {
        Self::Atomic(v)
    }
}

impl From<BinaryOperation> for SimpleSentence {
    fn from(v: BinaryOperation) -> Self {
        Self::Equation(v)
    }
}

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

impl From<ConstraintSentence> for BooleanSentence {
    fn from(v: ConstraintSentence) -> Self {
        Self::Negation(Box::new(v))
    }
}

impl From<Box<ConstraintSentence>> for BooleanSentence {
    fn from(v: Box<ConstraintSentence>) -> Self {
        Self::Negation(v)
    }
}

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

impl From<QuantifierNamedBinding> for QuantifierBinding {
    fn from(v: QuantifierNamedBinding) -> Self {
        Self::Named(v)
    }
}

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

impl QuantifierNamedBinding {
    pub fn new<B>(name: Identifier, target: B) -> Self
    where
        B: Into<IteratorTarget>,
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

    pub fn target(&self) -> &IteratorTarget {
        &self.target
    }
    pub fn set_target(&mut self, target: IteratorTarget) {
        self.target = target;
    }
}

// ------------------------------------------------------------------------------------------------

impl From<TypeIterator> for IteratorTarget {
    fn from(v: TypeIterator) -> Self {
        Self::Type(v)
    }
}

impl From<SequenceIterator> for IteratorTarget {
    fn from(v: SequenceIterator) -> Self {
        Self::Sequence(v)
    }
}

impl IteratorTarget {
    pub fn is_type_iterator(&self) -> bool {
        matches!(self, Self::Type(_))
    }
    pub fn as_type_iterator(&self) -> Option<&TypeIterator> {
        match self {
            Self::Type(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_sequence_iterator(&self) -> bool {
        matches!(self, Self::Sequence(_))
    }
    pub fn as_sequence_iterator(&self) -> Option<&SequenceIterator> {
        match self {
            Self::Sequence(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<NamePath> for SequenceIterator {
    fn from(v: NamePath) -> Self {
        Self::Call(v)
    }
}

impl From<Identifier> for SequenceIterator {
    fn from(v: Identifier) -> Self {
        Self::Variable(v)
    }
}

impl From<SequenceComprehension> for SequenceIterator {
    fn from(v: SequenceComprehension) -> Self {
        Self::Comprehension(v)
    }
}

impl SequenceIterator {
    pub fn is_call_path(&self) -> bool {
        matches!(self, Self::Call(_))
    }
    pub fn as_call_path(&self) -> Option<&NamePath> {
        match self {
            Self::Call(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_variable(&self) -> bool {
        matches!(self, Self::Variable(_))
    }
    pub fn as_variable(&self) -> Option<&Identifier> {
        match self {
            Self::Variable(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_sequence_comprehension(&self) -> bool {
        matches!(self, Self::Comprehension(_))
    }
    pub fn as_sequence_comprehension(&self) -> Option<&SequenceComprehension> {
        match self {
            Self::Comprehension(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<IdentifierReference> for TypeIterator {
    fn from(v: IdentifierReference) -> Self {
        Self::Type(v)
    }
}

impl TypeIterator {
    pub fn is_self_type(&self) -> bool {
        matches!(self, Self::SelfType)
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_type_name(&self) -> bool {
        matches!(self, Self::Type(_))
    }
    pub fn as_type_name(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Type(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
