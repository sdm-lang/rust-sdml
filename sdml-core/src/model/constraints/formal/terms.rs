use crate::model::constraints::{PredicateValue, SequenceBuilder};
use crate::model::identifiers::{Identifier, QualifiedIdentifier, IdentifierReference};
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

    is_as_variant!(Sequence (SequenceBuilder) => is_sequence, as_sequence);

    is_as_variant!(Function (FunctionalTerm) => is_function, as_function);

    is_as_variant!(Composition (FunctionComposition) => is_call, as_call);

    is_as_variant!(Identifier (IdentifierReference) => is_identifier, as_identifier);

    is_as_variant!(Value (PredicateValue) => is_value, as_value);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(FunctionComposition);

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

    get_and_set_vec!(
        pub
        has has_function_names,
        function_names_len,
        function_names,
        function_names_mut,
        add_to_function_names,
        extend_function_names
            => function_names, Identifier
    );
}

// ------------------------------------------------------------------------------------------------

impl From<Identifier> for Subject {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

impl Subject {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_variant!(ReservedSelf => is_reservesd_self);

    is_as_variant!(Identifier (Identifier) => is_identifier, as_identifier);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(FunctionalTerm);

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

    get_and_set!(pub function, set_function => Term);

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
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
