use crate::model::Span;

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

    // --------------------------------------------------------------------------------------------

    pub fn body(&self) -> &ConstraintSentence {
        &self.body
    }

    pub fn set_body(&mut self, body: ConstraintSentence) {
        self.body = body;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod comprehensions;
pub use comprehensions::{
    BinaryExpressionOperation, BooleanExpression, BoundExpression, Expression,
    QuantifiedExpression, SequenceComprehension,
};

mod environments;
pub use environments::{EnvironmentDef, EnvironmentDefBody};

mod functions;
pub use functions::{
    AnyOr, FunctionDef, FunctionParameter, FunctionSignature, FunctionType, FunctionTypeReference,
};

mod sentences;
pub use sentences::{
    AtomicSentence, BooleanSentence, BoundSentence, ConstraintSentence, IteratorTarget,
    QuantifiedSentence, QuantifierBinding, QuantifierNamedBinding, SequenceIterator,
    SimpleSentence, TypeIterator,
};

mod terms;
pub use terms::{FunctionalTerm, NamePath, Subject, Term};

mod values;
pub use values::{PredicateListMember, PredicateValue, PredicateValueList};
