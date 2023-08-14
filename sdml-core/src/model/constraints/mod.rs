use crate::model::{Identifier, Span};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Constraints
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `constraint`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Constraint {
    span: Option<Span>,
    name: Identifier,
    body: ConstraintBody,
}

///
/// Corresponds to the field `body` in the grammar rule `constraint`.
///
/// # Semantics
///
/// The domain of discourse, $\mathbb{D}$, is the set of all definitions present in the current
/// module and the set of modules transitively imported by it.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ConstraintBody {
    /// Corresponds to the grammar rule `informal_constraint`.
    Informal(ControlledLanguageString),
    /// Corresponds to the grammar rule `formal_constraint`.
    Formal(FormalConstraint),
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
// Implementations ❱ Constraints
// ------------------------------------------------------------------------------------------------

impl Constraint {
    pub fn new<B>(name: Identifier, body: B) -> Self
    where
        B: Into<ConstraintBody>,
    {
        Self {
            span: None,
            name,
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

    pub fn name(&self) -> &Identifier {
        &self.name
    }
    pub fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    pub fn body(&self) -> &ConstraintBody {
        &self.body
    }
    pub fn set_body(&mut self, body: ConstraintBody) {
        self.body = body;
    }
}

// ------------------------------------------------------------------------------------------------

impl From<ControlledLanguageString> for ConstraintBody {
    fn from(v: ControlledLanguageString) -> Self {
        Self::Informal(v)
    }
}

impl From<FormalConstraint> for ConstraintBody {
    fn from(v: FormalConstraint) -> Self {
        Self::Formal(v)
    }
}

impl ConstraintBody {
    pub fn is_informal(&self) -> bool {
        matches!(self, Self::Informal(_))
    }
    pub fn as_informal(&self) -> Option<&ControlledLanguageString> {
        match self {
            Self::Informal(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_formal(&self) -> bool {
        matches!(self, Self::Formal(_))
    }
    pub fn as_formal(&self) -> Option<&FormalConstraint> {
        match self {
            Self::Formal(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod formal;
pub use formal::{
    AnyOr, AtomicSentence, BinaryExpressionOperation, BooleanExpression, BooleanSentence,
    BoundExpression, BoundSentence, ConstraintSentence, EnvironmentDef, EnvironmentDefBody,
    Expression, FormalConstraint, FunctionDef, FunctionParameter, FunctionSignature, FunctionType,
    FunctionTypeReference, FunctionalTerm, IteratorTarget, NamePath, PredicateListMember,
    PredicateValue, PredicateValueList, QuantifiedExpression, QuantifiedSentence,
    QuantifierBinding, QuantifierNamedBinding, SequenceComprehension, SequenceIterator,
    SimpleSentence, Subject, Term, TypeIterator,
};

mod informal;
pub use informal::{ControlledLanguageString, ControlledLanguageTag};
