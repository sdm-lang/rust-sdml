/*!
This Rust module contains the SDML model of the SDML library module `dc_terms`.
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

pub const MODULE_NAME: &str = "dcterms";
pub const MODULE_URL: &str = "http://purl.org/dc/terms/";

pub const CONTRIBUTOR: &str = "contributor";
pub const COVERAGE: &str = "coverage";
pub const CREATOR: &str = "creator";
pub const DATE: &str = "date";
pub const DESCRIPTION: &str = "description";
pub const FORMAT: &str = "format";
pub const IDENTIFIER: &str = "identifier";
pub const LANGUAGE: &str = "language";
pub const PUBLISHER: &str = "publisher";
pub const RELATION: &str = "relation";
pub const RIGHTS: &str = "rights";
pub const SOURCE: &str = "source";
pub const SUBJECT: &str = "subject";
pub const TITLE: &str = "title";
pub const TYPE: &str = "type";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

module_function!(|| {
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked dcterms), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked dc),
            id!(unchecked rdf),
            id!(unchecked rdfs),
            id!(unchecked skos)
        )])
            .with_annotations([
                 annotation!(id!(unchecked modified), v!(id!(unchecked xsd:date), "2012-06-14")),
                 annotation!(id!(unchecked publisher), url!("http://purl.org/dc/aboutdcmi#DCMI")),
                 annotation!(id!(unchecked title), v!("DCMI Metadata Terms - other")),
            ])
            .with_definitions([
                // ---------------------------------------------------------------------------------
                // Classes
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked Agent) ;
                    class id!(unchecked AgentClass) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked AgentClass)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Agent"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A resource that acts or has the power to act."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked AgentClass) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("AgentClass"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A group of agents."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked BibliographicResource) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("BibliographicResource"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Bibliographic Resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Box) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("DCMI Box"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of regions in space defined by their geographic coordinates according to the DCMI Box Encoding Scheme."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.dublincore.org/specifications/dublin-core/dcmi-box/")),
                    ])).into(),
                rdf!(
                    id!(unchecked DCMIType) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("DCMI Type Vocabulary"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of classes specified by the DCMI Type Vocabulary, used to categorize the nature or genre of the resource."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://purl.org/dc/dcmitype/")),
                    ])).into(),
                rdf!(
                    id!(unchecked DDC) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("DDC"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of conceptual resources specified by the Dewey Decimal Classification."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.oclc.org/dewey/")),
                    ])).into(),
                rdf!(
                    id!(unchecked FileFormat) ;
                    class id!(unchecked MediaType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("File Format"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A digital resource format."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Frequency) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Frequency"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A rate at which something recurs."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked IMT) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked dcam:VocabularyEncodingScheme)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("IMT"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2007-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of media types specified by the Internet Assigned Numbers Authority."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.iana.org/assignments/media-types/")),
                    ])).into(),
                rdf!(
                    id!(unchecked ISO3166) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("ISO 3166"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2007-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of codes listed in ISO 3166-1 for the representation of names of countries."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked ISO639_2) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("ISO 639-2"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2007-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of two-letter codes listed in ISO 639-3 for the representation of names of languages."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked ISO639_3) ;
                    datatype  ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("ISO 639-3"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2007-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of three-letter codes listed in ISO 639-3 for the representation of names of languages."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked ISO639_3) ;
                    datatype  ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("ISO 639-3"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2007-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of three-letter codes listed in ISO 639-3 for the representation of names of languages."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Jurisdiction) ;
                    class id!(unchecked LocationPeriodOrJurisdiction) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Jurisdiction"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The extent or range of judicial, law enforcement, or other authority."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked LCC) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked dcam:VocabularyEncodingScheme)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("LCC"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of conceptual resources specified by the Library of Congress Classification."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://lcweb.loc.gov/catdir/cpso/lcco/lcco.html")),
                    ])).into(),
                rdf!(
                    id!(unchecked LCSH) ;
                    class id!(unchecked LocationPeriodOrJurisdiction) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked dcam:VocabularyEncodingScheme)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("LCSH"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of labeled concepts specified by the Library of Congress Subject Headings."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked LicenseDocument) ;
                    class id!(unchecked RightsStatement) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("License Document"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A legal document giving official permission to do something with a resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked LinguisticSystem) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Linguistic System"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A system of signs, symbols, sounds, gestures, or rules used in communication."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Location) ;
                    class id!(unchecked LocationPeriodOrJurisdiction) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Location, Period, or Jurisdiction"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A spatial region or named place."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked LocationPeriodOrJurisdiction) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Location"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A location, period of time, or jurisdiction."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked MESH) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked dcam:VocabularyEncodingScheme)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("MeSH"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of labeled concepts specified by the Medical Subject Headings."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.nlm.nih.gov/mesh/meshhome.html")),
                    ])).into(),
                rdf!(
                    id!(unchecked MediaType) ;
                    class id!(unchecked MediaTypeOrExtent) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Media Type"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A file format or physical medium."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked MediaTypeOrExtent) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Media Type or Extent"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A media type or extent."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked MethodOfAccrual) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Method of Accrual"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A method by which resources are added to a collection."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked MethodOfInstruction) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Method of Instruction"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A process that is used to engender knowledge, attitudes, and skills."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked NLM) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("NLM"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2005-06-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of conceptual resources specified by the National Library of Medicine Classification."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://wwwcf.nlm.nih.gov/class/")),
                    ])).into(),
                rdf!(
                    id!(unchecked Period) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Period"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of time intervals defined by their limits according to the DCMI Period Encoding Scheme."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.dublincore.org/specifications/dublin-core/dcmi-period/")),
                    ])).into(),
                rdf!(
                    id!(unchecked PeriodOfTime) ;
                    class id!(unchecked LocationPeriodOrJurisdiction) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Period of Time"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An interval of time that is named or defined by its start and end dates."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked PhysicalMedium) ;
                    class id!(unchecked MediaType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Physical Medium"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A physical material or carrier."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked PhysicalResource) ;
                    class id!(unchecked MediaType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Physical Resource"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A material thing."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Point) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("DCMI Point"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of points in space defined by their geographic coordinates according to the DCMI Point Encoding Scheme."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.dublincore.org/specifications/dublin-core/dcmi-point/")),
                    ])).into(),
                rdf!(
                    id!(unchecked Policy) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Policy"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A plan or course of action by an authority, intended to influence and determine decisions, actions, and other matters."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked ProvenanceStatement) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Provenance Statement"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Any changes in ownership and custody of a resource since its creation that are significant for its authenticity, integrity, and interpretation."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked RFC1766) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("RFC 1766"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of tags, constructed according to RFC 1766, for the identification of languages."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.ietf.org/rfc/rfc1766.txt")),
                    ])).into(),
                rdf!(
                    id!(unchecked RFC3066) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("RFC 3066"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2002-07-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of tags constructed according to RFC 3066 for the identification of languages."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.ietf.org/rfc/rfc3066.txt")),
                    ])).into(),
                rdf!(
                    id!(unchecked RFC4646) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("RFC 4646"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of tags constructed according to RFC 4646 for the identification of languages."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.ietf.org/rfc/rfc4646.txt")),
                    ])).into(),
                rdf!(
                    id!(unchecked RFC5646) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("RFC 5646"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2010-10-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of tags constructed according to RFC 5646 for the identification of languages."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.ietf.org/rfc/rfc5646.txt")),
                    ])).into(),
                rdf!(
                    id!(unchecked RightsStatement) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Rights Statement"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A statement about the intellectual property rights (IPR) held in or over a resource, a legal document giving official permission to do something with a resource, or a statement about access rights."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked SizeOrDuration) ;
                    class id!(unchecked MediaTypeOrExtent) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Size or Duration"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A dimension or extent, or a time taken to play or execute."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Standard) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Standard"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A reference point against which other things can be evaluated or compared."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked TGN) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked dcam:VocabularyEncodingScheme)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("TGN"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of places specified by the Getty Thesaurus of Geographic Names."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.getty.edu/research/tools/vocabulary/tgn/index.html")),
                    ])).into(),
                rdf!(
                    id!(unchecked UDC) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked dcam:VocabularyEncodingScheme)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("UDC"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of conceptual resources specified by the Universal Decimal Classification."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.udcc.org/")),
                    ])).into(),
                rdf!(
                    id!(unchecked URI) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("URI"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of identifiers constructed according to the generic syntax for Uniform Resource Identifiers as specified by the Internet Engineering Task Force."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.ietf.org/rfc/rfc3986.txt")),
                    ])).into(),
                rdf!(
                    id!(unchecked W3CDTF) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("W3C-DTF"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The set of dates and times constructed according to the W3C Date and Time Formats Specification."@en)),
                        annotation!(id!(unchecked rdfs:seeAlso), url!("http://www.w3.org/TR/NOTE-datetime")),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Properties
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked abstract) ;
                    property id!(unchecked dc:description), id!(unchecked description) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Abstract"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A summary of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked accessRights) ;
                    property id!(unchecked dc:rights), id!(unchecked rights) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Access Rights"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2003-02-15")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Information about who access the resource or an indication of its security status."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked accrualMethod) ;
                    property id!(unchecked dc:rights), id!(unchecked rights) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked dcmitype:Collection)),
                        annotation!(id!(unchecked dcam:rangeIncludes), id!(unchecked MethodOfAccrual)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Accrual Method"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2005-06-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The method by which items are added to a collection."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked accrualPeriodicity) ;
                    property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked dcmitype:Collection)),
                        annotation!(id!(unchecked dcam:rangeIncludes), id!(unchecked Frequency)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Accrual Periodicity"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2005-06-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The frequency with which items are added to a collection."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked accrualPolicy) ;
                    property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked dcmitype:Collection)),
                        annotation!(id!(unchecked dcam:rangeIncludes), id!(unchecked Policy)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Accrual Policy"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2005-06-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The policy governing the addition of items to a collection."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked alternative) ;
                    property id!(unchecked dc:title), id!(unchecked title) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Alternative"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Alternative Title."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked available) ;
                    property id!(unchecked dc:date), id!(unchecked date) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Date Available"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Date that the resource became or will become available."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked bibliographicCitation) ;
                    property id!(unchecked dc:identifier), id!(unchecked identifier) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Bibliographic Citation"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2003-02-15")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A bibliographic reference for the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked conformsTo) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), id!(unchecked Standard)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Conforms To"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2001-05-21")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An established standard to which the described resource conforms."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked contributor) ;
                    property id!(unchecked dc:contributor) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), id!(unchecked Agent)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Contributor"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An entity responsible for making contributions to the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked coverage) ;
                    property id!(unchecked dc:coverage) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked Jurisdiction), id!(unchecked Location), id!(unchecked Period))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Coverage"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The spatial or temporal topic of the resource, spatial applicability of the resource, or jurisdiction under which the resource is relevant."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked created) ;
                    property id!(unchecked dc:date), id!(unchecked date) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Date Created"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Date of creation of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked creator) ;
                    property id!(unchecked dc:creator), id!(unchecked contributor) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), id!(unchecked Agent)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Creator"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An entity primarily responsible for making the resource."@en)),
                        // TODO: owl:equivalentProperty <http://xmlns.com/foaf/0.1/maker>
                    ])).into(),
                rdf!(
                    id!(unchecked date) ;
                    property id!(unchecked dc:date) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Date"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A point or period of time associated with an event in the lifecycle of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked dateAccepted) ;
                    property id!(unchecked dc:date), id!(unchecked date) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Date Accepted"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2002-07-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Date of acceptance of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked dateCopyrighted) ;
                    property id!(unchecked dc:date), id!(unchecked date) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Date Copyrighted"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2002-07-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Date of copyright of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked dateSubmitted) ;
                    property id!(unchecked dc:date), id!(unchecked date) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Date Submitted"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2002-07-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Date of submission of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked description) ;
                    property id!(unchecked dc:description) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Description"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An account of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked educationLevel) ;
                    property id!(unchecked audience) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked dcam:rangeIncludes), id!(unchecked AgentClass)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Description"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2002-07-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Audience Education Level"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked extent) ;
                    property id!(unchecked dc:format), id!(unchecked format) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Extent"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The size or duration of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked format) ;
                    property id!(unchecked dc:format) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked Extent), id!(unchecked MediaType))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("format"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The file format, physical medium, or dimensions of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked hasPart) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Has Part"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource that is substantially the same as the pre-existing described resource, but in another format."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked hasFormat) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Has Format"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource that is included either physically or logically in the described resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked identifier) ;
                    property id!(unchecked dc:identifier) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Identifier"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An unambiguous reference to the resource within a given context."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked instructionalMethod) ;
                    property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked MethodOfInstruction))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Instructional Method"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2005-06-13")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A process, used to engender knowledge, attitudes and skills, that the described resource is designed to support."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked isFormatOf) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Is Format Of"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A pre-existing related resource that is substantially the same as the described resource, but in another format."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked isPartOf) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Is Replaced By"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource that references, cites, or otherwise points to the described resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked isReplacedBy) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Is Part Of"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource that supplants, displaces, or supersedes the described resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked isRequiredBy) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Is Required By"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource that requires the described resource to support its function, delivery, or coherence."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked isVersionOf) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Is Version Of"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource of which the described resource is a version, edition, or adaptation."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked issued) ;
                    property id!(unchecked dc:date), id!(unchecked date) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Date Issued"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Date of formal issuance of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked language) ;
                    property id!(unchecked dc:language) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked LinguisticSystem))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Language"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A language of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked license) ;
                    property id!(unchecked dc:rights), id!(unchecked rights) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked LicenseDocument))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("License"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2004-06-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A legal document giving official permission to do something with the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked mediator) ;
                    property id!(unchecked audience) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked AgentClass))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Mediator"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2001-05-21")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An entity that mediates access to the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked medium) ;
                    property id!(unchecked dc:format), id!(unchecked format) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:domainIncludes), vs!(id!(unchecked PhysicalResource))),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked PhysicalMedium))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Medium"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The material or physical carrier of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked modified) ;
                    property id!(unchecked dc:date), id!(unchecked date) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), vs!(id!(unchecked rdfs:Literal))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Date Modified"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Date on which the resource was changed."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked provenance) ;
                    property id!(unchecked dc:date), id!(unchecked date) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked ProvenanceStatement))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Provenance"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2004-09-20")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A statement of any changes in ownership and custody of the resource since its creation that are significant for its authenticity, integrity, and interpretation."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked publisher) ;
                    property id!(unchecked dc:publisher) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked Agent))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Publisher"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An entity responsible for making the resource available."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked references) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("References"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource that is referenced, cited, or otherwise pointed to by the described resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked relation) ;
                    property id!(unchecked dc:relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Relation"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked replaces) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Replaces"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource that is supplanted, displaced, or superseded by the described resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked requires) ;
                    property id!(unchecked dc:relation), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Requires"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource that is required by the described resource to support its function, delivery, or coherence."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked rights) ;
                    property id!(unchecked dc:rights) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked RightsStatement))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Rights"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Information about rights held in and over the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked rightsHolder) ;
                    property id!(unchecked dc:rights) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked Agent))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Rights Holder"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2004-06-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A person or organization owning or managing rights over the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked source) ;
                    property id!(unchecked dc:source), id!(unchecked relation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Source"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource from which the described resource is derived."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked spatial) ;
                    property id!(unchecked dc:coverage), id!(unchecked coverage) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked Location))),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Spatial Coverage"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Spatial characteristics of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked subject) ;
                    property id!(unchecked dc:subject) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Subject"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A topic of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked tableOfContents) ;
                    property id!(unchecked dc:description), id!(unchecked description) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Table Of Contents"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A list of subunits of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked temporal) ;
                    property id!(unchecked dc:coverage), id!(unchecked coverage) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked dcam:rangeIncludes), vs!(id!(unchecked PeriodOfTime))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Temporal Coverage"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Temporal characteristics of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked title) ;
                    property id!(unchecked dc:title) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), vs!(id!(unchecked rdfs:Literal))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Title"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A name given to the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked type) ;
                    property id!(unchecked dc:type) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Type"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2008-01-14")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The nature or genre of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked valid) ;
                    property id!(unchecked dc:date), id!(unchecked date) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dct)),
                        annotation!(id!(unchecked rdfs:range), vs!(id!(unchecked rdfs:Literal))),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Date Valid"@en)),
                        annotation!(id!(unchecked issued), v!(id!(unchecked xsd:date), "2000-07-11")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The nature or genre of the resource."@en)),
                    ])).into(),
            ])
    )
});
