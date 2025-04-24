/*!
Provide the Rust types that implement *member*-related components of the SDML Grammar.
*/

use crate::{
    load::ModuleLoader,
    model::{
        annotations::{AnnotationBuilder, AnnotationOnlyBody, AnnotationProperty, HasAnnotations},
        check::{find_definition, MaybeIncomplete, Validate},
        definitions::Definition,
        identifiers::{Identifier, IdentifierReference},
        modules::Module,
        values::Value,
        HasName, HasOptionalBody, HasSourceSpan, References, Span,
    },
    store::ModuleStore,
};
use sdml_errors::diagnostics::functions::{
    property_reference_not_property, type_definition_not_found, IdentifierCaseConvention,
};
use std::{collections::BTreeSet, fmt::Debug};
use tracing::error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Member
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rules `member` and `entity_identity`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Member {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    kind: MemberKind,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum MemberKind {
    Reference(IdentifierReference),
    Definition(MemberDef),
}

/// Corresponds to the definition component within grammar rule `by_reference_member`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MemberDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    target_cardinality: Cardinality,
    target_type: TypeReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ Member
// ------------------------------------------------------------------------------------------------

impl<T> From<T> for Member
where
    T: Into<MemberKind>,
{
    fn from(kind: T) -> Self {
        Self {
            span: Default::default(),
            kind: kind.into(),
        }
    }
}

impl HasSourceSpan for Member {
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

impl MaybeIncomplete for Member {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        match &self.kind {
            MemberKind::Reference(name) => {
                if let Some(defn) = find_definition(name, top, cache) {
                    if matches!(defn, Definition::Property(_)) {
                        defn.is_incomplete(top, cache)
                    } else {
                        error!("Member property reference not a property");
                        false
                    }
                } else {
                    error!("Member property reference not found");
                    false
                }
            }
            MemberKind::Definition(v) => v.is_incomplete(top, cache),
        }
    }
}

impl Validate for Member {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        match &self.kind {
            MemberKind::Reference(name) => {
                if let Some(defn) = find_definition(name, top, cache) {
                    if !matches!(defn, Definition::Property(_)) {
                        error!("Member property reference not a property");
                        loader
                            .report(&property_reference_not_property(
                                top.file_id().copied().unwrap_or_default(),
                                name.source_span().map(|span| span.byte_range()),
                                name,
                            ))
                            .unwrap()
                    }
                } else {
                    error!("Member property reference not found");
                    loader
                        .report(&type_definition_not_found(
                            top.file_id().copied().unwrap_or_default(),
                            name.source_span().map(|span| span.byte_range()),
                            name,
                        ))
                        .unwrap()
                }
            }
            MemberKind::Definition(v) => {
                v.validate(top, cache, loader, check_constraints);
            }
        }
        validate_is_incomplete_named(self, self.name(), top, cache, loader);
    }
}

impl References for Member {
    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        match &self.kind {
            MemberKind::Reference(v) => {
                names.insert(v);
            }
            MemberKind::Definition(v) => v.referenced_annotations(names),
        }
    }

    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        match &self.kind {
            MemberKind::Reference(v) => {
                names.insert(v);
            }
            MemberKind::Definition(v) => v.referenced_types(names),
        }
    }
}

impl Member {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new_reference(in_property: IdentifierReference) -> Self {
        Self {
            span: None,
            kind: MemberKind::Reference(in_property),
        }
    }

