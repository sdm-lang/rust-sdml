/*!
Provide the Rust types that implement *annotation*-related components of the SDML Grammar.
*/

use crate::{
    config,
    load::ModuleLoader,
    model::{
        check::Validate,
        constraints::Constraint,
        definitions::is_restriction_facet_name,
        identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
        modules::Module,
        values::{LanguageString, LanguageTag, Value},
        HasName, HasNameReference, References, Span,
    },
    stdlib,
    store::ModuleStore,
};
use std::{collections::BTreeSet, fmt::Debug};
use tracing::trace;
use url::Url;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::HasSourceSpan;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasAnnotations {
    fn with_annotations<I>(self, annotations: I) -> Self
    where
        I: IntoIterator<Item = Annotation>,
        Self: Sized,
    {
        let mut self_mut = self;
        self_mut.extend_annotations(annotations);
        self_mut
    }

    fn has_annotations(&self) -> bool;

    fn annotation_count(&self) -> usize;

    fn annotations(&self) -> impl Iterator<Item = &Annotation>;

    fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation>;

    fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>;

    fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>;

    fn has_annotation_properties(&self) -> bool {
        self.annotations().any(|a| a.is_annotation_property())
    }

    fn annotation_properties(&self) -> impl Iterator<Item = &AnnotationProperty> {
        self.annotations()
            .filter_map(|a| a.as_annotation_property())
    }

    fn has_rdf_type(&self, type_id: &IdentifierReference) -> bool {
        self.rdf_types().any(|id| id == type_id)
    }

    fn rdf_types(&self) -> impl Iterator<Item = &IdentifierReference> {
        self.annotation_properties()
            .filter(|ann| ann.name_reference() == "rdf:type")
            .filter_map(|ann| ann.value().as_reference())
    }

    fn preferred_label(&self) -> impl Iterator<Item = &LanguageString> {
        self.annotation_properties()
            .filter(|ann| ann.name_reference() == "skos:prefLabel")
            .filter_map(|ann| ann.value().as_string())
    }

    fn alternate_labels(&self) -> impl Iterator<Item = &LanguageString> {
        self.annotation_properties()
            .filter(|ann| ann.name_reference() == "skos:altLabel")
            .filter_map(|ann| ann.value().as_string())
    }

    fn descriptions(&self) -> impl Iterator<Item = &LanguageString> {
        self.annotation_properties()
            .filter(|ann| ann.name_reference() == "dc:description")
            .filter_map(|ann| ann.value().as_string())
    }

    fn skos_definitions(&self) -> impl Iterator<Item = &LanguageString> {
        self.annotation_properties()
            .filter(|ann| ann.name_reference() == "skos:definition")
            .filter_map(|ann| ann.value().as_string())
    }

    fn has_constraints(&self) -> bool {
        self.annotations().any(|a| a.is_constraint())
    }

    fn annotation_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.annotations().filter_map(|a| a.as_constraint())
    }
}

pub trait AnnotationBuilder {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>;

    fn with_type<I>(self, name: I) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdf::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdf::TYPE),
            ),
            Value::from(name.into()),
        )
    }

    fn with_super_class<I>(self, name: I) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::SUB_CLASS_OF),
            ),
            Value::from(name.into()),
        )
    }

    fn with_equivalent_class<I>(self, name: I) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::owl::MODULE_NAME),
                Identifier::new_unchecked(stdlib::owl::EQUIVALENT_CLASS),
            ),
            Value::from(name.into()),
        )
    }

    fn with_super_property<I>(self, name: I) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::SUB_PROPERTY_OF),
            ),
            Value::from(name.into()),
        )
    }

    fn with_domain<I>(self, name: I) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::DOMAIN),
            ),
            Value::from(name.into()),
        )
    }

    fn with_comment<S>(self, comment: S) -> Self
    where
        Self: Sized,
        S: Into<LanguageString>,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::COMMENT),
            ),
            comment.into(),
        )
    }

    fn with_label<S>(self, label: S) -> Self
    where
        Self: Sized,
        S: Into<LanguageString>,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::LABEL),
            ),
            Value::from(label.into()),
        )
    }

    fn with_see_also_str(self, resource: &str) -> Self
    where
        Self: Sized,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::SEE_ALSO),
            ),
            Value::from(Url::parse(resource).unwrap()),
        )
    }

    fn with_see_also(self, resource: Url) -> Self
    where
        Self: Sized,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::SEE_ALSO),
            ),
            Value::from(resource),
        )
    }

    fn with_see_also_ref<I>(self, resource: I) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::SEE_ALSO),
            ),
            Value::from(resource.into()),
        )
    }

    fn with_is_defined_by(self, resource: Url) -> Self
    where
        Self: Sized,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::IS_DEFINED_BY),
            ),
            Value::from(resource),
        )
    }

    fn with_is_defined_by_str(self, resource: &str) -> Self
    where
        Self: Sized,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::IS_DEFINED_BY),
            ),
            Value::from(Url::parse(resource).unwrap()),
        )
    }

    fn with_is_defined_by_ref<I>(self, resource: I) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::IS_DEFINED_BY),
            ),
            Value::from(resource.into()),
        )
    }

    fn with_range<I>(self, name: I) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
    {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::RANGE),
            ),
            Value::from(name.into()),
        )
    }
}

impl<A: HasAnnotations> AnnotationBuilder for A {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        let mut self_mut = self;
        self_mut.add_to_annotations(AnnotationProperty::new(predicate.into(), value.into()));
        self_mut
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Concrete
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `annotation`.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)] // TODO: why is this reported as an issue?
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Annotation {
    Property(AnnotationProperty),
    Constraint(Constraint),
}

