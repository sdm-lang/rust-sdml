/*
This Rust module contains the SDML model of the SDML library module `dc`.
*/

use crate::model::definitions::Definition;
use crate::model::HasBody;
use crate::model::{
    definitions::RdfDef,
    identifiers::Identifier,
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
    let rdfs_name = Identifier::new_unchecked(super::rdfs::MODULE_NAME);
    let mut module = Module::empty(Identifier::new_unchecked(MODULE_NAME))
        .with_base_uri(Url::parse(MODULE_URL).unwrap());

    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(rdfs_name.clone()));
    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(rdfs_name.clone()));

    let properties: Vec<Definition> = [
        PROP_CONTRIBUTOR_NAME,
        PROP_COVERAGE_NAME,
        PROP_CREATOR_NAME,
        PROP_DATE_NAME,
        PROP_DESCRIPTION_NAME,
        PROP_FORMAT_NAME,
        PROP_IDENTIFIER_NAME,
        PROP_LANGUAGE_NAME,
        PROP_PUBLISHER_NAME,
        PROP_RELATION_NAME,
        PROP_RIGHTS_NAME,
        PROP_SOURCE_NAME,
        PROP_SUBJECT_NAME,
        PROP_TITLE_NAME,
        PROP_TYPE_NAME,
    ]
    .into_iter()
    .map(|id| Definition::from(RdfDef::property(Identifier::new_unchecked(id))))
    .collect();
    module.body_mut().extend_definitions(properties);

    module
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod terms;
