/*!
Provide the Rust types that implement *constraint*-related components of the SDML Grammar.
*/
use std::collections::HashSet;

use crate::{
    load::ModuleLoader,
    model::{
        check::Validate, modules::Module, HasBody, HasName, HasSourceSpan, Identifier,
        IdentifierReference, References, Span,
    },
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
// Implementations ❱ Constraints ❱ Constraint
// ------------------------------------------------------------------------------------------------

impl HasName for Constraint {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasBody for Constraint {
    type Body = ConstraintBody;

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

impl HasSourceSpan for Constraint {
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

impl References for Constraint {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body.referenced_annotations(names);
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body.referenced_types(names);
    }
}

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
// Implementations ❱ Constraints ❱ ConstraintBody
// ------------------------------------------------------------------------------------------------

impl From<&ControlledLanguageString> for ConstraintBody {
    fn from(v: &ControlledLanguageString) -> Self {
        Self::Informal(v.clone())
    }
}

impl From<ControlledLanguageString> for ConstraintBody {
    fn from(v: ControlledLanguageString) -> Self {
        Self::Informal(v)
    }
}

impl From<&FormalConstraint> for ConstraintBody {
    fn from(v: &FormalConstraint) -> Self {
        Self::Formal(v.clone())
    }
}

impl From<FormalConstraint> for ConstraintBody {
    fn from(v: FormalConstraint) -> Self {
        Self::Formal(v)
    }
}

impl References for ConstraintBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match self {
            Self::Informal(v) => v.referenced_annotations(names),
            Self::Formal(v) => v.referenced_annotations(names),
        }
    }

    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        match self {
            Self::Informal(v) => v.referenced_types(names),
            Self::Formal(v) => v.referenced_types(names),
        }
    }
}

impl Validate for ConstraintBody {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        match self {
            Self::Informal(v) => v.validate(top, cache, loader, check_constraints),
            Self::Formal(v) => v.validate(top, cache, loader, check_constraints),
        }
    }
}

impl ConstraintBody {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_informal(&self) -> bool {
        match self {
            Self::Informal(_) => true,
            _ => false,
        }
    }

    pub const fn as_informal(&self) -> Option<&ControlledLanguageString> {
        match self {
            Self::Informal(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_formal(&self) -> bool {
        match self {
            Self::Formal(_) => true,
            _ => false,
        }
    }

    pub const fn as_formal(&self) -> Option<&FormalConstraint> {
        match self {
            Self::Formal(v) => Some(v),
            _ => None,
        }
    }
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
