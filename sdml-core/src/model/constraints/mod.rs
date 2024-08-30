use crate::{
    load::ModuleLoader,
    model::{Identifier, Span},
    store::ModuleStore,
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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
// Implementations ❱ Constraints
// ------------------------------------------------------------------------------------------------

impl_has_body_for!(Constraint, ConstraintBody);

impl_has_name_for!(Constraint);

impl_has_source_span_for!(Constraint);

impl_references_for!(Constraint => delegate body);

impl Validate for Constraint {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.body.validate(top, cache, loader, check_constraints)
    }
}

impl Constraint {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

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
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Informal (ControlledLanguageString) => is_informal, as_informal);

    is_as_variant!(Formal (FormalConstraint) => is_formal, as_formal);
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod formal;
pub use formal::{
    AtomicSentence, BinaryBooleanSentence, BooleanSentence, ConnectiveOperator, ConstraintSentence,
    EnvironmentDef, EnvironmentDefBody, Equation, FormalConstraint, FunctionCardinality,
    FunctionComposition, FunctionDef, FunctionParameter, FunctionSignature, FunctionType,
    FunctionTypeReference, FunctionTypeReferenceInner, FunctionalTerm, InequalityRelation,
    Inequation, MappingVariable, NamedVariables, PredicateSequenceMember, PredicateValue,
    QuantifiedSentence, QuantifiedVariable, QuantifiedVariableBinding, Quantifier, SequenceBuilder,
    SequenceOfPredicateValues, SimpleSentence, Subject, Term, UnaryBooleanSentence, Variables,
};

mod informal;
pub use informal::{ControlledLanguageString, ControlledLanguageTag};

use super::{check::Validate, modules::Module};
