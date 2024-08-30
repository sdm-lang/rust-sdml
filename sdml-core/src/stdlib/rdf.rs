/*!
This Rust module contains the SDML model of the SDML library module `rdf` for RDF syntax.
*/

use crate::model::annotations::AnnotationBuilder;
use crate::model::modules::Module;
use crate::model::HasBody;
use crate::stdlib::rdfs;
// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "rdf";
pub const MODULE_URL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

pub const HTML: &str = "HTML";
pub const JSON: &str = "JSON";
pub const LANG_STRING: &str = "langString";
pub const PLAIN_LITERAL: &str = "PlainLiteral";
pub const XML_LITERAL: &str = "XMLLiteral";

pub const ALT: &str = "Alt";
pub const BAG: &str = "Bag";
pub const LIST: &str = "List";
pub const PROPERTY: &str = "Property";
pub const SEQ: &str = "Seq";
pub const STATEMENT: &str = "Statement";

pub const FIRST: &str = "first";
pub const NIL: &str = "nil";
pub const OBJECT: &str = "object";
pub const PREDICATE: &str = "predicate";
pub const REST: &str = "rest";
pub const SUBJECT: &str = "subject";
pub const TYPE: &str = "type";
pub const VALUE: &str = "value";

pub const COMPOUND: &str = "CompoundLiteral";
pub const LANGUAGE: &str = "language";
pub const DIRECTION: &str = "direction";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(MODULE_IRI.clone());

    module
        .body_mut()
        .add_to_imports(import!(id!(rdfs::MODULE_NAME)));

    module.body_mut().extend_definitions(vec![
        // Datatypes
        rdf!(datatype HTML, MODULE_IRI; rdfs::LITERAL)
            .with_comment("The datatype of RDF literals storing fragments of HTML content")
            .with_see_also_str("http://www.w3.org/TR/rdf11-concepts/#section-html")
            .into(),
        rdf!(datatype JSON, MODULE_IRI; rdfs::LITERAL)
            .with_comment("The datatype of RDF literals storing JSON content")
            .with_see_also_str("https://www.w3.org/TR/json-ld11/#the-rdf-json-datatype")
            .into(),
        rdf!(datatype LANG_STRING, MODULE_IRI; rdfs::LITERAL)
            .with_comment("The datatype of language-tagged string values")
            .with_see_also_str("http://www.w3.org/TR/rdf11-concepts/#section-Graph-Literal")
            .into(),
        rdf!(datatype PLAIN_LITERAL, MODULE_IRI; rdfs::LITERAL)
            .with_comment("The class of plain (i.e. untyped) literal values, as used in RIF and OWL 2")
            .with_see_also_str("http://www.w3.org/TR/rdf-plain-literal/")
            .into(),
        rdf!(datatype XML_LITERAL, MODULE_IRI; rdfs::LITERAL)
            .with_comment("The datatype of XML literal values")
            .into(),

        // Classes
        rdf!(class PROPERTY, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The class of RDF properties.")
            .into(),

        // Properties
        rdf!(property TYPE, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::RESOURCE) =>
             (rdfs::MODULE_NAME, rdfs::CLASS))
            .into(),

        // Container Classes and Properties
        rdf!(class ALT, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::CONTAINER))
            .with_comment("The class of containers of alternatives.")
            .into(),
        rdf!(class BAG, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::CONTAINER))
            .with_comment("The class of unordered containers.")
            .into(),
        rdf!(class SEQ, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::CONTAINER))
            .with_comment("The class of ordered containers.")
            .into(),

        // RDF Collections
        rdf!(class LIST, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The class of RDF Lists.")
           .into(),
        rdf!(property FIRST, MODULE_IRI;
             LIST => (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The first item in the subject RDF list.")
            .into(),
        rdf!(property REST, MODULE_IRI; LIST => LIST)
            .with_comment("The rest of the subject RDF list after the first item.")
            .into(),
        rdf!(thing NIL, MODULE_IRI, LIST)
            .with_comment("The empty list, with no items in it. If the rest of a list is nil then the list has no more items in it.")
            .into(),

        // Reification Vocabulary
        rdf!(class STATEMENT, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The class of RDF statements.")
            .into(),
        rdf!(property SUBJECT, MODULE_IRI;
             STATEMENT => (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The subject of the subject RDF statement.")
            .into(),
        rdf!(property PREDICATE, MODULE_IRI;
             STATEMENT => (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The predicate of the subject RDF statement.")
            .into(),
        rdf!(property OBJECT, MODULE_IRI;
             STATEMENT => (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The object of the subject RDF statement.")
            .into(),

        rdf!(property VALUE, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::RESOURCE) =>
             (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("Idiomatic property used for structured values.")
            .into(),

        // Compound Literals
        rdf!(class COMPOUND, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("A class representing a compound literal.")
            .with_see_also_str("https://www.w3.org/TR/json-ld11/#the-rdf-compoundliteral-class-and-the-rdf-language-and-rdf-direction-properties")
            .into(),
        rdf!(property DIRECTION, MODULE_IRI; COMPOUND)
            .with_comment("The language component of a CompoundLiteral.")
            .with_see_also_str("https://www.w3.org/TR/json-ld11/#the-rdf-compoundliteral-class-and-the-rdf-language-and-rdf-direction-properties")
            .into(),
        rdf!(property LANGUAGE, MODULE_IRI; COMPOUND)
            .with_comment("The base direction component of a CompoundLiteral.")
            .with_see_also_str("https://www.w3.org/TR/json-ld11/#the-rdf-compoundliteral-class-and-the-rdf-language-and-rdf-direction-properties")
            .into(),
     ]).unwrap();

    module
}
