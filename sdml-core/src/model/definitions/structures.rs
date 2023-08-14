use crate::model::{
    Annotation, AnnotationProperty, ByValueMember, Constraint, Identifier, IdentifierReference,
    ModelElement, Span,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `structure_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<StructureBody>,
}

/// Corresponds to the grammar rule `structure_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>,
    groups: Vec<StructureGroup>,
}

/// Corresponds to the grammar rule `structure_group`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureGroup {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>, // assert!(!members.is_empty());
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

impl ModelElement for StructureDef {
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
        self.body().is_some()
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

impl StructureDef {
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
    pub fn body(&self) -> Option<&StructureBody> {
        self.body.as_ref()
    }
    pub fn set_body(&mut self, body: StructureBody) {
        self.body = Some(body);
    }
    pub fn unset_body(&mut self) {
        self.body = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl StructureBody {
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

    pub fn has_members(&self) -> bool {
        !self.members.is_empty()
    }
    pub fn members_len(&self) -> usize {
        self.members.len()
    }
    pub fn members(&self) -> impl Iterator<Item = &ByValueMember> {
        self.members.iter()
    }
    pub fn members_mut(&mut self) -> impl Iterator<Item = &mut ByValueMember> {
        self.members.iter_mut()
    }
    pub fn add_to_members(&mut self, value: ByValueMember) {
        self.members.push(value)
    }
    pub fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = ByValueMember>,
    {
        self.members.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_groups(&self) -> bool {
        !self.groups.is_empty()
    }
    pub fn groups_len(&self) -> usize {
        self.groups.len()
    }
    pub fn groups(&self) -> impl Iterator<Item = &StructureGroup> {
        self.groups.iter()
    }
    pub fn groups_mut(&mut self) -> impl Iterator<Item = &mut StructureGroup> {
        self.groups.iter_mut()
    }
    pub fn add_to_groups(&mut self, value: StructureGroup) {
        self.groups.push(value)
    }
    pub fn extend_groups<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = StructureGroup>,
    {
        self.groups.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn flat_members(&self) -> impl Iterator<Item = &ByValueMember> {
        self.members()
            .chain(self.groups().flat_map(|g| g.members()))
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete()) && self.groups().all(|m| m.is_complete())
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.flat_members()
            .flat_map(|m| m.referenced_annotations())
            .collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.flat_members()
            .flat_map(|m| m.referenced_types())
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl StructureGroup {
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

    pub fn has_members(&self) -> bool {
        !self.members.is_empty()
    }
    pub fn members_len(&self) -> usize {
        self.members.len()
    }
    pub fn members(&self) -> impl Iterator<Item = &ByValueMember> {
        self.members.iter()
    }
    pub fn members_mut(&mut self) -> impl Iterator<Item = &mut ByValueMember> {
        self.members.iter_mut()
    }
    pub fn add_to_members(&mut self, value: ByValueMember) {
        self.members.push(value)
    }
    pub fn extend_members<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = ByValueMember>,
    {
        self.members.extend(extension)
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete())
    }

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.annotation_properties()
            .map(|p| p.name())
            .chain(self.annotation_properties().map(|a| a.name()))
            .collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.members().flat_map(|m| m.referenced_types()).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
