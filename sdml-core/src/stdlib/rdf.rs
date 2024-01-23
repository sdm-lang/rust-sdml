/*!
Standard library module for namespace `rdf`.

*/

use crate::model::modules::{ImportStatement, Module};
use crate::model::annotations::AnnotationBuilder;
use crate::model::HasBody;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "rdf";
pub const MODULE_URL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

pub const DT_HTML_NAME: &str = "HTML";
pub const DT_JSON_NAME: &str = "JSON";
pub const DT_LANG_STRING_NAME: &str = "langString";
pub const DT_PLAIN_LITERAL_NAME: &str = "PlainLiteral";
pub const DT_XML_LITERAL_NAME: &str = "XMLLiteral";

pub const CLASS_ALT_NAME: &str = "Alt";
pub const CLASS_BAG_NAME: &str = "Bag";
pub const CLASS_LIST_NAME: &str = "List";
pub const CLASS_PROPERTY_NAME: &str = "Property";
pub const CLASS_SEQ_NAME: &str = "Seq";
pub const CLASS_STATEMENT_NAME: &str = "Statement";

pub const PROP_FIRST_NAME: &str = "first";
pub const PROP_NIL_NAME: &str = "nil";
pub const PROP_OBJECT_NAME: &str = "object";
pub const PROP_PREDICATE_NAME: &str = "predicate";
pub const PROP_REST_NAME: &str = "rest";
pub const PROP_SUBJECT_NAME: &str = "subject";
pub const PROP_TYPE_NAME: &str = "type";
pub const PROP_VALUE_NAME: &str = "value";

pub const CLASS_COMPOUND_LITERAL: &str = "CompoundLiteral";
pub const PROP_LANGUAGE_NAME: &str = "language";
pub const PROP_DIRECTION_NAME: &str = "direction";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(MODULE_IRI.clone());

    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(id!(super::rdfs::MODULE_NAME)));

    module.body_mut().extend_definitions(vec![
        // Datatypes
        rdf!(datatype DT_HTML_NAME, MODULE_IRI; super::rdfs::CLASS_LITERAL_NAME)
            .with_comment("The datatype of RDF literals storing fragments of HTML content")
            .with_see_also_str("http://www.w3.org/TR/rdf11-concepts/#section-html")
            .into(),
        rdf!(datatype DT_JSON_NAME, MODULE_IRI; super::rdfs::CLASS_LITERAL_NAME)
            .with_comment("The datatype of RDF literals storing JSON content")
            .with_see_also_str("https://www.w3.org/TR/json-ld11/#the-rdf-json-datatype")
            .into(),
        rdf!(datatype DT_LANG_STRING_NAME, MODULE_IRI; super::rdfs::CLASS_LITERAL_NAME)
            .with_comment("The datatype of language-tagged string values")
            .with_see_also_str("http://www.w3.org/TR/rdf11-concepts/#section-Graph-Literal")
            .into(),
        rdf!(datatype DT_PLAIN_LITERAL_NAME, MODULE_IRI; super::rdfs::CLASS_LITERAL_NAME)
            .with_comment("The class of plain (i.e. untyped) literal values, as used in RIF and OWL 2")
            .with_see_also_str("http://www.w3.org/TR/rdf-plain-literal/")
            .into(),
        rdf!(datatype DT_XML_LITERAL_NAME, MODULE_IRI; super::rdfs::CLASS_LITERAL_NAME)
            .with_comment("The datatype of XML literal values")
            .into(),

        // Classes
        rdf!(class CLASS_PROPERTY_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The class of RDF properties.")
            .into(),

        // Properties
        rdf!(property PROP_TYPE_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME) =>
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_CLASS_NAME))
            .into(),

        // Container Classes and Properties
        rdf!(class CLASS_ALT_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_CONTAINER_NAME))
            .with_comment("The class of containers of alternatives.")
            .into(),
        rdf!(class CLASS_BAG_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_CONTAINER_NAME))
            .with_comment("The class of unordered containers.")
            .into(),
        rdf!(class CLASS_SEQ_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_CONTAINER_NAME))
            .with_comment("The class of ordered containers.")
            .into(),

        // RDF Collections
        rdf!(class CLASS_LIST_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The class of RDF Lists.")
           .into(),
        rdf!(property PROP_FIRST_NAME, MODULE_IRI;
             CLASS_LIST_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The first item in the subject RDF list.")
            .into(),
        rdf!(property PROP_REST_NAME, MODULE_IRI; CLASS_LIST_NAME => CLASS_LIST_NAME)
            .with_comment("The rest of the subject RDF list after the first item.")
            .into(),
        rdf!(thing PROP_NIL_NAME, MODULE_IRI, CLASS_LIST_NAME)
            .with_comment("The empty list, with no items in it. If the rest of a list is nil then the list has no more items in it.")
            .into(),

        // Reification Vocabulary
        rdf!(class CLASS_STATEMENT_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The class of RDF statements.")
            .into(),
        rdf!(property PROP_SUBJECT_NAME, MODULE_IRI;
             CLASS_STATEMENT_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The subject of the subject RDF statement.")
            .into(),
        rdf!(property PROP_PREDICATE_NAME, MODULE_IRI;
             CLASS_STATEMENT_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The predicate of the subject RDF statement.")
            .into(),
        rdf!(property PROP_OBJECT_NAME, MODULE_IRI;
             CLASS_STATEMENT_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The object of the subject RDF statement.")
            .into(),

        rdf!(property PROP_VALUE_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME) =>
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("Idiomatic property used for structured values.")
            .into(),

        // Compound Literals
        rdf!(class CLASS_COMPOUND_LITERAL, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("A class representing a compound literal.")
            .with_see_also_str("https://www.w3.org/TR/json-ld11/#the-rdf-compoundliteral-class-and-the-rdf-language-and-rdf-direction-properties")
            .into(),
        rdf!(property PROP_DIRECTION_NAME, MODULE_IRI; CLASS_COMPOUND_LITERAL)
            .with_comment("The language component of a CompoundLiteral.")
            .with_see_also_str("https://www.w3.org/TR/json-ld11/#the-rdf-compoundliteral-class-and-the-rdf-language-and-rdf-direction-properties")
            .into(),
        rdf!(property PROP_LANGUAGE_NAME, MODULE_IRI; CLASS_COMPOUND_LITERAL)
            .with_comment("The base direction component of a CompoundLiteral.")
            .with_see_also_str("https://www.w3.org/TR/json-ld11/#the-rdf-compoundliteral-class-and-the-rdf-language-and-rdf-direction-properties")
            .into(),
     ]);

    module
}
