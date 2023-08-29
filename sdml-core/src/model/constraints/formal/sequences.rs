use std::collections::HashSet;
use crate::model::{
    constraints::ConstraintSentence, constraints::QuantifiedVariableBinding, identifiers::Identifier, Span,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱  Sequence Comprehensions
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `sequence_comprehension`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SequenceBuilder {
    span: Option<Span>,
    variables: Variables,
    bindings: Vec<QuantifiedVariableBinding>,
    body: ConstraintSentence,
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
    span: Option<Span>,
    names: HashSet<Identifier>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MappingVariable {
    span: Option<Span>,
    domain: Identifier,
    range: Identifier,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Formal Constraints ❱  Sequence Comprehensions
// ------------------------------------------------------------------------------------------------

impl_has_body_for!(SequenceBuilder, ConstraintSentence);

impl_has_source_span_for!(SequenceBuilder);

impl SequenceBuilder {
    pub fn new<V, B, S>(variables: V, bindings: B, body: S) -> Self
    where
        V: Into<Variables>,
        B: IntoIterator<Item = QuantifiedVariableBinding>,
        S: Into<ConstraintSentence>,
    {
        Self {
            span: Default::default(),
            variables: variables.into(),
            bindings: Vec::from_iter(bindings),
            body: body.into(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    pub fn set_variables<V>(&mut self, variables: V) where V: Into<Variables> {
        self.variables = variables.into();
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_bindings(&self) -> bool {
        !self.bindings.is_empty()
    }

    pub fn bindings_len(&self) -> usize {
        self.bindings.len()
    }

    pub fn bindings(&self) -> impl Iterator<Item = &QuantifiedVariableBinding> {
        self.bindings.iter()
    }

    pub fn bindings_mut(&mut self) -> impl Iterator<Item = &mut QuantifiedVariableBinding> {
        self.bindings.iter_mut()
    }

    pub fn add_to_bindings<I>(&mut self, value: I)
    where
        I: Into<QuantifiedVariableBinding>,
    {
        self.bindings.push(value.into())
    }

    pub fn extend_bindings<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = QuantifiedVariableBinding>,
    {
        self.bindings.extend(extension)
    }
}

// ------------------------------------------------------------------------------------------------

impl From<NamedVariables> for Variables {
    fn from(value: NamedVariables) -> Self {
        Self::Named(value)
    }
}

impl From<MappingVariable> for Variables {
    fn from(value: MappingVariable) -> Self {
        Self::Mapping(value)
    }
}

impl Variables {
    pub fn is_named_set(&self) -> bool {
        matches!(self, Self::Named(_))
    }

    pub fn as_named_set(&self) -> Option<&NamedVariables> {
        match self {
            Self::Named(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_mapping(&self) -> bool {
        matches!(self, Self::Mapping(_))
    }

    pub fn as_mapping(&self) -> Option<&MappingVariable> {
        match self {
            Self::Mapping(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(NamedVariables);

impl FromIterator<Identifier> for NamedVariables {
    fn from_iter<T: IntoIterator<Item = Identifier>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl AsRef<HashSet<Identifier>> for NamedVariables {
    fn as_ref(&self) -> &HashSet<Identifier> {
        &self.names
    }
}

impl AsMut<HashSet<Identifier>> for NamedVariables {
    fn as_mut(&mut self) -> &mut HashSet<Identifier> {
        &mut self.names
    }
}

impl NamedVariables {
    pub fn new(names: HashSet<Identifier>) -> Self {
        Self { span: Default::default(), names }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(MappingVariable);

impl MappingVariable {
    pub fn new(domain: Identifier, range: Identifier) -> Self {
        Self { span: Default::default(), domain, range }
    }

    // --------------------------------------------------------------------------------------------

    pub fn domain(&self) -> &Identifier {
        &self.domain
    }

    pub fn set_domain(&mut self, domain: Identifier)  {
        self.domain = domain;
    }

    // --------------------------------------------------------------------------------------------

    pub fn range(&self) -> &Identifier {
        &self.range
    }

    pub fn set_range(&mut self, range: Identifier)  {
        self.range = range;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
