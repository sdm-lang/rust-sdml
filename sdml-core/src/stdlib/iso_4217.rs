/*
This Rust module contains the SDML model of the SDML library module `iso_3166`.
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

pub const MODULE_NAME: &str = "iso_4217";
pub const MODULE_URL: &str = "https://sdml.io/modules/iso4217/2020.ttl#";

pub const CURRENCY_CODE_ALPHA: &str = "CurrencyCodeAlpha";
pub const CURRENCY_CODE_NUMERIC: &str = "CurrencyCodeNumeric";
pub const CURRENCY_CODE: &str = "CurrencyCode";

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
            simple!(lstr!("ISO 4217:2015"))
        )
        .into(),
        //prop!(
        //    dc_terms::MODULE_NAME, dc_terms::PROP_ISSUED_NAME;
        //    simple!(lstr!("2015-08-01"))
        //).into(),
        //prop!(
        //    dc_terms::MODULE_NAME, dc_terms::PROP_HAS_VERSION_NAME;
        //    simple!(8_u64)
        //).into(),
        prop!(
            dc_terms::MODULE_NAME, dc_terms::DESCRIPTION;
            simple!(lstr!("Codes for the representation of currencies"@"en"))
        )
        .into(),
        prop!(
            dc_terms::MODULE_NAME, dc_terms::DESCRIPTION;
            simple!(lstr!("Codes pour la représentation des monnaies"@"fr"))
        )
        .into(),
        prop!(
            rdfs::MODULE_NAME, rdfs::SEE_ALSO;
            simple!(Url::parse("https://www.iso.org/iso-4217-currency-codes.html").unwrap())
        )
        .into(),
    ]);

    module.body_mut().extend_definitions(vec![
        datatype!(CURRENCY_CODE_ALPHA => xsd::MODULE_NAME, xsd::STRING)
            .with_body(
                vec![
                    prop!(
                        xsd::MODULE_NAME, xsd::PATTERN;
                        simple!(lstr!("[A-Z]{3}"))
                    )
                    .into(),
                    prop!(
                        skos::MODULE_NAME, skos::PREF_LABEL;
                        simple!(lstr!("alpha code"@"en"))
                    )
                    .into(),
                    prop!(
                        skos::MODULE_NAME, skos::PREF_LABEL;
                        simple!(lstr!("code alpha"@"fr"))
                    )
                    .into(),
                ]
                .into(),
            )
            .into(),
        datatype!(CURRENCY_CODE_NUMERIC => xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER)
            .with_body(
                vec![
                    prop!(
                        xsd::MODULE_NAME, xsd::MIN_INCLUSIVE;
                        tc!(xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER => 0_u64)
                    )
                    .into(),
                    prop!(
                        xsd::MODULE_NAME, xsd::MAX_INCLUSIVE;
                        tc!(xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER => 899_u64)
                    )
                    .into(),
                    prop!(
                        skos::MODULE_NAME, skos::PREF_LABEL;
                        simple!(lstr!("numeric code"@"en"))
                    )
                    .into(),
                    prop!(
                        skos::MODULE_NAME, skos::PREF_LABEL;
                        simple!(lstr!("code numérique"@"fr"))
                    )
                    .into(),
                ]
                .into(),
            )
            .into(),
        union!(CURRENCY_CODE => CURRENCY_CODE_ALPHA, CURRENCY_CODE_NUMERIC).into(),
    ]);

    module
}
