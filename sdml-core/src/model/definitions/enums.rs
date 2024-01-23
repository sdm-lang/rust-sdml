use crate::{
    error::Error,
    model::{
        annotations::{Annotation, AnnotationOnlyBody, HasAnnotations},
        check::Validate,
        definitions::HasVariants,
        identifiers::{Identifier, IdentifierReference},
        modules::Module,
        References, Span,
    },
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
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(EnumDef);

impl_has_optional_body_for!(EnumDef, EnumBody);

impl_has_source_span_for!(EnumDef);

// TODO check that any equivalent class is a datatype.
impl_validate_for!(EnumDef => delegate optional body, false, true);

impl_annotation_builder!(EnumDef, optional body);

impl References for EnumDef {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
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
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(EnumBody);

impl_has_source_span_for!(EnumBody);

impl_has_variants_for!(EnumBody, ValueVariant);

impl References for EnumBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.variants
            .iter()
            .for_each(|v| v.referenced_annotations(names));
    }
}

impl_validate_for_annotations_and_variants!(EnumBody);

// ------------------------------------------------------------------------------------------------

impl_has_name_for!(ValueVariant);

impl_has_optional_body_for!(ValueVariant);

impl_has_source_span_for!(ValueVariant);

impl_annotation_builder!(ValueVariant, optional body);

impl Validate for ValueVariant {
    fn is_complete(&self, _: &Module, _: &crate::cache::ModuleCache) -> Result<bool, Error> {
        Ok(true)
    }

    fn is_valid(
        &self,
        _check_constraints: bool,
        _top: &Module,
        _cache: &crate::cache::ModuleCache,
    ) -> Result<bool, Error> {
        // TODO check values are valid for any specified type
        Ok(true)
    }
}

impl References for ValueVariant {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl ValueVariant {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
