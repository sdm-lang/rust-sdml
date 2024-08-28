use crate::cache::ModuleStore;
use crate::load::ModuleLoader;
use crate::model::values::{LanguageString, LanguageTag};
use crate::model::{
    check::Validate,
    constraints::Constraint,
    identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
    modules::Module,
    values::Value,
    HasNameReference, Span,
};
use crate::model::{HasName, References};
use crate::stdlib;
use std::{collections::HashSet, fmt::Debug};
use tracing::trace;
use url::Url;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasAnnotations {
    fn has_annotations(&self) -> bool;

    fn annotations_len(&self) -> usize;

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

    fn definitions(&self) -> impl Iterator<Item = &LanguageString> {
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
// Implementations ❱ Annotations
// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(Annotation => variants Property, Constraint);

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
    // Annotation :: Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Property (AnnotationProperty) => is_annotation_property, as_annotation_property);

    is_as_variant!(Constraint (Constraint) => is_constraint, as_constraint);
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Annotations ❱ Annotation Properties
// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(AnnotationProperty);

impl_has_name_reference_for!(AnnotationProperty);

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
    // AnnotationProperty :: Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name_reference: IdentifierReference, value: Value) -> Self {
        Self {
            span: None,
            name_reference,
            value,
        }
    }

    // --------------------------------------------------------------------------------------------
    // AnnotationProperty :: Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub value, set_value => Value);

    // --------------------------------------------------------------------------------------------
    // AnnotationProperty :: Helpers
    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn is_stdlib_property(&self) -> bool {
        if let IdentifierReference::QualifiedIdentifier(name) = self.name_reference() {
            stdlib::is_library_module(name.module())
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn is_datatype_facet(&self) -> bool {
        if let IdentifierReference::QualifiedIdentifier(name) = self.name_reference() {
            name.module().as_ref() == stdlib::xsd::MODULE_NAME
                && stdlib::xsd::is_constraining_facet(name.member())
        } else {
            false
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(AnnotationOnlyBody);

impl_has_annotations_for!(AnnotationOnlyBody);

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

impl References for AnnotationOnlyBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
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
