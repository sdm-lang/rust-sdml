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

    pub fn has_definitions(&self) -> bool {
        !self.environment.is_empty()
    }

    pub fn definitions_len(&self) -> usize {
        self.environment.len()
    }

    pub fn definitions(&self) -> impl Iterator<Item = &EnvironmentDef> {
        self.environment.iter()
    }

    pub fn definitions_mut(&mut self) -> impl Iterator<Item = &mut EnvironmentDef> {
        self.environment.iter_mut()
    }

    pub fn add_to_definitions<I>(&mut self, value: I)
    where
        I: Into<EnvironmentDef>,
    {
        self.environment.push(value.into())
    }

    pub fn extend_definitions<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = EnvironmentDef>,
    {
        self.environment.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod sequences;
pub use sequences::{
    SequenceBuilder, Variables, NamedVariables, MappingVariable, 
};

mod environments;
pub use environments::{EnvironmentDef, EnvironmentDefBody};

mod functions;
pub use functions::{
    FunctionDef, FunctionParameter, FunctionSignature, FunctionType, FunctionCardinality, FunctionTypeReference,
};

mod sentences;
pub use sentences::{
    AtomicSentence, BinaryBooleanSentence, BooleanSentence, ConstraintSentence, IteratorSource,
    QuantifiedSentence, QuantifiedBinding, QuantifierBoundNames, SequenceIterator,
    SimpleSentence, TypeIterator, Equation, Quantifier, Inequation, InequalityRelation,
    QuantifiedVariableBinding, UnaryBooleanSentence, ConnectiveOperator,
};

mod terms;
pub use terms::{FunctionalTerm, FunctionComposition, Subject, Term};

mod values;
pub use values::{SequenceOfPredicateValues, PredicateValue, PredicateSequenceMember};
