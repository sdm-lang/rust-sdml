/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::model::{identifiers::Identifier, modules::Module};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME_DC: &str = "dc";
pub const MODULE_NAME_DC_TERMS: &str = "dc_terms";
pub const MODULE_NAME_OWL: &str = "owl";
pub const MODULE_NAME_RDF: &str = "rdf";
pub const MODULE_NAME_RDFS: &str = "rdfs";
pub const MODULE_NAME_SDML: &str = "sdml";
pub const MODULE_NAME_SKOS: &str = "skos";
pub const MODULE_NAME_XSD: &str = "xsd";

pub const MODULE_URL_DC: &str = "http://purl.org/dc/elements/1.1/";
pub const MODULE_URL_DC_TERMS: &str = "http://purl.org/dc/terms/";
pub const MODULE_URL_OWL: &str = "http://www.w3.org/2002/07/owl#";
pub const MODULE_URL_RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
pub const MODULE_URL_RDFS: &str = "http://www.w3.org/2000/01/rdf-schema#";
pub const MODULE_URL_SDML: &str = "http://sdml.io/sdml-owl.ttl#";
pub const MODULE_URL_SKOS: &str = "http://www.w3.org/2004/02/skos/core#";
pub const MODULE_URL_XSD: &str = "http://www.w3.org/2001/XMLSchema#";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn is_library_module(name: &Identifier) -> bool {
    [
        MODULE_NAME_DC,
        MODULE_NAME_DC_TERMS,
        MODULE_NAME_OWL,
        MODULE_NAME_RDF,
        MODULE_NAME_RDFS,
        MODULE_NAME_SDML,
        MODULE_NAME_SKOS,
        MODULE_NAME_XSD,
    ]
    .contains(&name.as_ref())
}

pub fn library_module(name: &Identifier) -> Option<Module> {
    match name.as_ref() {
        MODULE_NAME_DC => Some(dc()),
        MODULE_NAME_DC_TERMS => Some(dc_terms()),
        MODULE_NAME_OWL => Some(rdf()),
        MODULE_NAME_RDF => Some(rdf()),
        MODULE_NAME_RDFS => Some(rdfs()),
        MODULE_NAME_SDML => Some(sdml()),
        MODULE_NAME_SKOS => Some(skos()),
        MODULE_NAME_XSD => Some(xsd()),
        _ => None,
    }
}

pub fn dc() -> Module {
    Module::empty(Identifier::new_unchecked(MODULE_NAME_DC))
        .with_base(Url::parse(MODULE_URL_DC).unwrap())
}

pub fn dc_terms() -> Module {
    Module::empty(Identifier::new_unchecked(MODULE_NAME_DC_TERMS))
        .with_base(Url::parse(MODULE_URL_DC_TERMS).unwrap())
}

pub fn owl() -> Module {
    Module::empty(Identifier::new_unchecked(MODULE_NAME_OWL))
        .with_base(Url::parse(MODULE_URL_OWL).unwrap())
}

pub fn rdf() -> Module {
    Module::empty(Identifier::new_unchecked(MODULE_NAME_RDF))
        .with_base(Url::parse(MODULE_URL_RDF).unwrap())
}

pub fn rdfs() -> Module {
    Module::empty(Identifier::new_unchecked(MODULE_NAME_RDFS))
        .with_base(Url::parse(MODULE_URL_RDFS).unwrap())
}

pub fn sdml() -> Module {
    Module::empty(Identifier::new_unchecked(MODULE_NAME_SDML))
        .with_base(Url::parse(MODULE_URL_SDML).unwrap())
}

pub fn skos() -> Module {
    Module::empty(Identifier::new_unchecked(MODULE_NAME_SKOS))
        .with_base(Url::parse(MODULE_URL_SKOS).unwrap())
}

pub fn xsd() -> Module {
    Module::empty(Identifier::new_unchecked(MODULE_NAME_XSD))
        .with_base(Url::parse(MODULE_URL_XSD).unwrap())
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
