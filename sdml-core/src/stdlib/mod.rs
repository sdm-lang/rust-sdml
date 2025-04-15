/*!
This module provides modules corresponding to the SDML standard library.
*/

use crate::model::{identifiers::Identifier, modules::Module};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_library_module_implementation(name: &Identifier) -> Option<&'static Module> {
    match name.as_ref() {
        dc::MODULE_NAME => Some(dc::module()),
        dcterms::MODULE_NAME => Some(dcterms::module()),
        iso_3166::MODULE_NAME => Some(iso_3166::module()),
        iso_4217::MODULE_NAME => Some(iso_4217::module()),
        owl::MODULE_NAME => Some(owl::module()),
        rdf::MODULE_NAME => Some(rdf::module()),
        rdfs::MODULE_NAME => Some(rdfs::module()),
        sdml::MODULE_NAME => Some(sdml::module()),
        skos::MODULE_NAME => Some(skos::module()),
        xsd::MODULE_NAME => Some(xsd::module()),
        _ => None,
    }
}

pub const PATH_ROOT_SEGMENT_ORG: &str = "org";
pub const PATH_ROOT_SEGMENT_IO: &str = "io";

pub const PATH_STDORG_SEGMENT_GS1: &str = "gs1";
pub const PATH_STDORG_SEGMENT_ISO: &str = "iso";
pub const PATH_STDORG_SEGMENT_W3C: &str = "w3";
pub const PATH_STDORG_SEGMENT_DC: &str = "dc";

pub const PATH_ORG_SEGMENT_PURL: &str = "purl";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
pub mod macros;

pub mod dc;
pub mod dcam;
pub mod dcterms;
pub mod dctype;
pub mod iso_3166;
pub mod iso_4217;
pub mod owl;
pub mod rdf;
pub mod rdfs;
pub mod sdml;
pub mod skos;
pub mod xsd;