    pub const fn new_definition(definition: MemberDef) -> Self {
        Self {
            span: None,
            kind: MemberKind::Definition(definition),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn kind(&self) -> &MemberKind {
        &self.kind
    }

    #[inline(always)]
    pub const fn is_definition(&self) -> bool {
        self.kind.is_definition()
    }

    #[inline(always)]
    pub const fn as_definition(&self) -> Option<&MemberDef> {
        self.kind.as_definition()
    }

    #[inline(always)]
    pub const fn is_property_reference(&self) -> bool {
        self.kind.is_property_reference()
    }

    #[inline(always)]
    pub const fn as_property_reference(&self) -> Option<&IdentifierReference> {
        self.kind.as_property_reference()
    }

    // --------------------------------------------------------------------------------------------
    // Delegated
    // --------------------------------------------------------------------------------------------

    pub fn name(&self) -> &Identifier {
        match self.kind() {
            MemberKind::Reference(v) => v.member(),
            MemberKind::Definition(defn) => defn.name(),
        }
    }

    pub fn resolve_target_type(
        &self,
        module: &Module,
        cache: &impl ModuleStore,
    ) -> Option<TypeReference> {
        match self.kind() {
            MemberKind::Reference(v) => {
                if let Some(Definition::Property(property)) = cache.resolve_or_in(v, module.name())
                {
                    Some(property.member_def().target_type().clone())
                } else {
                    None
                }
            }
            MemberKind::Definition(defn) => Some(defn.target_type().clone()),
        }
    }

    pub fn resolve_target_cardinality(
        &self,
        module: &Module,
        cache: &impl ModuleStore,
    ) -> Option<Cardinality> {
        match self.kind() {
            MemberKind::Reference(v) => {
                if let Some(Definition::Property(property)) = cache.resolve_or_in(v, module.name())
                {
                    Some(property.member_def().target_cardinality().clone())
                } else {
                    None
                }
            }
            MemberKind::Definition(defn) => Some(defn.target_cardinality().clone()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ MemberKind
// ------------------------------------------------------------------------------------------------

impl From<MemberDef> for MemberKind {
    fn from(value: MemberDef) -> Self {
        Self::Definition(value)
    }
}

impl From<IdentifierReference> for MemberKind {
    fn from(value: IdentifierReference) -> Self {
        Self::Reference(value)
    }
}

impl MemberKind {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_definition(&self) -> bool {
        matches!(self, Self::Definition(_))
    }

    pub const fn as_definition(&self) -> Option<&MemberDef> {
        match self {
            Self::Definition(v) => Some(v),
            _ => None,
        }
    }

    pub const fn is_property_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub const fn as_property_reference(&self) -> Option<&IdentifierReference> {
        match self {
            Self::Reference(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Members ❱ MemberDef
// ------------------------------------------------------------------------------------------------

impl HasName for MemberDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasCardinality for MemberDef {
    fn target_cardinality(&self) -> &Cardinality {
        &self.target_cardinality
    }

    fn set_target_cardinality(&mut self, target_cardinality: Cardinality) {
        self.target_cardinality = target_cardinality;
    }
}

impl HasOptionalBody for MemberDef {
    type Body = AnnotationOnlyBody;

    fn body(&self) -> Option<&Self::Body> {
        self.body.as_ref()
    }

    fn body_mut(&mut self) -> Option<&mut Self::Body> {
        self.body.as_mut()
    }

    fn set_body(&mut self, body: Self::Body) {
        self.body = Some(body);
    }

    fn unset_body(&mut self) {
        self.body = None;
    }
}

impl HasSourceSpan for MemberDef {
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

impl HasType for MemberDef {
    fn target_type(&self) -> &TypeReference {
        &self.target_type
    }

    fn set_target_type(&mut self, target_type: TypeReference) {
        self.target_type = target_type;
    }
}

impl AnnotationBuilder for MemberDef {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        let mut self_mut = self;
        if self_mut.body().is_none() {
            self_mut.set_body(AnnotationOnlyBody::default());
        }
        if let Some(ref mut inner) = self_mut.body {
            inner.add_to_annotations(AnnotationProperty::new(predicate.into(), value.into()));
        }
        self_mut
    }
}

impl References for MemberDef {
    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }

    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.target_type.referenced_types(names);
    }
}

impl MaybeIncomplete for MemberDef {
    fn is_incomplete(&self, top: &Module, cache: &impl ModuleStore) -> bool {
        self.target_type.is_incomplete(top, cache)
    }
}

impl Validate for MemberDef {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.name()
            .validate(top, loader, Some(IdentifierCaseConvention::Member));
        self.target_type()
            .validate(top, cache, loader, check_constraints);
        self.target_cardinality()
            .validate(top, cache, loader, check_constraints);
    }
}

impl MemberDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<T>(name: Identifier, target_type: T) -> Self
    where
        T: Into<TypeReference>,
    {
        Self {
            span: None,
            name,
            target_type: target_type.into(),
            target_cardinality: Cardinality::one(),
            body: None,
        }
    }

    pub const fn new_unknown(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            target_type: TypeReference::Unknown,
            target_cardinality: Cardinality::one(),
            body: None,
        }
    }

    pub fn with_target_type(self, target_type: TypeReference) -> Self {
        let mut self_mut = self;
        self_mut.target_type = target_type;
        self_mut
    }

    pub fn with_target_cardinality(self, target_cardinality: Cardinality) -> Self {
        let mut self_mut = self;
        self_mut.target_cardinality = target_cardinality;
        self_mut
    }

    pub fn with_body(self, body: AnnotationOnlyBody) -> Self {
        let mut self_mut = self;
        self_mut.body = Some(body);
        self_mut
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn target_type(&self) -> &TypeReference {
        &self.target_type
    }

    pub fn set_target_type(&mut self, target_type: TypeReference) {
        self.target_type = target_type;
    }

    pub const fn target_cardinality(&self) -> &Cardinality {
        &self.target_cardinality
    }

    pub fn set_target_cardinality(&mut self, target_cardinality: Cardinality) {
        self.target_cardinality = target_cardinality;
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod cardinality;
pub use cardinality::{
    Cardinality, CardinalityRange, HasCardinality, Ordering, PseudoSequenceType, Uniqueness,
    DEFAULT_CARDINALITY, DEFAULT_CARDINALITY_RANGE, TYPE_BAG_CARDINALITY, TYPE_LIST_CARDINALITY,
    TYPE_MAYBE_CARDINALITY, TYPE_ORDERED_SET_CARDINALITY, TYPE_SET_CARDINALITY,
};

mod types;
pub use types::{HasType, MappingType, TypeReference};

use super::check::validate_is_incomplete_named;
