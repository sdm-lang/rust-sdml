use crate::model::{Identifier, IdentifierReference, SimpleValue};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum ConstraintBody {
    Informal(String),
    Formal(ConstraintSentence),
}

#[derive(Clone, Debug)]
pub enum ConstraintSentence {
    Simple(SimpleSentence),
    Boolean(BooleanSentence),
    Quantified(QuantifiedSentence),
}

#[derive(Clone, Debug)]
pub enum SimpleSentence {
    Atomic(AtomicSentence),
    Equation(BinaryOperation),
}

#[derive(Clone, Debug)]
pub struct AtomicSentence {
    predicate: Term,
    arguments: Vec<Term>,
}

#[derive(Clone, Debug)]
pub enum BooleanSentence {
    Negation(UnaryOperation),
    Conjunction(BinaryOperation),
    Disjunction(BinaryOperation),
    Implication(BinaryOperation),
    Biconditional(BinaryOperation),
}

#[derive(Clone, Debug)]
pub struct UnaryOperation {
    operand: Box<ConstraintSentence>,
}

#[derive(Clone, Debug)]
pub struct BinaryOperation {
    left_operand: Box<ConstraintSentence>,
    right_operand: Box<ConstraintSentence>,
}

#[derive(Clone, Debug)]
pub enum QuantifiedSentence {
    Universal(BoundSentence),
    Existential(BoundSentence),
}

#[derive(Clone, Debug)]
pub struct BoundSentence {
    bindings: Vec<Binding>,
    body: Box<ConstraintSentence>,
}

#[derive(Clone, Debug)]
pub struct Binding {
    name: Identifier,
    target_type: Option<IdentifierReference>,
}

#[derive(Clone, Debug)]
pub enum Term {
    Name(NamePath),
    Value(PredicateValue),
    Function(Box<FunctionalTerm>),
}

#[derive(Clone, Debug)]
pub struct NamePath(Vec<Identifier>);

#[derive(Clone, Debug)]
pub enum PredicateValue {
    Simple(SimpleValue),
    List(Vec<SimpleValue>),
}

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
