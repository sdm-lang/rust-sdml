/*!
Standard library module for namespace `rdf`.

*/

use crate::model::{identifiers::Identifier, modules::Module, definitions::{RdfDef, RdfDefBody}};
use url::Url;
use crate::model::HasBody;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "rdf";
pub const MODULE_URL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

pub const CLASS_ALT_NAME: &str = "Alt";
pub const CLASS_BAG_NAME: &str = "Bag";
pub const CLASS_HTML_NAME: &str = "HTML";
pub const CLASS_LANG_STRING_NAME: &str = "langString";
pub const CLASS_LIST_NAME: &str = "List";
pub const CLASS_PROPERTY_NAME: &str = "Property";
pub const CLASS_SEQ_NAME: &str = "Seq";
pub const CLASS_STATEMENT_NAME: &str = "Statement";
pub const CLASS_XML_LITERAL_NAME: &str = "XMLLiteral";

pub const PROP_FIRST_NAME: &str = "first";
pub const PROP_NIL_NAME: &str = "nil";
pub const PROP_OBJECT_NAME: &str = "object";
pub const PROP_PREDICATE_NAME: &str = "predicate";
pub const PROP_REST_NAME: &str = "rest";
pub const PROP_SUBJECT_NAME: &str = "subject";
pub const PROP_TYPE_NAME: &str = "type";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    let mut module = Module::empty(Identifier::new_unchecked(MODULE_NAME))
        .with_base_uri(Url::parse(MODULE_URL).unwrap());

    module.body_mut().extend_definitions(vec![
        // Classes
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_LANG_STRING_NAME))
        ).into(),
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_HTML_NAME))
        ).into(),
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_XML_LITERAL_NAME))
        ).into(),
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_PROPERTY_NAME))
        ).into(),
        // Properties
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_TYPE_NAME))
        ).into(),
        // Container Classes and Properties
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_ALT_NAME))
        ).into(),
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_BAG_NAME))
        ).into(),
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_SEQ_NAME))
        ).into(),
        // RDF Collections
         RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_LIST_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_FIRST_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_REST_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_NIL_NAME))
        ).into(),
        // Reification Vocabulary
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_STATEMENT_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_SUBJECT_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_PREDICATE_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_OBJECT_NAME))
        ).into(),
    ]);

    module
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
