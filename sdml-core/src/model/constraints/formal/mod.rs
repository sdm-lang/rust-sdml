use crate::{
    load::ModuleLoader,
    model::{check::Validate, modules::Module, HasBody, HasSourceSpan, References, Span},
    store::ModuleStore,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Constraints ❱ Formal
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FormalConstraint {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    environment: Vec<FunctionDef>,
    body: ConstraintSentence,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ FormalConstraint
// ------------------------------------------------------------------------------------------------

impl HasBody for FormalConstraint {
    type Body = ConstraintSentence;

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn body_mut(&mut self) -> &mut Self::Body {
        &mut self.body
    }

    fn set_body(&mut self, body: Self::Body) {
        self.body = body;
    }
}

impl HasSourceSpan for FormalConstraint {
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

impl References for FormalConstraint {}

impl Validate for FormalConstraint {
    fn validate(
        &self,
        _top: &Module,
        _cache: &impl ModuleStore,
        _loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
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

    pub fn with_definition<I>(self, definition: FunctionDef) -> Self {
        let mut self_mut = self;
        self_mut.environment.push(definition);
        self_mut
    }

    pub fn with_environment(self, environment: Vec<FunctionDef>) -> Self {
        let mut self_mut = self;
        self_mut.environment = environment;
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn has_definitions(&self) -> bool {
        !self.environment.is_empty()
    }

    pub fn definitions_len(&self) -> usize {
        self.environment.len()
    }

    pub fn definitions(&self) -> impl Iterator<Item = &FunctionDef> {
        self.environment.iter()
    }

    pub fn definitions_mut(&mut self) -> impl Iterator<Item = &mut FunctionDef> {
        self.environment.iter_mut()
    }

    pub fn add_to_definitions<I>(&mut self, value: I)
    where
        I: Into<FunctionDef>,
    {
        self.environment.push(value.into())
    }

    pub fn extend_definitions<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = FunctionDef>,
    {
        self.environment.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod sequences;
pub use sequences::SequenceBuilder;

mod functions;
pub use functions::{
    FunctionBody, FunctionCardinality, FunctionDef, FunctionParameter, FunctionSignature,
    FunctionType, FunctionTypeReference,
};

mod sentences;
pub use sentences::{
    AtomicSentence, BinaryBooleanSentence, BooleanSentence, ConnectiveOperator, ConstraintSentence,
    Equation, InequalityRelation, Inequation, QuantifiedSentence, QuantifiedVariable,
    QuantifiedVariableBinding, Quantifier, SimpleSentence, UnaryBooleanSentence, Variable,
};

mod terms;
pub use terms::{FunctionComposition, FunctionalTerm, Subject, Term};

mod values;
pub use values::{PredicateSequenceMember, PredicateValue, SequenceOfPredicateValues};
