/*!
This Rust module contains the SDML model of the SDML library module `rdf` for RDF syntax.
*/

use crate::model::{
    annotations::{AnnotationOnlyBody, HasAnnotations},
    modules::Module,
    HasBody,
};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_PATH: &str = "::org::w3";
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

module_function!(|| {
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked rdf), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked rdfs),
        )])
            .with_definitions([
                // ---------------------------------------------------------------------------------
                // Literal Types
                // ---------------------------------------------------------------------------------
                rdf!(id!(unchecked HTML) ;
                    class id!(unchecked rdfs:Literal) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("HTML"@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The datatype of RDF literals storing fragments of HTML content"@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.w3.org/TR/rdf11-concepts/#section-html")),
                    ])).into(),
                rdf!(id!(unchecked JSON) ;
                    class id!(unchecked rdfs:Literal) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("JSON"@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The datatype of RDF literals storing JSON content"@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.w3.org/TR/json-ld11/#the-rdf-json-datatype")),
                    ])).into(),
                rdf!(id!(unchecked langString) ;
                    class id!(unchecked rdfs:Literal) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Language-Tagged String"@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The datatype of language-tagged string values"@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.w3.org/TR/rdf11-concepts/#section-Graph-Literal")),
                    ])).into(),
                rdf!(id!(unchecked plainLiteral) ;
                    class id!(unchecked rdfs:Literal) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Plain Literal"@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of plain (i.e. untyped) literal values, as used in RIF and OWL 2"@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.w3.org/TR/rdf-plain-literal/")),
                    ])).into(),
                rdf!(id!(unchecked XMLLiteral) ;
                    class id!(unchecked rdfs:Literal) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("XML Literal"@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The datatype of XML literal values")),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Generic Properties
                // ---------------------------------------------------------------------------------
                rdf!(id!(unchecked Property) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(Property@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of RDF properties")),
                    ])).into(),
                rdf!(id!(unchecked type) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(type@en)),
                        annotation!(id!(unchecked skos:altLabel), rdf_str!(a@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Class)),
                    ])).into(),
                rdf!(id!(unchecked value) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(value@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Idiomatic property used for structured values"@en)),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Container Classes and Properties
                // ---------------------------------------------------------------------------------
                rdf!(id!(unchecked Alt) ;
                    class id!(unchecked rdfs:Container) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Property"@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of containers of alternatives")),
                    ])).into(),
                rdf!(id!(unchecked Bag) ;
                    class id!(unchecked rdfs:Container) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Bag"@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of unordered containers")),
                    ])).into(),
                rdf!(id!(unchecked Seq) ;
                    class id!(unchecked rdfs:Container) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Seq"@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of ordered containers")),
                    ])).into(),
                rdf!(id!(unchecked List) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(List@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of RDF Lists")),
                    ])).into(),
                rdf!(id!(unchecked first) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(first@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked List)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The first item in the subject RDF list"@en)),
                    ])).into(),
                rdf!(id!(unchecked rest) ;
                    property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(rest@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked List)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked List)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The rest of the subject RDF list after the first item"@en)),
                    ])).into(),
                rdf!(id!(unchecked nil) ;
                    unnamed individual ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(language@en)),
                        annotation!(id!(unchecked type), id!(unchecked List)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The empty list, with no items in it. If the rest of a list is nil then the list has no more items in it")),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Reification Vocabulary
                // ---------------------------------------------------------------------------------
                rdf!(id!(unchecked Statement) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(Statement@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of RDF statements")),
                    ])).into(),
                rdf!(id!(unchecked subject) ;
                    property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(subject@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Statement)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The subject of the subject RDF statement")),
                    ])).into(),                rdf!(id!(unchecked predicate) ;
                    property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(predicate@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Statement)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The predicate of the subject RDF statement")),
                    ])).into(),
                rdf!(id!(unchecked object) ;
                    property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(object@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Statement)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The object of the subject RDF statement")),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Compound Literals
                // ---------------------------------------------------------------------------------
                rdf!(id!(unchecked CompoundLiteral) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(CompoundLiteral@en)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A class representing a compound literal")),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.w3.org/TR/json-ld11/#the-rdf-compoundliteral-class-and-the-rdf-language-and-rdf-direction-properties")),
                    ])).into(),
                rdf!(id!(unchecked direction) ;
                    property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(direction@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked CompoundLiteral)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Literal)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The base direction component of a CompoundLiteral")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The range of the property is an rdfs:Literal, whose value MUST be either 'ltr' or 'rtl'")),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.w3.org/TR/json-ld11/#the-rdf-compoundliteral-class-and-the-rdf-language-and-rdf-direction-properties")),
                    ])).into(),
                rdf!(id!(unchecked language) ;
                    property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdf)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(language@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked CompoundLiteral)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Literal)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The language component of a CompoundLiteral")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The range of the property is an rdfs:Literal, whose value MUST be a well-formed [BCP47] language tag")),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.w3.org/TR/json-ld11/#the-rdf-compoundliteral-class-and-the-rdf-language-and-rdf-direction-properties")),
                    ])).into(),
            ])
    )
});
