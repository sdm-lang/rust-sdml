use crate::model::{Identifier, PredicateValue, QualifiedIdentifier, SequenceComprehension, Span};

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
    Call(NamePath),
    Variable(Identifier),
    Type(QualifiedIdentifier),
    /// Corresponds to the grammar rule `predicate_value`.
    Value(PredicateValue),
    Function(Box<FunctionalTerm>),
    Sequence(Box<SequenceComprehension>),
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

impl From<NamePath> for Term {
    fn from(v: NamePath) -> Self {
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

impl Term {
    // --------------------------------------------------------------------------------------------

    pub fn is_call(&self) -> bool {
        matches!(self, Self::Call(_))
    }
    pub fn as_call(&self) -> Option<&NamePath> {
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
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
