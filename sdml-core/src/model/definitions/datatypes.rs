use crate::cache::ModuleCache;
use crate::error::Error;
use crate::model::check::Validate;
use crate::model::modules::Module;
use crate::model::References;
use crate::model::{
    annotations::AnnotationOnlyBody,
    identifiers::{Identifier, IdentifierReference},
    HasOptionalBody, Span,
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
    opaque: bool,
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

impl_annotation_builder!(DatatypeDef, optional body);

impl Validate for DatatypeDef {
    fn is_complete(&self, top: &Module, cache: &ModuleCache) -> Result<bool, Error> {
        println!("DatatypeDef::is_complete");
        if let Some(body) = self.body() {
            body.is_complete(top, cache)
        } else {
            Ok(true)
        }
    }

    fn is_valid(
        &self,
        _check_constraints: bool,
        _top: &Module,
        _cache: &ModuleCache,
    ) -> Result<bool, Error> {
        println!("DatatypeDef::is_valid");
        // TODO: check that base_type is also a datatype
        Ok(true)
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
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name: Identifier, base_type: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            opaque: false,
            base_type,
            body: None,
        }
    }

    pub fn new_opaque(name: Identifier, base_type: IdentifierReference) -> Self {
        Self {
            span: None,
            name,
            opaque: true,
            base_type,
            body: None,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn is_opaque(&self) -> bool {
        self.opaque
    }

    pub fn set_opaque(&mut self, opaque: bool) {
        self.opaque = opaque;
    }

    get_and_set!(pub base_type, set_base_type => IdentifierReference);
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
