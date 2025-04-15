/*!
This Rust module contains the SDML model of the SDML library module `iso_3166` for ISO-4217.
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

pub const MODULE_NAME: &str = "iso_4217";
library_module_url! { "iso", "4217-2020" }

pub const MODULE_SRC: &str = include_str!("org/iso/iso_4217.sdml");

pub const CURRENCY_CODE_ALPHA: &str = "CurrencyCodeAlpha";
pub const CURRENCY_CODE_NUMERIC: &str = "CurrencyCodeNumeric";
pub const CURRENCY_CODE: &str = "CurrencyCode";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

module_function!(|| {
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked iso_4217), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked dc_terms),
            id!(unchecked rdfs),
            id!(unchecked skos),
            id!(unchecked xsd),
        )])
            .with_annotations([
                 annotation!(id!(unchecked skos:prefLabel), rdf_str!("ISO 4217:2015"@en)),
                 annotation!(id!(unchecked dc_terms:description), rdf_str!("Codes for the representation of currencies"@en)),
                 annotation!(id!(unchecked dc_terms:description), rdf_str!("Codes pour la représentation des monnaies"@fr)),
                annotation!(id!(unchecked rdfs:seeAlso), url!("https://www.iso.org/iso-4217-currency-codes.html")),
            ])
            .with_definitions([
                // ---------------------------------------------------------------------------------
                // Datatypes
                // ---------------------------------------------------------------------------------
                datatype!(
                    id!(unchecked AlphaCode), idref!(unchecked xsd:string) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked iso_4217)),
                        annotation!(id!(unchecked xsd:pattern), rdf_str!("[A-Z]{2}")),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("alpha code"@en)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("code alpha"@fr)),
                    ])).into(),
                datatype!(
                    id!(unchecked NumericCode), idref!(unchecked xsd:nonNegativeInteger) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked iso_4217)),
                        annotation!(id!(unchecked xsd:minInclusive), v!(id!(unchecked xsd:nonNegativeInteger), 0)),
                        annotation!(id!(unchecked xsd:maxInclusive), v!(id!(unchecked xsd:nonNegativeInteger), 899)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("numeric code"@en)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("code numérique"@fr)),
                    ])).into(),
                union!(
                    id!(unchecked CurrencyCode) ;
                    call |body: UnionBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked iso_4217)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("currency code"@en)),
                    ])
                        .with_variants([
                            unvar!(id!(unchecked AlphaCode)),
                            unvar!(id!(unchecked NumericCode)),
                        ])).into(),
            ])
    )
});
