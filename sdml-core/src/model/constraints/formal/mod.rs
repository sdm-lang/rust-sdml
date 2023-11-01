use crate::model::{check::Validate, References, Span};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FormalConstraint {
    span: Option<Span>,
    environment: Vec<EnvironmentDef>,
    body: ConstraintSentence,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Formal Constraints
// ------------------------------------------------------------------------------------------------

impl_has_body_for!(FormalConstraint, ConstraintSentence);

impl_has_source_span_for!(FormalConstraint);

impl References for FormalConstraint {}

impl Validate for FormalConstraint {
    fn is_complete(
        &self,
        _top: &crate::model::modules::Module,
    ) -> Result<bool, crate::error::Error> {
        todo!()
    }

    fn is_valid(
        &self,
        _check_constraints: bool,
        _top: &crate::model::modules::Module,
    ) -> Result<bool, crate::error::Error> {
        todo!()
    }
}

impl FormalConstraint {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<V>(body: V) -> Self
    where
        V: Into<ConstraintSentence>,
    {
        Self {
            span: Default::default(),
            environment: Default::default(),
            body: body.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn with_definition<I>(self, definition: EnvironmentDef) -> Self {
        let mut self_mut = self;
        self_mut.environment.push(definition);
        self_mut
    }

    pub fn with_environment(self, environment: Vec<EnvironmentDef>) -> Self {
        let mut self_mut = self;
        self_mut.environment = environment;
        self_mut
    }

    get_and_set_vec!(
        pub
        has has_definitions,
        definitions_len,
        definitions,
        definitions_mut,
        add_to_definitions,
        extend_definitions
            => environment, EnvironmentDef
    );
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod sequences;
pub use sequences::{MappingVariable, NamedVariables, SequenceBuilder, Variables};

mod environments;
pub use environments::{EnvironmentDef, EnvironmentDefBody};

mod functions;
pub use functions::{
    FunctionCardinality, FunctionDef, FunctionParameter, FunctionSignature, FunctionType,
    FunctionTypeReference, FunctionTypeReferenceInner,
};

mod sentences;
pub use sentences::{
    AtomicSentence, BinaryBooleanSentence, BooleanSentence, ConnectiveOperator, ConstraintSentence,
    Equation, InequalityRelation, Inequation, QuantifiedSentence, QuantifiedVariable,
    QuantifiedVariableBinding, Quantifier, SimpleSentence, UnaryBooleanSentence,
};

mod terms;
pub use terms::{FunctionComposition, FunctionalTerm, Subject, Term};

mod values;
pub use values::{PredicateSequenceMember, PredicateValue, SequenceOfPredicateValues};
