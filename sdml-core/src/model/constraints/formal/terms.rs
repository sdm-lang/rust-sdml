use crate::model::constraints::{PredicateValue, SequenceBuilder};
use crate::model::identifiers::{Identifier, IdentifierReference, QualifiedIdentifier};
use crate::model::{HasSourceSpan, Span};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Constraints ❱ Terms
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `term`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Term {
    Sequence(Box<SequenceBuilder>),
    Function(Box<FunctionalTerm>),
    Composition(FunctionComposition),
    Identifier(IdentifierReference),
    ReservedSelf,
    Value(PredicateValue),
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    subject: Subject,                // <- should be term?
    function_names: Vec<Identifier>, // assert!(!is_empty())
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    function: Term,
    arguments: Vec<Term>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ Term
// ------------------------------------------------------------------------------------------------

impl From<FunctionComposition> for Term {
    fn from(v: FunctionComposition) -> Self {
        Self::Composition(v)
    }
}

impl From<IdentifierReference> for Term {
    fn from(v: IdentifierReference) -> Self {
        Self::Identifier(v)
    }
}

impl From<Identifier> for Term {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v.into())
    }
}

impl From<QualifiedIdentifier> for Term {
    fn from(v: QualifiedIdentifier) -> Self {
        Self::Identifier(v.into())
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
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_sequence(&self) -> bool {
        matches!(self, Self::Sequence(_))
    }
    pub const fn as_sequence(&self) -> Option<&SequenceBuilder> {
        match self {
            Self::Sequence(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub const fn is_function(&self) -> bool {
        matches!(self, Self::Function(_))
    }

    pub const fn as_function(&self) -> Option<&FunctionalTerm> {
        match self {
            Self::Function(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub const fn is_call(&self) -> bool {
        matches!(self, Self::Composition(_))
    }

    pub const fn as_call(&self) -> Option<&FunctionComposition> {
        match self {
            Self::Composition(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub const fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }

    pub const fn as_identifier(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Identifier(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub const fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub const fn as_value(&self) -> Option<&PredicateValue> {
        match self {
            Self::Value(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ FunctionComposition
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for FunctionComposition {
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

impl FunctionComposition {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

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
    // Fields
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

    pub fn has_function_names(&self) -> bool {
        !self.function_names.is_empty()
    }

    pub fn function_names_len(&self) -> usize {
        self.function_names.len()
    }

    pub fn function_names(&self) -> impl Iterator<Item = &Identifier> {
        self.function_names.iter()
    }

    pub fn function_names_mut(&mut self) -> impl Iterator<Item = &mut Identifier> {
        self.function_names.iter_mut()
    }

    pub fn add_to_function_names<I>(&mut self, value: I)
    where
        I: Into<Identifier>,
    {
        self.function_names.push(value.into())
    }

    pub fn extend_function_names<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Identifier>,
    {
        self.function_names.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ Subject
// ------------------------------------------------------------------------------------------------

impl From<&Identifier> for Subject {
    fn from(v: &Identifier) -> Self {
        Self::Identifier(v.clone())
    }
}

impl From<Identifier> for Subject {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

impl Subject {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_reserved_self(&self) -> bool {
        matches!(self, Self::ReservedSelf)
    }

    // --------------------------------------------------------------------------------------------

    pub const fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }

    pub const fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            Self::Identifier(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ FunctionalTerm
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for FunctionalTerm {
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

impl FunctionalTerm {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

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
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn function(&self) -> &Term {
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