/// Corresponds to the grammar rule `annotation_property`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AnnotationProperty {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name_reference: IdentifierReference,
    value: Value,
}

/// Corresponds to the grammar rule `annotation_only_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AnnotationOnlyBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    annotations: Vec<Annotation>, // assert!(!annotations.is_empty());
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn preferred_type_label<T: HasAnnotations + HasName>(
    element: T,
    _for_language: Option<LanguageTag>,
) -> String {
    let labels: Vec<&LanguageString> = element.preferred_label().collect();

    // TODO: match by language

    if labels.is_empty() {
        element.name().to_type_label()
    } else {
        element.name().to_string()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotation
// ------------------------------------------------------------------------------------------------

impl From<AnnotationProperty> for Annotation {
    fn from(value: AnnotationProperty) -> Self {
        Self::Property(value)
    }
}

impl From<Constraint> for Annotation {
    fn from(value: Constraint) -> Self {
        Self::Constraint(value)
    }
}

impl HasSourceSpan for Annotation {
    #[inline]
    fn with_source_span(self, span: Span) -> Self {
        match self {
            Self::Property(v) => Self::Property(v.with_source_span(span)),
            Self::Constraint(v) => Self::Constraint(v.with_source_span(span)),
        }
    }
    #[inline]
    fn source_span(&self) -> Option<&Span> {
        match self {
            Self::Property(v) => v.source_span(),
            Self::Constraint(v) => v.source_span(),
        }
    }
    #[inline]
    fn set_source_span(&mut self, span: Span) {
        match self {
            Self::Property(v) => v.set_source_span(span),
            Self::Constraint(v) => v.set_source_span(span),
        }
    }
    #[inline]
    fn unset_source_span(&mut self) {
        match self {
            Self::Property(v) => v.unset_source_span(),
            Self::Constraint(v) => v.unset_source_span(),
        }
    }
}

impl References for Annotation {}

impl Validate for Annotation {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        trace!("Annotation::is_valid");
        match (self, check_constraints) {
            (Annotation::Property(v), _) => v.validate(top, cache, loader, check_constraints),
            (Annotation::Constraint(v), true) => v.validate(top, cache, loader, check_constraints),
            _ => {}
        };
    }
}

impl Annotation {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    pub const fn is_annotation_property(&self) -> bool {
        matches!(self, Self::Property(_))
    }
    pub const fn as_annotation_property(&self) -> Option<&AnnotationProperty> {
        match self {
            Self::Property(v) => Some(v),
            _ => None,
        }
    }
    pub const fn is_constraint(&self) -> bool {
        matches!(self, Self::Constraint(_))
    }
    pub const fn as_constraint(&self) -> Option<&Constraint> {
        match self {
            Self::Constraint(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ AnnotationProperty
// ------------------------------------------------------------------------------------------------

impl HasSourceSpan for AnnotationProperty {
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

impl HasNameReference for AnnotationProperty {
    fn name_reference(&self) -> &IdentifierReference {
        &self.name_reference
    }
    fn set_name_reference(&mut self, name: IdentifierReference) {
        self.name_reference = name;
    }
}

impl Validate for AnnotationProperty {
    fn validate(&self, _top: &Module, _cache: &impl ModuleStore, _: &impl ModuleLoader, _: bool) {
        trace!("AnnotationProperty::is_valid -- missing type/value conformance");
        // TODO: check value/type conformance
        // 1. Lookup property
        // 2. Get property range
        // 3. check::validate_value(self.value, range, ...)
    }
}

impl AnnotationProperty {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<I, V>(name_reference: I, value: V) -> Self
    where
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        Self {
            span: None,
            name_reference: name_reference.into(),
            value: value.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub const fn value(&self) -> &Value {
        &self.value
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }

    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn is_rdf_type(&self) -> bool {
        self.name_reference
            == IdentifierReference::from(QualifiedIdentifier::new_unchecked("rdf", "type"))
    }

    #[inline(always)]
    pub fn is_stdlib_property(&self) -> bool {
        if let IdentifierReference::QualifiedIdentifier(name) = self.name_reference() {
            config::is_library_module(name.module())
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn is_datatype_facet(&self) -> bool {
        if let IdentifierReference::QualifiedIdentifier(name) = self.name_reference() {
            name.module().as_ref() == stdlib::xsd::MODULE_NAME
                && is_restriction_facet_name(name.member())
        } else {
            false
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ AnnotationOnlyBody
// ------------------------------------------------------------------------------------------------

impl From<Vec<Annotation>> for AnnotationOnlyBody {
    fn from(annotations: Vec<Annotation>) -> Self {
        Self {
            span: Default::default(),
            annotations,
        }
    }
}

impl From<AnnotationOnlyBody> for Vec<Annotation> {
    fn from(value: AnnotationOnlyBody) -> Self {
        value.annotations
    }
}

impl FromIterator<Annotation> for AnnotationOnlyBody {
    fn from_iter<T: IntoIterator<Item = Annotation>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl HasSourceSpan for AnnotationOnlyBody {
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

impl HasAnnotations for AnnotationOnlyBody {
    fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }

    fn annotation_count(&self) -> usize {
        self.annotations.len()
    }

    fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }

    fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }

    fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>,
    {
        self.annotations.push(value.into())
    }

    fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.annotations.extend(extension)
    }
}

impl References for AnnotationOnlyBody {
    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        names.extend(self.annotation_properties().map(|ann| ann.name_reference()));
    }
}

impl Validate for AnnotationOnlyBody {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        trace!("AnnotationOnlyBody::is_valid");
        self.annotations()
            .for_each(|ann| ann.validate(top, cache, loader, check_constraints));
    }
}
