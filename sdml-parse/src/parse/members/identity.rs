use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::members::parse_type_reference;
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::definitions::{EntityIdentity, EntityIdentityDef};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_PROPERTY, FIELD_NAME_TARGET};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
