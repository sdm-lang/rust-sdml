/*!
Standard library module for namespace `rdfs`.

*/

use crate::model::modules::{ImportStatement, Module};
use crate::model::HasBody;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "rdfs";
pub const MODULE_URL: &str = "http://www.w3.org/2000/01/rdf-schema#";

pub const CLASS_CLASS_NAME: &str = "Class";
pub const CLASS_CONTAINER_NAME: &str = "Container";
pub const CLASS_CONTAINER_MEMBERSHIP_PROPERTY_NAME: &str = "ContainerMembershipProperty";
pub const CLASS_DATATYPE_NAME: &str = "Datatype";
pub const CLASS_LITERAL_NAME: &str = "Literal";
pub const CLASS_RESOURCE_NAME: &str = "Resource";
pub const PROP_COMMENT_NAME: &str = "comment";
pub const PROP_DOMAIN_NAME: &str = "domain";
pub const PROP_IS_DEFINED_BY_NAME: &str = "isDefinedBy";
pub const PROP_LABEL_NAME: &str = "label";
pub const PROP_MEMBER_NAME: &str = "member";
pub const PROP_RANGE_NAME: &str = "range";
pub const PROP_SEE_ALSO_NAME: &str = "seeAlso";
pub const PROP_SUB_CLASS_OF_NAME: &str = "subClassOf";
pub const PROP_SUB_PROPERTY_OF_NAME: &str = "subPropertyOf";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(MODULE_IRI.clone());

    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(id!(super::rdf::MODULE_NAME)));

    module.body_mut().extend_definitions(vec![
        // Classes
        rdf!(class CLASS_CLASS_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_CONTAINER_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_DATATYPE_NAME, MODULE_IRI; CLASS_CLASS_NAME).into(),
        rdf!(class CLASS_LITERAL_NAME, MODULE_IRI; CLASS_RESOURCE_NAME).into(),
        rdf!(class CLASS_RESOURCE_NAME, MODULE_IRI).into(),

        // Individuals
        rdf!(thing CLASS_CONTAINER_MEMBERSHIP_PROPERTY_NAME, MODULE_IRI;
             (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME))
            .into(),

        // Properties
        rdf!(property PROP_COMMENT_NAME, MODULE_IRI; CLASS_RESOURCE_NAME => CLASS_LITERAL_NAME)
            .into(),
        rdf!(property PROP_DOMAIN_NAME, MODULE_IRI;
             (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME) => CLASS_CLASS_NAME)
            .into(),
        rdf!(property PROP_IS_DEFINED_BY_NAME, MODULE_IRI; CLASS_RESOURCE_NAME => CLASS_RESOURCE_NAME)
            .into(),
        rdf!(property PROP_LABEL_NAME, MODULE_IRI; CLASS_RESOURCE_NAME => CLASS_LITERAL_NAME)
            .into(),
        rdf!(property PROP_MEMBER_NAME, MODULE_IRI; CLASS_RESOURCE_NAME => CLASS_RESOURCE_NAME)
            .into(),
        rdf!(property PROP_RANGE_NAME, MODULE_IRI;
             (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME) => CLASS_CLASS_NAME)
            .into(),
        rdf!(property PROP_SEE_ALSO_NAME, MODULE_IRI; CLASS_RESOURCE_NAME => CLASS_RESOURCE_NAME)
            .into(),
        rdf!(property PROP_SUB_CLASS_OF_NAME, MODULE_IRI; CLASS_CLASS_NAME => CLASS_CLASS_NAME)
            .into(),
        rdf!(property PROP_SUB_PROPERTY_OF_NAME, MODULE_IRI;
             (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME) =>
             (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME))
            .into(),
    ]);

    module
}
