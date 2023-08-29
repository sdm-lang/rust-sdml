use crate::model::constraints::{PredicateValue, SequenceBuilder};
use crate::model::identifiers::{Identifier, QualifiedIdentifier};
use crate::model::Span;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱ Terms
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `term`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Term {
    Call(FunctionComposition),
    Variable(Identifier),
    Type(QualifiedIdentifier),
    Value(PredicateValue),
    Function(Box<FunctionalTerm>),
    Sequence(Box<SequenceBuilder>),
}

///
/// Corresponds to the grammar rule `function_composition`.
///
/// # Well-Formedness Rules
///
/// 1. The list of function names MUST have at least one element.
///
/// $$\forall r \in FunctionComposition \left( |name(r)| \gte 1 \right)$$
///
/// # Semantics
///
/// The keyword **`self`** may ONLY appear as the first element.
///
/// The name path $x.y.z$ is equivalent to $z(y(x))$, or $(z \circ y)(x)$.
///
/// For example:
///
/// `self.name.length` becomes `length(name(self))`
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionComposition {
    span: Option<Span>,
    subject: Subject, // assert!(!is_empty())
    function_names: Vec<Identifier>,
}

/// Corresponds to the field `subject` in the grammar rule `name`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Subject {
    /// Corresponds to the grammar rule `reserved_self`, or the keyword **`self`**.
    ReservedSelf,
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

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Formal Constraints ❱ Terms
// ------------------------------------------------------------------------------------------------

impl From<FunctionComposition> for Term {
    fn from(v: FunctionComposition) -> Self {
        Self::Call(v)
    }
}

impl From<Identifier> for Term {
    fn from(v: Identifier) -> Self {
        Self::Variable(v)
    }
}

impl From<QualifiedIdentifier> for Term {
    fn from(v: QualifiedIdentifier) -> Self {
        Self::Type(v)
    }
}

impl From<PredicateValue> for Term {
    fn from(v: PredicateValue) -> Self {
        Self::Value(v)
    }
}

impl From<FunctionalTerm> for Term {
    fn from(v: FunctionalTerm) -> Self {
        Self::Function(Box::new(v))
    }
}

impl From<Box<FunctionalTerm>> for Term {
    fn from(v: Box<FunctionalTerm>) -> Self {
        Self::Function(v)
    }
}

impl From<SequenceBuilder> for Term {
    fn from(v: SequenceBuilder) -> Self {
        Self::Sequence(Box::new(v))
    }
}

impl From<Box<SequenceBuilder>> for Term {
    fn from(v: Box<SequenceBuilder>) -> Self {
        Self::Sequence(v)
    }
}

impl Term {
    // --------------------------------------------------------------------------------------------

    pub fn is_call(&self) -> bool {
        matches!(self, Self::Call(_))
    }

    pub fn as_call(&self) -> Option<&FunctionComposition> {
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

    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(_))
    }

    pub fn as_type(&self) -> Option<&QualifiedIdentifier> {
        match self {
            Self::Type(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn as_value(&self) -> Option<&PredicateValue> {
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

impl_has_source_span_for!(FunctionComposition);

impl FunctionComposition {
    pub fn new<S, N>(subject: S, function_names: N) -> Self
    where
        S: Into<Subject>,
        N: Into<Vec<Identifier>>,
    {
        let function_names = function_names.into();
        assert!(!function_names.is_empty());
        Self {
            span: Default::default(),
            subject: subject.into(),
            function_names,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn subject(&self) -> &Subject {
        &self.subject
    }

    pub fn set_subject<S>(&mut self, subject: S)
    where
        S: Into<Subject>,
    {
        self.subject = subject.into();
    }

    // --------------------------------------------------------------------------------------------

    pub fn function_names_len(&self) -> usize {
        self.function_names.len()
    }

    pub fn function_names(&self) -> impl Iterator<Item = &Identifier> {
        self.function_names.iter()
    }

    pub fn function_names_mut(&mut self) -> impl Iterator<Item = &mut Identifier> {
        self.function_names.iter_mut()
    }

    pub fn add_to_function_names(&mut self, value: Identifier) {
        self.function_names.push(value)
    }

    pub fn extend_function_names<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Identifier>,
    {
        self.function_names.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Identifier> for Subject {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

impl Subject {
    // --------------------------------------------------------------------------------------------

    pub fn is_reserved_self(&self) -> bool {
        matches!(self, Self::ReservedSelf)
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

impl_has_source_span_for!(FunctionalTerm);

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
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
