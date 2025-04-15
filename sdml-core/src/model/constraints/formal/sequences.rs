use crate::model::{
    constraints::{QuantifiedSentence, Variable},
    HasBody, HasSourceSpan, Span,
};

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
    variables: Vec<Variable>,
    body: QuantifiedSentence,
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
        V: IntoIterator<Item = Variable>,
        S: Into<QuantifiedSentence>,
    {
        Self {
            span: Default::default(),
            variables: Vec::from_iter(variables.into_iter()),
            body: body.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn has_variables(&self) -> bool {
        !self.variables.is_empty()
    }

    pub fn variables_len(&self) -> usize {
        self.variables.len()
    }

    pub fn variables(&self) -> impl Iterator<Item = &Variable> {
        self.variables.iter()
    }

    pub fn variables_mut(&mut self) -> impl Iterator<Item = &mut Variable> {
        self.variables.iter_mut()
    }

    pub fn add_to_variables<I>(&mut self, value: I)
    where
        I: Into<Variable>,
    {
        self.variables.push(value.into())
    }

    pub fn extend_variables<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Variable>,
    {
        self.variables.extend(extension)
    }
}
