use crate::{
    load::ModuleLoader,
    model::{
        annotations::{AnnotationBuilder, AnnotationOnlyBody, AnnotationProperty, HasAnnotations},
        check::{MaybeIncomplete, Validate},
        identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
        modules::Module,
        values::Value,
        HasBody, HasName, HasSourceSpan, References, Span,
    },
    stdlib,
    store::ModuleStore,
};
use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
use std::collections::BTreeSet;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Definitions ❱ RDF
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `rdf_class_def` and `rdf_property_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RdfDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    body: AnnotationOnlyBody,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Definitions ❱ RDF
// ------------------------------------------------------------------------------------------------

impl HasName for RdfDef {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn set_name(&mut self, name: Identifier) {
        self.name = name;
    }
}

impl HasBody for RdfDef {
    type Body = AnnotationOnlyBody;

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

impl HasSourceSpan for RdfDef {
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

impl References for RdfDef {
    fn referenced_annotations<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.body.referenced_annotations(names);
    }

    fn referenced_types<'a>(&'a self, names: &mut BTreeSet<&'a IdentifierReference>) {
        self.body.referenced_types(names);
    }
}

impl AnnotationBuilder for RdfDef {
    fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        Self: Sized,
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        let mut self_mut = self;
        self_mut
            .body
            .add_to_annotations(AnnotationProperty::new(predicate.into(), value.into()));
        self_mut
    }
}

impl MaybeIncomplete for RdfDef {
    fn is_incomplete(&self, _: &Module, _: &impl ModuleStore) -> bool {
        false
    }
}

impl Validate for RdfDef {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.name
            .validate(top, loader, Some(IdentifierCaseConvention::RdfDefinition));
        self.body.validate(top, cache, loader, check_constraints);
    }
}

impl RdfDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: Default::default(),
        }
    }

    pub fn class(name: Identifier) -> Self {
        Self::new(name).with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::CLASS),
        ))
    }

    pub fn is_class(&self) -> bool {
        self.body.has_rdf_type(
            &QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::CLASS),
            )
            .into(),
        )
    }

    pub fn datatype(name: Identifier) -> Self {
        Self::new(name).with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::DATATYPE),
        ))
    }

    pub fn is_datatype(&self) -> bool {
        self.body.has_rdf_type(
            &QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::DATATYPE),
            )
            .into(),
        )
    }

    pub fn property(name: Identifier) -> Self {
        Self::new(name).with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdf::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdf::PROPERTY),
        ))
    }

    pub fn is_property(&self) -> bool {
        self.body.has_rdf_type(
            &QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdf::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdf::PROPERTY),
            )
            .into(),
        )
    }

    pub fn individual(name: Identifier) -> Self {
        Self::new(name)
    }
}
