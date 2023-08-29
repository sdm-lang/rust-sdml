use crate::{
    error::Error,
    model::{
        annotations::{Annotation, AnnotationOnlyBody},
        check::Validate,
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
// Public Types ❱ Type Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `union_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UnionDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<UnionBody>,
}

/// Corresponds to the grammar rule `union_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UnionBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    variants: Vec<TypeVariant>, // assert!(!variants.is_empty());
}

/// Corresponds to the grammar rule `type_variant`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TypeVariant {
    span: Option<Span>,
    name_reference: IdentifierReference,
    rename: Option<Identifier>,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(UnionDef);

impl_has_optional_body_for!(UnionDef, UnionBody);

impl_has_source_span_for!(UnionDef);

impl_validate_for!(UnionDef => delegate optional body, false, true);

impl References for UnionDef {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl UnionDef {
    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(UnionBody);

impl_has_source_span_for!(UnionBody);

impl_has_variants_for!(UnionBody, TypeVariant);

impl References for UnionBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.variants
            .iter()
            .for_each(|v| v.referenced_annotations(names));
    }
}

impl Validate for UnionBody {
    fn is_complete(&self, _top: &Module) -> Result<bool, Error> {
        todo!()
    }

    fn is_valid(&self, _check_constraints: bool, _top: &Module) -> Result<bool, Error> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_name_reference_for!(TypeVariant);

impl_has_optional_body_for!(TypeVariant);

impl_has_source_span_for!(TypeVariant);

impl References for TypeVariant {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl TypeVariant {
    pub fn new(name_reference: IdentifierReference) -> Self {
        Self {
            span: None,
            name_reference,
            rename: None,
            body: None,
        }
    }

    pub fn new_with(name_reference: IdentifierReference, body: AnnotationOnlyBody) -> Self {
        Self {
            span: None,
            name_reference,
            rename: None,
            body: Some(body),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_rename(self, rename: Identifier) -> Self {
        Self {
            rename: Some(rename),
            ..self
        }
    }

    pub fn has_rename(&self) -> bool {
        self.rename.is_some()
    }

    pub fn rename(&self) -> Option<&Identifier> {
        self.rename.as_ref()
    }

    pub fn set_rename(&mut self, rename: Identifier) {
        self.rename = Some(rename);
    }

    pub fn unset_rename(&mut self) {
        self.rename = None;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
