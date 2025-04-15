/*!
This Rust module contains the SDML model of the SDML library module `dcam`.
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

pub const MODULE_NAME: &str = "dcam";
pub const MODULE_URL: &str = "http://purl.org/dc/dcam/";

pub const VOCABULARY_ENCODING_SCHEME: &str = "VocabularyEncodingScheme";
pub const DOMAIN_INCLUDES: &str = "domainIncludes";
pub const MEMBER_OF: &str = "memberOf";
pub const RANGE_INCLUDES: &str = "rangeIncludes";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

module_function!(|| {
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked dcam), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked dct),
            id!(unchecked rdf),
            id!(unchecked rdfs),
        )])
            .with_annotations([
                 annotation!(id!(unchecked dct:modified), v!(id!(unchecked xsd:date), "2012-06-14")),
                 annotation!(id!(unchecked publisher), url!("http://purl.org/dc/aboutdcmi#DCMI")),
                 annotation!(id!(unchecked title), v!("Metadata terms for vocabulary description")),
            ])
            .with_definitions([
                rdf!(
                    id!(unchecked VocabularyEncodingScheme) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dcam)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Vocabulary Encoding Scheme"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An enumerated set of resources."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.dublincore.org/specifications/dublin-core/2007/06/04/abstract-model/")),
                    ])).into(),
                rdf!(
                    id!(unchecked domainIncludes) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dcam)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Domain Includes"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2020-01-20")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A suggested class for subjects of this property."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked memberOf) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dcam)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Member Of"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A relationship between a resource and a vocabulary encoding scheme which indicates that the resource is a member of a set."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.dublincore.org/specifications/dublin-core/2007/06/04/abstract-model/")),
                    ])).into(),
                rdf!(
                    id!(unchecked rangeIncludes) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dcam)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Range Includes"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "2020-01-20")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A suggested class for values of this property."@en)),
                    ])).into(),
            ])
    )
});
