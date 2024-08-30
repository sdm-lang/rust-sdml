/*!
This Rust module contains the SDML model of the SDML library module `iso_3166` for ISO-3166.
*/

use crate::model::annotations::HasAnnotations;
use crate::model::modules::Module;
use crate::model::HasBody;
use crate::stdlib::{dc::terms as dc_terms, rdfs, skos, xsd};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "iso_3166";
pub const MODULE_URL: &str = "https://sdml.io/modules/iso3166/2020.ttl#";

pub const COUNTRY_CODE_ALPHA_2: &str = "CountryCodeAlpha2";
pub const COUNTRY_CODE_ALPHA_3: &str = "CountryCodeAlpha3";
pub const COUNTRY_CODE_NUMERIC_3: &str = "CountryCodeNumeric3";
pub const COUNTRY_CODE: &str = "CountryCode";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(Url::parse(MODULE_URL).unwrap());

    module.body_mut().add_to_imports(import!(
        id!(dc_terms::MODULE_NAME),
        id!(rdfs::MODULE_NAME),
        id!(skos::MODULE_NAME),
        id!(xsd::MODULE_NAME)
    ));

    module.body_mut().extend_annotations(vec![
        prop!(
            skos::MODULE_NAME, skos::PREF_LABEL;
            simple!(lstr!("ISO 3166-1:2020"))
        ).into(),
        //prop!(
        //    dc_terms::MODULE_NAME, dc_terms::PROP_ISSUED_NAME;
        //    simple!(lstr!("ISO 3166-1:2020"))
        //).into(),
        //prop!(
        //    dc_terms::MODULE_NAME, dc_terms::PROP_HAS_VERSION_NAME;
        //    simple!(4_u64)
        //).into(),
        //prop!(
        //    dc_terms::MODULE_NAME, dc_terms::PROP_REPLACES_NAME;
        //    simple!(lstr!("ISO 3166-1:2013"))
        //).into(),
        prop!(
            dc_terms::MODULE_NAME, dc_terms::DESCRIPTION;
            simple!(lstr!("Codes for the representation of names of countries and their subdivisions — Part 1: Country code"@"en"))
        ).into(),
        prop!(
            dc_terms::MODULE_NAME, dc_terms::DESCRIPTION;
            simple!(lstr!("Codes pour la représentation des noms de pays et de leurs subdivisions — Partie 1: Codes de pays"@"fr"))
        ).into(),
        prop!(
            rdfs::MODULE_NAME, rdfs::SEE_ALSO;
            simple!(Url::parse("https://www.iso.org/standard/72482.html").unwrap())
        ).into(),
    ]);

    module
        .body_mut()
        .extend_definitions(vec![
        datatype!(COUNTRY_CODE_ALPHA_2 => xsd::MODULE_NAME, xsd::STRING)
            .with_body(
                vec![
                    prop!(
                        xsd::MODULE_NAME, xsd::PATTERN;
                        simple!(lstr!("[A-Z]{2}"))
                    )
                    .into(),
                    prop!(
                        skos::MODULE_NAME, skos::PREF_LABEL;
                        simple!(lstr!("alpha-2 code"@"en"))
                    )
                    .into(),
                    prop!(
                        skos::MODULE_NAME, skos::PREF_LABEL;
                        simple!(lstr!("code alpha-2"@"fr"))
                    )
                    .into(),
                ]
                .into(),
            )
            .into(),
        datatype!(COUNTRY_CODE_ALPHA_3 => xsd::MODULE_NAME, xsd::STRING)
            .with_body(
                vec![
                    prop!(
                        xsd::MODULE_NAME, xsd::PATTERN;
                        simple!(lstr!("[A-Z]{3}"))
                    )
                    .into(),
                    prop!(
                        skos::MODULE_NAME, skos::PREF_LABEL;
                        simple!(lstr!("alpha-3 code"@"en"))
                    )
                    .into(),
                    prop!(
                        skos::MODULE_NAME, skos::PREF_LABEL;
                        simple!(lstr!("code alpha-3"@"fr"))
                    )
                    .into(),
                ]
                .into(),
            )
            .into(),
        datatype!(COUNTRY_CODE_NUMERIC_3 => xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER)
            .with_body(
                vec![
                    prop!(
                        xsd::MODULE_NAME, xsd::MIN_INCLUSIVE;
                        tc!(xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER => 0_u64)
                    )
                    .into(),
                    prop!(
                        xsd::MODULE_NAME, xsd::MAX_INCLUSIVE;
                        tc!(xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER => 999_u64)
                    )
                    .into(),
                    prop!(
                        skos::MODULE_NAME, skos::PREF_LABEL;
                        simple!(lstr!("numeric-3 code"@"en"))
                    )
                    .into(),
                    prop!(
                        skos::MODULE_NAME, skos::PREF_LABEL;
                        simple!(lstr!("code numérique-3"@"fr"))
                    )
                    .into(),
                ]
                .into(),
            )
            .into(),
        union!(COUNTRY_CODE => COUNTRY_CODE_ALPHA_2, COUNTRY_CODE_ALPHA_3, COUNTRY_CODE_NUMERIC_3)
            .into(),
    ])
        .unwrap();

    module
}
