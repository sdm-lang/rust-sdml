use crate::model::{
    Annotation, AnnotationOnlyBody, AnnotationProperty, Cardinality, Constraint, Identifier,
    IdentifierReference, ModelElement, Span, TypeReference,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Properties
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `property_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PropertyDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<PropertyBody>,
}

/// Corresponds to the grammar rule `property_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PropertyBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    roles: Vec<PropertyRole>, // assert!(!roles.is_empty());
}

/// Corresponds to the grammar rule `property_role`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PropertyRole {
    span: Option<Span>,
    name: Identifier,
    target_type: TypeReference,
    inverse_name: Option<Option<Identifier>>,
    target_cardinality: Option<Cardinality>,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Properties
// ------------------------------------------------------------------------------------------------

impl ModelElement for PropertyDef {
    fn ts_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    fn set_ts_span(&mut self, span: Span) {
        self.span = Some(span);
    }
    fn unset_ts_span(&mut self) {
        self.span = None;
    }

    // --------------------------------------------------------------------------------------------

    fn name(&self) -> &Identifier {
        &self.name
    }
    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }

    // --------------------------------------------------------------------------------------------

    fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    // --------------------------------------------------------------------------------------------

    fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_types())
            .unwrap_or_default()
    }
}

impl PropertyDef {
    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
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

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    pub fn body(&self) -> Option<&PropertyBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: PropertyBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl PropertyBody {
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

    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    pub fn annotations_len(&self) -> usize {
        self.annotations.len()
    }
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }
    pub fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }
    pub fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }
    pub fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension)
    }

    pub fn annotation_properties(&self) -> impl Iterator<Item = &AnnotationProperty> {
        self.annotations()
            .filter_map(|a| a.as_annotation_property())
    }

    pub fn annotation_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.annotations().filter_map(|a| a.as_constraint())
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_roles(&self) -> bool {
        !self.roles.is_empty()
    }
    pub fn roles_len(&self) -> usize {
        self.roles.len()
    }
    pub fn roles(&self) -> impl Iterator<Item = &PropertyRole> {
        self.roles.iter()
    }
    pub fn roles_mut(&mut self) -> impl Iterator<Item = &mut PropertyRole> {
        self.roles.iter_mut()
    }
    pub fn add_to_roles(&mut self, value: PropertyRole) {
        self.roles.push(value)
    }
    pub fn extend_roles<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = PropertyRole>,
    {
        self.roles.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.annotation_properties().map(|a| a.name()).collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.roles()
            .flat_map(|role| role.referenced_types())
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl PropertyRole {
    pub fn new(name: Identifier, target_type: TypeReference) -> Self {
        Self {
            span: None,
            name,
            target_type,
            inverse_name: Default::default(),
            target_cardinality: Default::default(),
            body: None,
        }
    }

    pub fn new_unknown(name: Identifier) -> Self {
        Self::new(name, TypeReference::Unknown)
    }

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

    pub fn target_type(&self) -> &TypeReference {
        &self.target_type
    }
    pub fn set_target_type(&mut self, target_type: TypeReference) {
        self.target_type = target_type;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_inverse_name(&self) -> bool {
        self.inverse_name().is_some()
    }
    pub fn inverse_name(&self) -> Option<&Option<Identifier>> {
        self.inverse_name.as_ref()
    }
    pub fn set_inverse_name(&mut self, inverse_name: Option<Identifier>) {
        self.inverse_name = Some(inverse_name);
    }
    pub fn unset_inverse_name(&mut self) {
        self.inverse_name = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_target_cardinality(&self) -> bool {
        self.target_cardinality().is_some()
    }
    pub fn target_cardinality(&self) -> Option<&Cardinality> {
        self.target_cardinality.as_ref()
    }
    pub fn set_target_cardinality(&mut self, target_cardinality: Cardinality) {
        self.target_cardinality = Some(target_cardinality);
    }
    pub fn unset_target_cardinality(&mut self) {
        self.target_cardinality = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body().is_some()
    }
    pub fn body(&self) -> Option<&AnnotationOnlyBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: AnnotationOnlyBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_annotations())
            .unwrap_or_default()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
