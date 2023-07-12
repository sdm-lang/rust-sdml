use crate::model::{Identifier, IdentifierReference, SimpleValue};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Corresponds to the field `body` in the grammar rule `constraint`.
///
/// # Semantics
///
/// The domain of discourse, $\mathbb{D}$, is the set of all definitions present in the current
/// module and the set of modules transitively imported by it.
///
#[derive(Clone, Debug)]
pub enum ConstraintBody {
    /// Corresponds to the grammar rule `informal_constraint`.
    Informal(String),
    /// Corresponds to the grammar rule `formal_constraint`.
    Formal(ConstraintSentence),
}

/// Corresponds to the grammar rule `constraint_sentence`.
#[derive(Clone, Debug)]
pub enum ConstraintSentence {
    Simple(SimpleSentence),
    Boolean(BooleanSentence),
    Quantified(QuantifiedSentence),
}

/// Corresponds to the grammar rule `simple_sentence`.
#[derive(Clone, Debug)]
pub enum SimpleSentence {
    Atomic(AtomicSentence),
    /// Corresponds to the grammar rule `equation`.
    Equation(BinaryOperation),
}

/// Corresponds to the grammar rule `atomic_sentence`.
#[derive(Clone, Debug)]
pub struct AtomicSentence {
    predicate: Term,
    arguments: Vec<Term>,
}

/// Corresponds to the grammar rule `boolean_sentence`.
#[derive(Clone, Debug)]
pub enum BooleanSentence {
    /// Corresponds to the grammar rule `negation`. Uses the prefix keyword **`not`**
    /// or the operator $\lnot$.
    Negation(UnaryOperation),
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

/// Used to model the rule `negation`.
#[derive(Clone, Debug)]
pub struct UnaryOperation {
    operand: Box<ConstraintSentence>,
}

/// Used to capture the commonality in rules `conjunction`, `disjunction`, `implication`,
/// and `biconditional`.
#[derive(Clone, Debug)]
pub struct BinaryOperation {
    left_operand: Box<ConstraintSentence>,
    right_operand: Box<ConstraintSentence>,
}

/// Corresponds to the grammar rule `quantified_sentence`.
#[derive(Clone, Debug)]
pub enum QuantifiedSentence {
    /// Corresponds to the grammar rule `universal`. Introduced with the keyword **`forall`**
    /// or the operator $\forall$.
    Universal(BoundSentence),
    /// Corresponds to the grammar rule `existential`. Introduced with the keyword **`exists`**
    /// or the operator $\exists$.
    Existential(BoundSentence),
}

/// Corresponds to the inner part of the grammar rule `quantified_sentence`,
/// and the rule `quantified_body`).
#[derive(Clone, Debug)]
pub struct BoundSentence {
    bindings: Vec<Binding>,
    body: Box<ConstraintSentence>,
}

/// Corresponds to the grammar rule `quantifier_binding`.
#[derive(Clone, Debug)]
pub struct Binding {
    name: Identifier,
    /// Corresponds to the grammar rule `binding_type_reference`.
    target_type: Option<IdentifierReference>,
}

/// Corresponds to the grammar rule `term`.
#[derive(Clone, Debug)]
pub enum Term {
    Name(NamePath),
    Value(PredicateValue),
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
pub struct NamePath(Vec<Name>);

/// Corresponds to the grammar rule `name`.
#[derive(Clone, Debug)]
pub enum Name {
    /// Corresponds to the grammar rule `reserved_self`, or the keyword **`self`**.
    ReservedSelf,
    /// Corresponds to the grammar rule `reserved_self_type`, or the keyword **`Self`**.
    ReservedSelfType,
    Identifier(Identifier),
}

/// Corresponds to the grammar rule `predicate_value`.
#[derive(Clone, Debug)]
pub enum PredicateValue {
    Simple(SimpleValue),
    /// Corresponds to the grammar rule `tautology`, or the symbol $\top$
    Tautology,
    /// Corresponds to the grammar rule `contradiction`, or the symbol $\bot$
    Contradiction,
    List(Vec<SimpleValue>),
}

/// Corresponds to the grammar rule `functional_term`.
#[derive(Clone, Debug)]
pub struct FunctionalTerm {
    function: Term,
    arguments: Vec<Term>,
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

impl_from_for_variant!(ConstraintBody, Informal, into String);
impl_from_for_variant!(ConstraintBody, Formal, ConstraintSentence);

impl ConstraintBody {
    is_as_variant!(pub informal => Informal, String);
    is_as_variant!(pub formal => Formal, ConstraintSentence);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(ConstraintSentence, Simple, into SimpleSentence);
impl_from_for_variant!(ConstraintSentence, Boolean, BooleanSentence);
//impl_from_for_variant!(ConstraintSentence, Equation, BinaryOperation);

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
    get_and_mutate!(pub predicate => Term);
    get_and_mutate_collection_of!(pub arguments => Vec, Term);
}

// ------------------------------------------------------------------------------------------------

impl BooleanSentence {
    is_as_variant!(pub simple => Negation, UnaryOperation);
    is_as_variant!(pub conjunction => Conjunction, BinaryOperation);
    is_as_variant!(pub disjunction => Disjunction, BinaryOperation);
    is_as_variant!(pub implication => Implication, BinaryOperation);
    is_as_variant!(pub biconditional => Biconditional, BinaryOperation);
}

// ------------------------------------------------------------------------------------------------

impl UnaryOperation {
    get_and_mutate!(pub operand => boxed ConstraintSentence);
}

// ------------------------------------------------------------------------------------------------

impl BinaryOperation {
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
    get_and_mutate_collection_of!(pub bindings => Vec, Binding);
    get_and_mutate!(pub body => boxed ConstraintSentence);
}

// ------------------------------------------------------------------------------------------------

impl Binding {
    get_and_mutate!(pub name => Identifier);
    get_and_mutate!(pub target_type => option IdentifierReference);
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(Term, Name, NamePath);
impl_from_for_variant!(Term, Value, PredicateValue);
impl_from_for_variant!(Term, Function, Box<FunctionalTerm>);

impl Term {
    is_as_variant!(pub name => Name, NamePath);
    is_as_variant!(pub value => Value, PredicateValue);
    is_as_variant!(pub function => Function, Box<FunctionalTerm>);
}

// ------------------------------------------------------------------------------------------------

impl PredicateValue {
    is_as_variant!(pub simple => Simple, SimpleValue);
    is_as_variant!(pub list => List, Vec<SimpleValue>);
}

// ------------------------------------------------------------------------------------------------

impl FunctionalTerm {
    get_and_mutate!(pub function => Term);
    get_and_mutate_collection_of!(pub arguments => Vec, Term);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
