use crate::{
    cache::ModuleCache,
    model::{
        annotations::{AnnotationOnlyBody, AnnotationProperty, HasAnnotations},
        check::Validate,
        identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
        modules::Module,
        values::{LanguageString, Value},
        HasBody, Span,
    },
    stdlib,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `rdf_class_def` and `rdf_property_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RdfDef {
    span: Option<Span>,
    name: Identifier,
    body: AnnotationOnlyBody,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(RdfDef);

impl_has_body_for!(RdfDef, AnnotationOnlyBody);

impl_has_source_span_for!(RdfDef);

impl_references_for!(RdfDef => delegate body);

impl Validate for RdfDef {
    fn is_complete(&self, _: &Module, _: &ModuleCache) -> Result<bool, crate::error::Error> {
        info!("RdfDef::is_complete true by definition");
        Ok(true)
    }

    fn is_valid(&self, _: bool, _: &Module, _: &ModuleCache) -> Result<bool, crate::error::Error> {
        info!("RdfDef::is_valid true-enough by definition");
        Ok(true)
    }
}

impl RdfDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: Default::default(),
        }
    }

    pub fn class(name: Identifier) -> Self {
        let new_self = Self::new(name);
        new_self.with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::CLASS_CLASS_NAME),
        ))
    }

    pub fn datatype(name: Identifier) -> Self {
        let new_self = Self::new(name);
        new_self.with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdfs::CLASS_DATATYPE_NAME),
        ))
    }

    pub fn property(name: Identifier) -> Self {
        let new_self = Self::new(name);
        new_self.with_type(QualifiedIdentifier::new(
            Identifier::new_unchecked(stdlib::rdf::MODULE_NAME),
            Identifier::new_unchecked(stdlib::rdf::CLASS_PROPERTY_NAME),
        ))
    }

    pub fn individual(name: Identifier) -> Self {
        Self::new(name)
    }

    // --------------------------------------------------------------------------------------------
    // Builder Functions
    // --------------------------------------------------------------------------------------------

    pub fn with_predicate<I, V>(self, predicate: I, value: V) -> Self
    where
        I: Into<IdentifierReference>,
        V: Into<Value>,
    {
        let mut self_mut = self;
        self_mut
            .body_mut()
            .add_to_annotations(AnnotationProperty::new(predicate.into(), value.into()));
        self_mut
    }

    pub fn with_type<I>(self, name: I) -> Self
    where
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

    pub fn with_super_class<I>(self, name: I) -> Self
    where
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

    pub fn with_equivalent_class<I>(self, name: I) -> Self
    where
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

    pub fn with_super_property<I>(self, name: I) -> Self
    where
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

    pub fn with_domain<I>(self, name: I) -> Self
    where
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

    pub fn with_comment<S>(self, comment: S) -> Self
    where
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

    pub fn with_label<S>(self, label: S) -> Self
    where
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

    pub fn with_see_also_str(self, resource: &str) -> Self {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::PROP_SEE_ALSO_NAME),
            ),
            Value::from(Url::parse(resource).unwrap()),
        )
    }

    pub fn with_see_also(self, resource: Url) -> Self {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::PROP_SEE_ALSO_NAME),
            ),
            Value::from(resource),
        )
    }

    pub fn with_see_also_ref<I>(self, resource: I) -> Self
    where
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

    pub fn with_is_defined_by(self, resource: Url) -> Self {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::PROP_IS_DEFINED_BY_NAME),
            ),
            Value::from(resource),
        )
    }

    pub fn with_is_defined_by_str(self, resource: &str) -> Self {
        self.with_predicate(
            QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdfs::PROP_IS_DEFINED_BY_NAME),
            ),
            Value::from(Url::parse(resource).unwrap()),
        )
    }

    pub fn with_is_defined_by_ref<I>(self, resource: I) -> Self
    where
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

    pub fn with_range<I>(self, name: I) -> Self
    where
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
