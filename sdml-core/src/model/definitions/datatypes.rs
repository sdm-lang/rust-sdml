use crate::model::References;
use crate::model::{
    annotations::AnnotationOnlyBody,
    identifiers::{Identifier, IdentifierReference},
    Span,
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `data_type_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DatatypeDef {
    span: Option<Span>,
    name: Identifier,
    /// Corresponds to the grammar rule `data_type_base`.
    base_type: IdentifierReference,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(DatatypeDef);

impl_has_name_for!(DatatypeDef);

impl_has_optional_body_for!(DatatypeDef);

impl_validate_for!(DatatypeDef => delegate optional body, true, true);

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
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name: Identifier, base_type: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            base_type,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub base_type, set_base_type => IdentifierReference);
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
