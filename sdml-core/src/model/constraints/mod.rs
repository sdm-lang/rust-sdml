use crate::{
    error::Error,
    model::{Identifier, Span},
};

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

impl_has_body_for!(Constraint, ConstraintBody);

impl_has_name_for!(Constraint);

impl_has_source_span_for!(Constraint);

impl_references_for!(Constraint => delegate body);

impl Validate for Constraint {
    fn is_complete(&self, _top: &Module) -> Result<bool, Error> {
        // TODO: is this correct?
        Ok(true)
    }

    fn is_valid(&self, check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        if check_constraints {
            todo!()
        } else {
            Ok(true)
        }
    }
}

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

impl_references_for!(ConstraintBody => variants Informal, Formal);

impl_validate_for!(ConstraintBody => variants Informal, Formal);

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
    AtomicSentence, BinaryBooleanSentence, BooleanSentence,
    ConstraintSentence, EnvironmentDef, EnvironmentDefBody,
    FormalConstraint, FunctionDef, FunctionParameter, FunctionSignature, FunctionType,
    FunctionTypeReference, FunctionalTerm, IteratorSource, FunctionComposition, PredicateSequenceMember,
    PredicateValue, SequenceOfPredicateValues, QuantifiedSentence,Inequation, InequalityRelation,
    QuantifiedBinding, QuantifierBoundNames, SequenceBuilder, SequenceIterator,
    SimpleSentence, Subject, Term, TypeIterator, Equation, Quantifier, Variables,NamedVariables, MappingVariable,
    QuantifiedVariableBinding, FunctionCardinality,UnaryBooleanSentence, ConnectiveOperator,
};

mod informal;
pub use informal::{ControlledLanguageString, ControlledLanguageTag};

use super::{check::Validate, modules::Module};
