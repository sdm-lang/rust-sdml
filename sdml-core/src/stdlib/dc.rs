/*!
This Rust module contains the SDML model of the SDML library module `dc`.
*/

use crate::model::{
    annotations::{AnnotationOnlyBody, HasAnnotations},
    modules::Module,
    {HasOptionalBody},
};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "dc";
pub const MODULE_URL: &str = "http://purl.org/dc/elements/1.1/";

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
        id!(unchecked dc), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked dct),
            id!(unchecked rdf),
            id!(unchecked rdfs),
            id!(unchecked skos)
        )])
            .with_annotations([
                 annotation!(id!(unchecked dct:modified), v!(id!(unchecked xsd:date), "2012-06-14")),
                 annotation!(id!(unchecked publisher), url!("http://purl.org/dc/aboutdcmi#DCMI")),
                 annotation!(id!(unchecked title), v!("Dublin Core Metadata Element Set, Version 1.1")),
            ])
            .with_definitions([
                rdf!(
                    id!(unchecked contributor) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Contributor"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An entity responsible for making contributions to the resource."@en)),
                        annotation!(id!(unchecked description), rdf_str!("The guidelines for using names of persons or organizations as creators also apply to contributors. Typically, the name of a Contributor should be used to indicate the entity."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked coverage) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Coverage"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The spatial or temporal topic of the resource, spatial applicability of the resource, or jurisdiction under which the resource is relevant."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Spatial topic and spatial applicability may be a named place or a location specified by its geographic coordinates. Temporal topic may be a named period, date, or date range. A jurisdiction may be a named administrative entity or a geographic place to which the resource applies. Recommended practice is to use a controlled vocabulary such as the Getty Thesaurus of Geographic Names [[TGN](https://www.getty.edu/research/tools/vocabulary/tgn/index.html)]. Where appropriate, named places or time periods may be used in preference to numeric identifiers such as sets of coordinates or date ranges."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked creator) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Creator"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An entity primarily responsible for making the resource."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Examples o tor include a person, an organization, or a service. Typically, the name of a Creator should to indicate the entity."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked date) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Date"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A point or period of time associated with an event in the lifecycle of the resource."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Date may be used to express temporal information at any level of granularity.  Recommended practice is to express the date, date/time, or period of time according to ISO 8601-1 [[ISO 8601-1](https://www.iso.org/iso-8601-date-and-time-format.html)] or a published profile of the ISO standard, such as the W3C Note on Date and Time Formats [[W3CDTF](https://www.w3.org/TR/NOTE-datetime)] or the Extended Date/Time Format Specification [[EDTF](http://www.loc.gov/standards/datetime/)].  If the full date is unknown, month and year (YYYY-MM) or just year (YYYY) may be used. Date ranges may be specified using ISO 8601 period of time specification in which start and end dates are separated by a '/' (slash) character.  Either the start or end date may be missing."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked description) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Description"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An account of the resource."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Description may include but is not limited to: an abstract, a table of contents, a graphical representation, or a free-text account of the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked format) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("format"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The file format, physical medium, or dimensions of the resource."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Recommended practice is to use a controlled vocabulary where available. For example, for file formats one could use the list of Internet Media Types [[MIME](https://www.iana.org/assignments/media-types/media-types.xhtml)]."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked identifier) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Identifier"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An unambiguous reference to the resource within a given context."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Recommended practice is to identify the resource by means of a string conforming to an identification system."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked language) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Language"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A language of the resource."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Recommended practice is to use either a non-literal value representing a language from a controlled vocabulary such as ISO 639-2 or ISO 639-3, or a literal value consisting of an IETF Best Current Practice 47 [[IETF-BCP47](https://tools.ietf.org/html/bcp47)] language tag."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked publisher) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Publisher"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("An entity responsible for making the resource available."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Examples of a Publisher include a person, an organization, or a service. Typically, the name of a Publisher should be used to indicate the entity."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked relation) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Relation"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Recommended practice is to identify the related resource by means of a URI. If this is not possible or feasible, a string conforming to a formal identification system may be provided."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked rights) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Rights"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("Information about rights held in and over the resource."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Typically, rights information includes a statement about various property rights associated with the resource, including intellectual property rights."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked source) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Source"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A related resource from which the described resource is derived."@en)),
                        annotation!(id!(unchecked description), rdf_str!("The described resource may be derived from the related resource in whole or in part. Recommended best practice is to identify the related resource by means of a string conforming to a formal identification system."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked subject) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Subject"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The topic of the resource."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Typically, the subject will be represented using keywords, key phrases, or classification codes.  Recommended best practice is to use a controlled vocabulary."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked title) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Title"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("A name given to the resource."@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked type) ;
                    property id!(unchecked rdf:Property), id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked dc)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Type"@en)),
                        annotation!(id!(unchecked dct:issued), v!(id!(unchecked xsd:date), "1999-07-02")),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The nature or genre of the resource."@en)),
                        annotation!(id!(unchecked description), rdf_str!("Recommended practice is to use a controlled vocabulary such as the DCMI Type Vocabulary [[DCMI-TYPE](http://dublincore.org/documents/dcmi-type-vocabulary/)]. To describe the file format, physical medium, or dimensions of the resource, use the Format element."@en)),
                    ])).into(),
            ])
    )
});
