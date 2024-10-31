use crate::model::{
    constraints::QuantifiedSentence, identifiers::Identifier, HasBody, HasSourceSpan, Span,
};
use std::collections::BTreeSet;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Constraints ❱  Sequence Comprehensions
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `sequence_comprehension`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SequenceBuilder {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    variables: Variables,
    body: QuantifiedSentence,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Variables {
    Named(NamedVariables),
    Mapping(MappingVariable),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct NamedVariables {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    names: BTreeSet<Identifier>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MappingVariable {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    domain: Identifier,
    range: Identifier,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱  SequenceBuilder
// ------------------------------------------------------------------------------------------------

impl HasBody for SequenceBuilder {
    type Body = QuantifiedSentence;

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

impl HasSourceSpan for SequenceBuilder {
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

impl SequenceBuilder {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<V, S>(variables: V, body: S) -> Self
    where
        V: Into<Variables>,
        S: Into<QuantifiedSentence>,
    {
        Self {
            span: Default::default(),
            variables: variables.into(),
            body: body.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    pub fn set_variables<V>(&mut self, variables: V)
    where
        V: Into<Variables>,
    {
        self.variables = variables.into();
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱  Variables
// ------------------------------------------------------------------------------------------------

impl From<&NamedVariables> for Variables {
    fn from(value: &NamedVariables) -> Self {
        Self::Named(value.clone())
    }
}

impl From<NamedVariables> for Variables {
    fn from(value: NamedVariables) -> Self {
        Self::Named(value)
    }
}

impl From<&MappingVariable> for Variables {
    fn from(value: &MappingVariable) -> Self {
        Self::Mapping(value.clone())
    }
}

impl From<MappingVariable> for Variables {
    fn from(value: MappingVariable) -> Self {
        Self::Mapping(value)
    }
}

impl Variables {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_named_set(&self) -> bool {
        match self {
            Self::Named(_) => true,
            _ => false,
        }
    }

    pub const fn as_named_set(&self) -> Option<&NamedVariables> {
        match self {
            Self::Named(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_mapping(&self) -> bool {
        match self {
            Self::Mapping(_) => true,
            _ => false,
        }
    }

    pub const fn as_mapping(&self) -> Option<&MappingVariable> {
        match self {
            Self::Mapping(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱  NamedVariables
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for NamedVariables {
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

impl FromIterator<Identifier> for NamedVariables {
    fn from_iter<T: IntoIterator<Item = Identifier>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl AsRef<BTreeSet<Identifier>> for NamedVariables {
    fn as_ref(&self) -> &BTreeSet<Identifier> {
        &self.names
    }
}

impl AsMut<BTreeSet<Identifier>> for NamedVariables {
    fn as_mut(&mut self) -> &mut BTreeSet<Identifier> {
        &mut self.names
    }
}

impl NamedVariables {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(names: BTreeSet<Identifier>) -> Self {
        Self {
            span: Default::default(),
            names,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn names(&self) -> impl Iterator<Item = &Identifier> {
        self.names.iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Constraints ❱  MappingVariable
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for MappingVariable {
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

impl MappingVariable {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(domain: Identifier, range: Identifier) -> Self {
        Self {
            span: None,
            domain,
            range,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn domain(&self) -> &Identifier {
        &self.domain
    }

    pub fn set_domain(&mut self, domain: Identifier) {
        self.domain = domain;
    }

    // --------------------------------------------------------------------------------------------

    pub const fn range(&self) -> &Identifier {
        &self.range
    }

    pub fn set_range(&mut self, range: Identifier) {
        self.range = range;
    }
}
