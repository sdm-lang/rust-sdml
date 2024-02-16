use crate::cache::ModuleCache;
use crate::load::ModuleLoader;
use crate::model::check::{find_definition, Validate};
use crate::model::definitions::Definition;
use crate::model::modules::Module;
use crate::model::HasSourceSpan;
use crate::model::References;
use crate::model::{
    annotations::AnnotationOnlyBody,
    identifiers::{Identifier, IdentifierReference},
    Span,
};
use sdml_error::diagnostics::functions::{datatype_invalid_base_type, type_definition_not_found};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `data_type_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DatatypeDef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Span>,
    name: Identifier,
    opaque: bool,
    /// Corresponds to the grammar rule `data_type_base`.
    base_type: IdentifierReference,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(DatatypeDef);

impl_has_name_for!(DatatypeDef);

impl_has_optional_body_for!(DatatypeDef);

impl_maybe_invalid_for!(DatatypeDef; exists body);

impl_annotation_builder!(DatatypeDef, optional body);

impl Validate for DatatypeDef {
    fn validate(
        &self,
        top: &Module,
        cache: &ModuleCache,
        loader: &impl ModuleLoader,
        _check_constraints: bool,
    ) {
        if let Some(defn) = find_definition(self.base_type(), top, cache) {
            if let Definition::Datatype(_base) = defn {
                // TODO: check restriction annotations.
            } else if let Definition::Rdf(base) = defn {
                if !base.is_datatype() {
                    loader
                        .report(&datatype_invalid_base_type(
                            top.file_id().copied().unwrap_or_default(),
                            self.base_type()
                                .source_span()
                                .as_ref()
                                .map(|span| (*span).into()),
                            self.base_type(),
                        ))
                        .unwrap();
                }
                // TODO: check type and restrictions
            } else {
                loader
                    .report(&datatype_invalid_base_type(
                        top.file_id().copied().unwrap_or_default(),
                        self.base_type()
                            .source_span()
                            .as_ref()
                            .map(|span| (*span).into()),
                        self.base_type(),
                    ))
                    .unwrap();
            }
        } else {
            loader
                .report(&type_definition_not_found(
                    top.file_id().copied().unwrap_or_default(),
                    self.span.as_ref().map(|span| span.clone().into()),
                    self.base_type(),
                ))
                .unwrap();
        }
    }
}

impl References for DatatypeDef {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        names.insert(&self.base_type);
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.body
            .as_ref()
            .map(|b| b.referenced_annotations(names))
            .unwrap_or_default()
    }
}

impl DatatypeDef {
    // --------------------------------------------------------------------------------------------
    // DatatypeDef :: Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(name: Identifier, base_type: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            opaque: false,
            base_type,
            body: None,
        }
    }

    pub const fn new_opaque(name: Identifier, base_type: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            opaque: true,
            base_type,
            body: None,
        }
    }

    pub fn with_body(self, body: AnnotationOnlyBody) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------
    // DatatypeDef :: Fields
    // --------------------------------------------------------------------------------------------

    pub fn is_opaque(&self) -> bool {
        self.opaque
    }

    pub fn set_opaque(&mut self, opaque: bool) {
        self.opaque = opaque;
    }

    get_and_set!(pub base_type, set_base_type => IdentifierReference);
}
