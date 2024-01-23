use crate::cache::ModuleCache;
use crate::error::Error;
use crate::model::{
    check::Validate, constraints::Constraint, identifiers::IdentifierReference, modules::Module,
    values::Value, HasNameReference, Span,
};
use std::{collections::HashSet, fmt::Debug};
use crate::stdlib;
use crate::model::values::{LanguageString, LanguageTag};
use crate::model::{HasName, References};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tracing::trace;
use url::Url;

use super::identifiers::{QualifiedIdentifier, Identifier};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasAnnotations {
    fn has_annotations(&self) -> bool;

    fn annotations_len(&self) -> usize;

    fn annotations(&self) -> Box<dyn Iterator<Item = &Annotation> + '_>;

    fn annotations_mut(&mut self) -> Box<dyn Iterator<Item = &mut Annotation> + '_>;

    fn add_to_annotations<I>(&mut self, value: I)
    where
        I: Into<Annotation>;

    fn extend_annotations<I>(&mut self, extension: I)
    where
        I: IntoIterator<Item = Annotation>;

    fn has_annotation_properties(&self) -> bool {
        self.annotations().any(|a| a.is_annotation_property())
    }

    fn annotation_properties(&self) -> Box<dyn Iterator<Item = &AnnotationProperty> + '_> {
        Box::new(
            self.annotations()
                .filter_map(|a| a.as_annotation_property()),
        )
    }

    fn preferred_label(&self) -> Box<dyn Iterator<Item = &LanguageString> + '_> {
        Box::new(
            self.annotation_properties()
                .filter(|ann| ann.name_reference() == "skos:prefLabel")
                .filter_map(|ann| ann.value().as_string()),
        )
    }

    fn alternate_labels(&self) -> Box<dyn Iterator<Item = &LanguageString> + '_> {
        Box::new(
            self.annotation_properties()
                .filter(|ann| ann.name_reference() == "skos:altLabel")
                .filter_map(|ann| ann.value().as_string()),
        )
    }

    fn descriptions(&self) -> Box<dyn Iterator<Item = &LanguageString> + '_> {
        Box::new(
            self.annotation_properties()
                .filter(|ann| ann.name_reference() == "dc:description")
                .filter_map(|ann| ann.value().as_string()),
        )
    }

    fn definitions(&self) -> Box<dyn Iterator<Item = &LanguageString> + '_> {
        Box::new(
            self.annotation_properties()
                .filter(|ann| ann.name_reference() == "skos:definition")
                .filter_map(|ann| ann.value().as_string()),
        )
    }

    fn has_constraints(&self) -> bool {
        self.annotations().any(|a| a.is_constraint())
    }

    fn annotation_constraints<I>(&self) -> Box<dyn Iterator<Item = &Constraint> + '_> {
        Box::new(self.annotations().filter_map(|a| a.as_constraint()))
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
                Identifier::new_unchecked(stdlib::rdf::PROP_TYPE_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_SUB_CLASS_OF_NAME),
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
                Identifier::new_unchecked(stdlib::owl::PROP_EQUIVALENT_CLASS_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_SUB_PROPERTY_OF_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_DOMAIN_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_COMMENT_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_LABEL_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_SEE_ALSO_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_SEE_ALSO_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_SEE_ALSO_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_IS_DEFINED_BY_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_IS_DEFINED_BY_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_IS_DEFINED_BY_NAME),
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
                Identifier::new_unchecked(stdlib::rdfs::PROP_RANGE_NAME),
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
    span: Option<Span>,
    name_reference: IdentifierReference,
    value: Value,
}

/// Corresponds to the grammar rule `annotation_only_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AnnotationOnlyBody {
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
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

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
    fn is_complete(&self, top: &Module, cache: &ModuleCache) -> Result<bool, Error> {
        trace!("Annotation::is_complete");
        match self {
            Annotation::Property(v) => v.is_complete(top, cache),
            Annotation::Constraint(v) => v.is_complete(top, cache),
        }
    }

    fn is_valid(
        &self,
        check_constraints: bool,
        top: &Module,
        cache: &ModuleCache,
    ) -> Result<bool, Error> {
        trace!("Annotation::is_valid");
        match (self, check_constraints) {
            (Annotation::Property(v), _) => v.is_valid(check_constraints, top, cache),
            (Annotation::Constraint(v), true) => v.is_valid(check_constraints, top, cache),
            _ => Ok(true),
        }
    }
}

impl Annotation {
    // --------------------------------------------------------------------------------------------
    // Variants
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
    fn is_complete(&self, _top: &Module, _cache: &ModuleCache) -> Result<bool, Error> {
        trace!("AnnotationProperty::is_complete");
        Ok(true)
    }

    fn is_valid(
        &self,
        _check_constraints: bool,
        _top: &Module,
        _cache: &ModuleCache,
    ) -> Result<bool, Error> {
        trace!("AnnotationProperty::is_valid -- missing type/value conformance");
        // TODO: ensure type/value conformance.
        Ok(true)
    }
}

impl AnnotationProperty {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name_reference: IdentifierReference, value: Value) -> Self {
        Self {
            span: None,
            name_reference,
            value,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub value, set_value => Value);

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
            name.module().as_ref() == stdlib::xsd::MODULE_NAME &&
                stdlib::xsd::is_constraining_facet(name.member())
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
    fn is_complete(&self, top: &Module, cache: &ModuleCache) -> Result<bool, Error> {
        trace!("AnnotationOnlyBody::is_complete");
        let failed: Result<Vec<bool>, Error> = self
            .annotations()
            .map(|ann| ann.is_complete(top, cache))
            .collect();
        Ok(failed?.iter().all(|b| *b))
    }

    fn is_valid(
        &self,
        check_constraints: bool,
        top: &Module,
        cache: &ModuleCache,
    ) -> Result<bool, Error> {
        trace!("AnnotationOnlyBody::is_valid");
        let failed: Result<Vec<bool>, Error> = self
            .annotations()
            .map(|ann| ann.is_valid(check_constraints, top, cache))
            .collect();
        Ok(failed?.iter().all(|b| *b))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
