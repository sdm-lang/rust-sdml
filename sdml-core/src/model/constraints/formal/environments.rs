use crate::model::constraints::{
    BooleanSentence, ConstraintSentence, FunctionDef, PredicateValue, SequenceOfPredicateValues,
    QuantifiedSentence, SimpleSentence,
};
use crate::model::identifiers::Identifier;
use crate::model::values::SimpleValue;
use crate::model::Span;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱ Environments
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EnvironmentDef {
    span: Option<Span>,
    name: Identifier,
    body: EnvironmentDefBody,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum EnvironmentDefBody {
    Function(FunctionDef),
    Value(PredicateValue),
    Sentence(ConstraintSentence),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Formal Constraints ❱ Environments
// ------------------------------------------------------------------------------------------------

impl_has_body_for!(EnvironmentDef, EnvironmentDefBody);

impl_has_name_for!(EnvironmentDef);

impl_has_source_span_for!(EnvironmentDef);

impl EnvironmentDef {
    pub fn new(name: Identifier, body: EnvironmentDefBody) -> Self {
        Self {
            span: Default::default(),
            name,
            body,
        }
    }

    pub fn new_value<V>(name: Identifier, body: V) -> Self
    where
        V: Into<PredicateValue>,
    {
        Self {
            span: Default::default(),
            name,
            body: EnvironmentDefBody::Value(body.into()),
        }
    }

    pub fn new_function<V>(name: Identifier, body: V) -> Self
    where
        V: Into<FunctionDef>,
    {
        Self {
            span: Default::default(),
            name,
            body: EnvironmentDefBody::Function(body.into()),
        }
    }

    pub fn new_sentence<V>(name: Identifier, body: V) -> Self
    where
        V: Into<ConstraintSentence>,
    {
        Self {
            span: Default::default(),
            name,
            body: EnvironmentDefBody::Sentence(body.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<FunctionDef> for EnvironmentDefBody {
    fn from(v: FunctionDef) -> Self {
        Self::Function(v)
    }
}

impl From<PredicateValue> for EnvironmentDefBody {
    fn from(v: PredicateValue) -> Self {
        Self::Value(v)
    }
}

impl From<SimpleValue> for EnvironmentDefBody {
    fn from(v: SimpleValue) -> Self {
        Self::Value(v.into())
    }
}

impl From<SequenceOfPredicateValues> for EnvironmentDefBody {
    fn from(v: SequenceOfPredicateValues) -> Self {
        Self::Value(v.into())
    }
}

impl From<ConstraintSentence> for EnvironmentDefBody {
    fn from(v: ConstraintSentence) -> Self {
        Self::Sentence(v)
    }
}

impl From<SimpleSentence> for EnvironmentDefBody {
    fn from(v: SimpleSentence) -> Self {
        Self::Sentence(v.into())
    }
}

impl From<BooleanSentence> for EnvironmentDefBody {
    fn from(v: BooleanSentence) -> Self {
        Self::Sentence(v.into())
    }
}

impl From<QuantifiedSentence> for EnvironmentDefBody {
    fn from(v: QuantifiedSentence) -> Self {
        Self::Sentence(v.into())
    }
}

impl EnvironmentDefBody {
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
    pub fn as_function(&self) -> Option<&FunctionDef> {
        match self {
            Self::Function(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_sentence(&self) -> bool {
        matches!(self, Self::Sentence(_))
    }
    pub fn as_sentence(&self) -> Option<&ConstraintSentence> {
        match self {
            Self::Sentence(v) => Some(v),
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
