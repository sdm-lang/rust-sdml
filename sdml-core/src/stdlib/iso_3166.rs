/*!
This Rust module contains the SDML model of the SDML library module `iso_3166` for ISO-3166.
*/

use crate::model::annotations::{AnnotationOnlyBody, HasAnnotations};
use crate::model::definitions::UnionBody;
use crate::model::modules::Module;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "iso_3166";
library_module_url! { "iso", "3166:2020" }

pub const MODULE_SRC: &str = include_str!("org/iso/iso_3166.sdml");

pub const COUNTRY_CODE_ALPHA_2: &str = "CountryCodeAlpha2";
pub const COUNTRY_CODE_ALPHA_3: &str = "CountryCodeAlpha3";
pub const COUNTRY_CODE_NUMERIC_3: &str = "CountryCodeNumeric3";
pub const COUNTRY_CODE: &str = "CountryCode";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

module_function!(|| {
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked iso_3166), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked dc_terms),
            id!(unchecked owl),
            id!(unchecked rdf),
            id!(unchecked rdfs),
            id!(unchecked skos)
        )])
            .with_annotations([
                annotation!(id!(unchecked skos:prefLabel), rdf_str!("ISO 3166-1:2020"@en)),
                annotation!(id!(unchecked dc_terms:version), rdf_str!("4"@en)),
                annotation!(id!(unchecked dc_terms:replaces), rdf_str!("ISO 3166-1:2013"@en)),
                annotation!(id!(unchecked dc_terms:description), rdf_str!("Codes for the representation of names of countries and their subdivisions — Part 1: Country code"@en)),
                annotation!(id!(unchecked dc_terms:description), rdf_str!("Codes pour la représentation des noms de pays et de leurs subdivisions — Partie 1: Codes de pays"@fr)),
                annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.iso.org/standard/72482.html")),
            ])
            .with_definitions([
                datatype!(
                    id!(unchecked AlphaTwoCode), idref!(unchecked xsd:string) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked iso_3166)),
                        annotation!(id!(unchecked xsd:pattern), rdf_str!("[A-Z]{2}")),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("alpha-2 code"@en)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("code alpha-2"@fr)),
                    ])).into(),
                datatype!(
                    id!(unchecked AlphaThreeCode), idref!(unchecked xsd:string) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked iso_3166)),
                        annotation!(id!(unchecked xsd:pattern), rdf_str!("[A-Z]{3}")),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("alpha-3 code"@en)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("code alpha-3"@fr)),
                    ])).into(),
                datatype!(
                    id!(unchecked NumericCode), idref!(unchecked xsd:nonNegativeInteger) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked iso_3166)),
                        annotation!(id!(unchecked xsd:minInclusive), v!(id!(unchecked xsd:nonNegativeInteger), 0)),
                        annotation!(id!(unchecked xsd:maxInclusive), v!(id!(unchecked xsd:nonNegativeInteger), 999)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("numeric-3 code"@en)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("code numérique-3"@fr)),
                    ])).into(),
                union!(
                    id!(unchecked CountryCode) ;
                    call |body: UnionBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked iso_3166)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("country code"@en)),
                    ])
                        .with_variants([
                            unvar!(id!(unchecked AlphaTwoCode)),
                            unvar!(id!(unchecked AlphaThreeCode)),
                            unvar!(id!(unchecked NumericCode)),
                        ])).into(),
            ])
    )
});
