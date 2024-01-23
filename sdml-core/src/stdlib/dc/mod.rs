/*
This Rust module contains the SDML model of the SDML library module `dc`.
*/

use crate::model::HasBody;
use crate::model::{
    annotations::AnnotationBuilder,
    modules::{ImportStatement, Module},
};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "dc";
pub const MODULE_URL: &str = "http://purl.org/dc/elements/1.1/";

pub const PROP_CONTRIBUTOR_NAME: &str = "contributor";
pub const PROP_COVERAGE_NAME: &str = "coverage";
pub const PROP_CREATOR_NAME: &str = "creator";
pub const PROP_DATE_NAME: &str = "date";
pub const PROP_DESCRIPTION_NAME: &str = "description";
pub const PROP_FORMAT_NAME: &str = "format";
pub const PROP_IDENTIFIER_NAME: &str = "identifier";
pub const PROP_LANGUAGE_NAME: &str = "language";
pub const PROP_PUBLISHER_NAME: &str = "publisher";
pub const PROP_RELATION_NAME: &str = "relation";
pub const PROP_RIGHTS_NAME: &str = "rights";
pub const PROP_SOURCE_NAME: &str = "source";
pub const PROP_SUBJECT_NAME: &str = "subject";
pub const PROP_TITLE_NAME: &str = "title";
pub const PROP_TYPE_NAME: &str = "type";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME))
        .with_base_uri(Url::parse(MODULE_URL).unwrap());

    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(id!(super::rdf::MODULE_NAME)));
    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(id!(super::rdfs::MODULE_NAME)));

    module.body_mut().extend_definitions(vec![
        rdf!(property PROP_CONTRIBUTOR_NAME, MODULE_IRI)
            .with_comment(lstr!("An entity responsible for making contributions to the resource."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("The guidelines for using names of persons or organizations as creators also apply to contributors.  Typically, the name of a Contributor should be used to indicate the entity."@en)
            )
            .into(),
        rdf!(property PROP_COVERAGE_NAME, MODULE_IRI)
            .with_comment(lstr!("The spatial or temporal topic of the resource, spatial applicability of the resource, or jurisdiction under which the resource is relevant."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Spatial topic and spatial applicability may be a named place or a location specified by its geographic coordinates. Temporal topic may be a named period, date, or date range. A jurisdiction may be a named administrative entity or a geographic place to which the resource applies. Recommended practice is to use a controlled vocabulary such as the Getty Thesaurus of Geographic Names [[TGN](https://www.getty.edu/research/tools/vocabulary/tgn/index.html)]. Where appropriate, named places or time periods may be used in preference to numeric identifiers such as sets of coordinates or date ranges."@en)
            )
            .into(),
        rdf!(property PROP_CREATOR_NAME, MODULE_IRI)
            .with_comment(lstr!("An entity primarily responsible for making the resource."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Examples of a Creator include a person, an organization, or a service. Typically, the name of a Creator should be used to indicate the entity."@en)
            )
            .into(),
        rdf!(property PROP_DATE_NAME, MODULE_IRI)
            .with_comment(lstr!("A point or period of time associated with an event in the lifecycle of the resource."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Date may be used to express temporal information at any level of granularity.  Recommended practice is to express the date, date/time, or period of time according to ISO 8601-1 [[ISO 8601-1](https://www.iso.org/iso-8601-date-and-time-format.html)] or a published profile of the ISO standard, such as the W3C Note on Date and Time Formats [[W3CDTF](https://www.w3.org/TR/NOTE-datetime)] or the Extended Date/Time Format Specification [[EDTF](http://www.loc.gov/standards/datetime/)].  If the full date is unknown, month and year (YYYY-MM) or just year (YYYY) may be used. Date ranges may be specified using ISO 8601 period of time specification in which start and end dates are separated by a '/' (slash) character.  Either the start or end date may be missing."@en)
            )
            .into(),
        rdf!(property PROP_DESCRIPTION_NAME, MODULE_IRI)
            .with_comment(lstr!("An account of the resource."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Description may include but is not limited to: an abstract, a table of contents, a graphical representation, or a free-text account of the resource."@en)
            )
            .into(),
        rdf!(property PROP_FORMAT_NAME, MODULE_IRI)
            .with_comment(lstr!("The file format, physical medium, or dimensions of the resource."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Recommended practice is to use a controlled vocabulary where available. For example, for file formats one could use the list of Internet Media Types [[MIME](https://www.iana.org/assignments/media-types/media-types.xhtml)]."@en)
            )
            .into(),
        rdf!(property PROP_IDENTIFIER_NAME, MODULE_IRI)
            .with_comment(lstr!("An unambiguous reference to the resource within a given context."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Recommended practice is to identify the resource by means of a string conforming to an identification system."@en)
            )
            .into(),
        rdf!(property PROP_LANGUAGE_NAME, MODULE_IRI)
            .with_comment(lstr!("A language of the resource."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Recommended practice is to use either a non-literal value representing a language from a controlled vocabulary such as ISO 639-2 or ISO 639-3, or a literal value consisting of an IETF Best Current Practice 47 [[IETF-BCP47](https://tools.ietf.org/html/bcp47)] language tag."@en)
            )
            .into(),
        rdf!(property PROP_PUBLISHER_NAME, MODULE_IRI)
            .with_comment(lstr!("An entity responsible for making the resource available."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Examples of a Publisher include a person, an organization, or a service. Typically, the name of a Publisher should be used to indicate the entity."@en)
            )
            .into(),
        rdf!(property PROP_RELATION_NAME, MODULE_IRI)
            .with_comment(lstr!("A related resource."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Recommended practice is to identify the related resource by means of a URI. If this is not possible or feasible, a string conforming to a formal identification system may be provided."@en)
            )
            .into(),
        rdf!(property PROP_RIGHTS_NAME, MODULE_IRI)
            .with_comment(lstr!("Information about rights held in and over the resource."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Typically, rights information includes a statement about various property rights associated with the resource, including intellectual property rights."@en)
            )
            .into(),
        rdf!(property PROP_SOURCE_NAME, MODULE_IRI)
            .with_comment(lstr!("A related resource from which the described resource is derived."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("The described resource may be derived from the related resource in whole or in part. Recommended best practice is to identify the related resource by means of a string conforming to a formal identification system."@en)
            )
            .into(),
        rdf!(property PROP_SUBJECT_NAME, MODULE_IRI)
            .with_comment(lstr!("The topic of the resource."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Typically, the subject will be represented using keywords, key phrases, or classification codes. Recommended best practice is to use a controlled vocabulary."@en)
            )
            .into(),
        rdf!(property PROP_TITLE_NAME, MODULE_IRI)
            .with_comment(lstr!("A name given to the resource."@en))
            .into(),
        rdf!(property PROP_TYPE_NAME, MODULE_IRI)
            .with_comment(lstr!("The nature or genre of the resource."@en))
            .with_predicate(
                id!(PROP_DESCRIPTION_NAME),
                lstr!("Recommended practice is to use a controlled vocabulary such as the DCMI Type Vocabulary [[DCMI-TYPE](http://dublincore.org/documents/dcmi-type-vocabulary/)]. To describe the file format, physical medium, or dimensions of the resource, use the Format element."@en)
            )
            .into(),
    ]);

    module
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod terms;
