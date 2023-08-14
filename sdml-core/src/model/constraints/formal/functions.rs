use crate::model::{Cardinality, ConstraintSentence, Identifier, IdentifierReference, MappingType, Span};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱ Environments ❱ Functions
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionDef {
    span: Option<Span>,
    signature: FunctionSignature,
    body: ConstraintSentence,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionSignature {
    span: Option<Span>,
    parameters: Vec<FunctionParameter>,
    target_type: FunctionType,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionParameter {
    span: Option<Span>,
    name: Identifier,
    target_type: FunctionType,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionType {
    span: Option<Span>,
    target_cardinality: AnyOr<Cardinality>,
    target_type: AnyOr<FunctionTypeReference>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum FunctionTypeReference {
    Reference(IdentifierReference),
    // builtin_simple_type is converted into a reference
    MappingType(MappingType),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AnyOr<T> {
    Any,
    Some(T),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Formal Constraints ❱ Environments ❱ Functions
// ------------------------------------------------------------------------------------------------

impl FunctionDef {
    pub fn new(signature: FunctionSignature, body: ConstraintSentence) -> Self {
        Self {
            span: Default::default(),
            signature,
            body,
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

    pub fn signature(&self) -> &FunctionSignature {
        &self.signature
    }
    pub fn set_signature(&mut self, signature: FunctionSignature) {
        self.signature = signature;
    }

    // --------------------------------------------------------------------------------------------

    pub fn body(&self) -> &ConstraintSentence {
        &self.body
    }
    pub fn set_target_type(&mut self, body: ConstraintSentence) {
        self.body = body;
    }
}

// ------------------------------------------------------------------------------------------------

impl FunctionSignature {
    pub fn new(parameters: Vec<FunctionParameter>, target_type: FunctionType) -> Self {
        Self {
            span: Default::default(),
            parameters,
            target_type,
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

    pub fn has_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }
    pub fn parameters_len(&self) -> usize {
        self.parameters.len()
    }
    pub fn parameters(&self) -> impl Iterator<Item = &FunctionParameter> {
        self.parameters.iter()
    }
    pub fn parameters_mut(&mut self) -> impl Iterator<Item = &mut FunctionParameter> {
        self.parameters.iter_mut()
    }
    pub fn add_to_parameters(&mut self, value: FunctionParameter) {
        self.parameters.push(value)
    }
    pub fn extend_parameters<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = FunctionParameter>,
    {
        self.parameters.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn target_type(&self) -> &FunctionType {
        &self.target_type
    }
    pub fn set_target_type(&mut self, target_type: FunctionType) {
        self.target_type = target_type;
    }
}

// ------------------------------------------------------------------------------------------------

impl FunctionParameter {
    pub fn new(name: Identifier, target_type: FunctionType) -> Self {
        Self {
            span: Default::default(),
            name,
            target_type,
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

    pub fn target_type(&self) -> &FunctionType {
        &self.target_type
    }
    pub fn set_target_type(&mut self, target_type: FunctionType) {
        self.target_type = target_type;
    }
}

// ------------------------------------------------------------------------------------------------

impl FunctionType {
    pub fn new(
        target_cardinality: AnyOr<Cardinality>,
        target_type: AnyOr<FunctionTypeReference>,
    ) -> Self {
        Self {
            span: Default::default(),
            target_cardinality,
            target_type,
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

    pub fn with_any_cardinality(self) -> Self {
        Self {
            target_cardinality: AnyOr::Any,
            ..self
        }
    }

    pub fn with_target_cardinality(self, target_cardinality: Cardinality) -> Self {
        Self {
            target_cardinality: AnyOr::Some(target_cardinality),
            ..self
        }
    }

    pub fn target_cardinality(&self) -> &AnyOr<Cardinality> {
        &self.target_cardinality
    }
    pub fn set_target_cardinality(&mut self, target_cardinality: AnyOr<Cardinality>) {
        self.target_cardinality = target_cardinality;
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_any_type(self) -> Self {
        Self {
            target_cardinality: AnyOr::Any,
            ..self
        }
    }

    pub fn with_target_type(self, target_type: FunctionTypeReference) -> Self {
        Self {
            target_type: AnyOr::Some(target_type),
            ..self
        }
    }

    pub fn target_type(&self) -> &AnyOr<FunctionTypeReference> {
        &self.target_type
    }
    pub fn set_target_type(&mut self, target_type: AnyOr<FunctionTypeReference>) {
        self.target_type = target_type;
    }
}

// ------------------------------------------------------------------------------------------------

impl From<IdentifierReference> for FunctionTypeReference {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl From<MappingType> for FunctionTypeReference {
    fn from(value: MappingType) -> Self {
        Self::MappingType(value)
    }
}

impl FunctionTypeReference {
    pub fn is_type_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }
    pub fn as_type_reference(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Reference(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_mapping_type(&self) -> bool {
        matches!(self, Self::MappingType(_))
    }
    pub fn as_mapping_type(&self) -> Option<&MappingType> {
        match self {
            Self::MappingType(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<T> From<T> for AnyOr<T> {
    fn from(value: T) -> Self {
        Self::Some(value)
    }
}

impl<T> AnyOr<T> {
    pub fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }
    pub fn as_some(&self) -> Option<&T> {
        match self {
            Self::Some(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(&T) -> U,
    {
        self.as_some().map(f)
    }

    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        self.as_some().map_or(default, f)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
