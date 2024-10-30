use crate::model::constraints::{
    BooleanSentence, ConstraintSentence, FunctionDef, PredicateValue, QuantifiedSentence,
    SequenceOfPredicateValues, SimpleSentence,
};
use crate::model::identifiers::Identifier;
use crate::model::values::SimpleValue;
use crate::model::{HasBody, HasName, HasSourceSpan, Span};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Constraints ❱ Environments
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EnvironmentDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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
// Implementations ❱ Constraints ❱ EnvironmentDef
// ------------------------------------------------------------------------------------------------

impl HasName for EnvironmentDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasBody for EnvironmentDef {
    type Body = EnvironmentDefBody;

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

impl HasSourceSpan for EnvironmentDef {
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

impl EnvironmentDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: Identifier, body: EnvironmentDefBody) -> Self {
        Self {
            span: None,
            name,
            body,
        }
    }

    pub fn new_value<V>(name: Identifier, body: V) -> Self
    where
        V: Into<PredicateValue>,
    {
        Self {
            span: None,
            name,
            body: EnvironmentDefBody::Value(body.into()),
        }
    }

    pub fn new_function<V>(name: Identifier, body: V) -> Self
    where
        V: Into<FunctionDef>,
    {
        Self {
            span: None,
            name,
            body: EnvironmentDefBody::Function(body.into()),
        }
    }

    pub fn new_sentence<V>(name: Identifier, body: V) -> Self
    where
        V: Into<ConstraintSentence>,
    {
        Self {
            span: None,
            name,
            body: EnvironmentDefBody::Sentence(body.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱ EnvironmentDefBody
// ------------------------------------------------------------------------------------------------

impl From<&FunctionDef> for EnvironmentDefBody {
    fn from(v: &FunctionDef) -> Self {
        Self::Function(v.clone())
    }
}

impl From<FunctionDef> for EnvironmentDefBody {
    fn from(v: FunctionDef) -> Self {
        Self::Function(v)
    }
}

impl From<&PredicateValue> for EnvironmentDefBody {
    fn from(v: &PredicateValue) -> Self {
        Self::Value(v.clone())
    }
}

impl From<PredicateValue> for EnvironmentDefBody {
    fn from(v: PredicateValue) -> Self {
        Self::Value(v)
    }
}

impl From<&SimpleValue> for EnvironmentDefBody {
    fn from(v: &SimpleValue) -> Self {
        Self::Value(v.clone().into())
    }
}

impl From<SimpleValue> for EnvironmentDefBody {
    fn from(v: SimpleValue) -> Self {
        Self::Value(v.into())
    }
}

impl From<&SequenceOfPredicateValues> for EnvironmentDefBody {
    fn from(v: &SequenceOfPredicateValues) -> Self {
        Self::Value(v.clone().into())
    }
}

impl From<SequenceOfPredicateValues> for EnvironmentDefBody {
    fn from(v: SequenceOfPredicateValues) -> Self {
        Self::Value(v.into())
    }
}

impl From<&ConstraintSentence> for EnvironmentDefBody {
    fn from(v: &ConstraintSentence) -> Self {
        Self::Sentence(v.clone())
    }
}

impl From<ConstraintSentence> for EnvironmentDefBody {
    fn from(v: ConstraintSentence) -> Self {
        Self::Sentence(v)
    }
}

impl From<&SimpleSentence> for EnvironmentDefBody {
    fn from(v: &SimpleSentence) -> Self {
        Self::Sentence(v.clone().into())
    }
}

impl From<SimpleSentence> for EnvironmentDefBody {
    fn from(v: SimpleSentence) -> Self {
        Self::Sentence(v.into())
    }
}

impl From<&BooleanSentence> for EnvironmentDefBody {
    fn from(v: &BooleanSentence) -> Self {
        Self::Sentence(v.clone().into())
    }
}

impl From<BooleanSentence> for EnvironmentDefBody {
    fn from(v: BooleanSentence) -> Self {
        Self::Sentence(v.into())
    }
}

impl From<&QuantifiedSentence> for EnvironmentDefBody {
    fn from(v: &QuantifiedSentence) -> Self {
        Self::Sentence(v.clone().into())
    }
}

impl From<QuantifiedSentence> for EnvironmentDefBody {
    fn from(v: QuantifiedSentence) -> Self {
        Self::Sentence(v.into())
    }
}

impl EnvironmentDefBody {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_value(&self) -> bool {
        match self {
            Self::Value(_) => true,
            _ => false,
        }
    }

    pub const fn as_value(&self) -> Option<&PredicateValue> {
        match self {
            Self::Value(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_function(&self) -> bool {
        match self {
            Self::Function(_) => true,
            _ => false,
        }
    }

    pub const fn as_function(&self) -> Option<&FunctionDef> {
        match self {
            Self::Function(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_sentence(&self) -> bool {
        match self {
            Self::Sentence(_) => true,
            _ => false,
        }
    }

    pub const fn as_sentence(&self) -> Option<&ConstraintSentence> {
        match self {
            Self::Sentence(v) => Some(v),
            _ => None,
        }
    }
}
