use crate::{
    load::ModuleLoader,
    model::{
        annotations::{Annotation, AnnotationOnlyBody, HasAnnotations},
        check::Validate,
        definitions::HasVariants,
        identifiers::{Identifier, IdentifierReference},
        modules::Module,
        References, Span,
    },
    store::ModuleStore,
};
use sdml_errors::diagnostics::functions::IdentifierCaseConvention;
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `union_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UnionDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Box<Span>>,
    name: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<UnionBody>,
}

/// Corresponds to the grammar rule `union_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UnionBody {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Box<Span>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    annotations: Vec<Annotation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    variants: Vec<TypeVariant>, // assert!(!variants.is_empty());
}

/// Corresponds to the grammar rule `type_variant`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TypeVariant {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Box<Span>>,
    name_reference: IdentifierReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    rename: Option<Identifier>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Type Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(UnionDef);

impl_has_optional_body_for!(UnionDef, UnionBody);

impl_has_source_span_for!(UnionDef);

impl_maybe_incomplete_for!(UnionDef; exists body);

impl Validate for UnionDef {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.name
            .validate(top, loader, Some(IdentifierCaseConvention::TypeDefinition));
        if let Some(body) = &self.body {
            body.validate(top, cache, loader, check_constraints);
        }
    }
}

impl References for UnionDef {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl UnionDef {
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

    pub fn with_body(self, body: UnionBody) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(UnionBody);

impl_has_source_span_for!(UnionBody);

impl_has_variants_for!(UnionBody, TypeVariant);

impl_validate_for_annotations_and_variants!(UnionBody);

impl_annotation_builder!(UnionDef, optional body);

impl References for UnionBody {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.variants
            .iter()
            .for_each(|v| v.referenced_annotations(names));
    }
}

impl UnionBody {
    pub fn with_variants(self, variants: Vec<TypeVariant>) -> Self {
        Self { variants, ..self }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_name_reference_for!(TypeVariant);

impl_has_optional_body_for!(TypeVariant);

impl_has_source_span_for!(TypeVariant);

impl_annotation_builder!(TypeVariant, optional body);

impl Validate for TypeVariant {
    fn validate(
        &self,
        top: &Module,
        cache: &impl ModuleStore,
        loader: &impl ModuleLoader,
        check_constraints: bool,
    ) {
        self.name_reference.validate(top, loader);
        if let Some(rename) = &self.rename {
            rename.validate(top, loader, Some(IdentifierCaseConvention::TypeDefinition));
        }
        if let Some(body) = &self.body {
            body.validate(top, cache, loader, check_constraints);
        }
    }
}

impl References for TypeVariant {
    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl TypeVariant {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

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
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn with_rename(self, rename: Identifier) -> Self {
        Self {
            rename: Some(rename),
            ..self
        }
    }

    get_and_set!(pub name_reference, set_name_reference => IdentifierReference);

    get_and_set!(pub rename, set_rename, unset_rename => optional has_rename, Identifier);

    pub fn name(&self) -> &Identifier {
        if let Some(rename) = self.rename() {
            rename
        } else {
            match &self.name_reference {
                IdentifierReference::Identifier(name) => name,
                IdentifierReference::QualifiedIdentifier(name) => name.member(),
            }
        }
    }
}
