use crate::model::{
    Annotation, AnnotationOnlyBody, AnnotationProperty, Constraint, Identifier,
    IdentifierReference, ModelElement, Span,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `enum_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EnumDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<EnumBody>,
}

/// Corresponds to the grammar rule `enum_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EnumBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    variants: Vec<ValueVariant>, // assert!(!variants.is_empty());
}

/// Corresponds to the grammar rule `enum_variant`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ValueVariant {
    span: Option<Span>,
    name: Identifier,
    value: u32,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

impl ModelElement for EnumDef {
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
        Default::default()
    }
}

impl EnumDef {
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
    pub fn body(&self) -> Option<&EnumBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: EnumBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl EnumBody {
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

    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }
    pub fn variants_len(&self) -> usize {
        self.variants.len()
    }
    pub fn variants(&self) -> impl Iterator<Item = &ValueVariant> {
        self.variants.iter()
    }
    pub fn variants_mut(&mut self) -> impl Iterator<Item = &mut ValueVariant> {
        self.variants.iter_mut()
    }
    pub fn add_to_variants(&mut self, value: ValueVariant) {
        self.variants.push(value)
    }
    pub fn extend_variants<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = ValueVariant>,
    {
        self.variants.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        let mut body: HashSet<&IdentifierReference> = self
            .annotations()
            .filter_map(|a| {
                if let Annotation::Property(aprop) = a {
                    Some(aprop.name())
                } else {
                    None
                }
            })
            .collect();
        let variants: HashSet<&IdentifierReference> = self
            .variants()
            .flat_map(|v| v.referenced_annotations())
            .collect();
        body.extend(variants);
        body
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        Default::default()
    }
}

// ------------------------------------------------------------------------------------------------

impl ValueVariant {
    pub fn new(name: Identifier, value: u32) -> Self {
        Self {
            span: None,
            name,
            value,
            body: None,
        }
    }

    pub fn new_with(name: Identifier, value: u32, body: AnnotationOnlyBody) -> Self {
        Self {
            span: None,
            name,
            value,
            body: Some(body),
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

    pub fn value(&self) -> u32 {
        self.value
    }
    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_body(&self) -> bool {
        self.body.is_some()
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
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
