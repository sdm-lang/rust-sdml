/*!
Standard library module for namespace `rdfs`.

*/

use crate::model::annotations::AnnotationBuilder;
use crate::model::modules::Module;
use crate::model::HasBody;
use crate::stdlib::rdf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "rdfs";
pub const MODULE_URL: &str = "http://www.w3.org/2000/01/rdf-schema#";

pub const CLASS: &str = "Class";
pub const CONTAINER: &str = "Container";
pub const CONTAINER_MEMBERSHIP_PROPERTY: &str = "ContainerMembershipProperty";
pub const DATATYPE: &str = "Datatype";
pub const LITERAL: &str = "Literal";
pub const RESOURCE: &str = "Resource";

pub const COMMENT: &str = "comment";
pub const DOMAIN: &str = "domain";
pub const IS_DEFINED_BY: &str = "isDefinedBy";
pub const LABEL: &str = "label";
pub const MEMBER: &str = "member";
pub const RANGE: &str = "range";
pub const SEE_ALSO: &str = "seeAlso";
pub const SUB_CLASS_OF: &str = "subClassOf";
pub const SUB_PROPERTY_OF: &str = "subPropertyOf";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(MODULE_IRI.clone());

    module
        .body_mut()
        .add_to_imports(import!(id!(rdf::MODULE_NAME)));

    module.body_mut().extend_definitions(vec![
        // Classes
        rdf!(class CLASS, MODULE_IRI).into(),
        rdf!(class CONTAINER, MODULE_IRI).into(),
        rdf!(class DATATYPE, MODULE_IRI; CLASS).into(),
        rdf!(class LITERAL, MODULE_IRI; RESOURCE).into(),
        rdf!(class RESOURCE, MODULE_IRI).into(),
        // Individuals
        rdf!(thing CONTAINER_MEMBERSHIP_PROPERTY, MODULE_IRI;
             (rdf::MODULE_NAME, rdf::PROPERTY))
        .into(),
        // Properties
        rdf!(property COMMENT, MODULE_IRI; RESOURCE => LITERAL).into(),
        rdf!(property DOMAIN, MODULE_IRI;
             (rdf::MODULE_NAME, rdf::PROPERTY) => CLASS)
        .into(),
        rdf!(property IS_DEFINED_BY, MODULE_IRI; RESOURCE => RESOURCE).into(),
        rdf!(property LABEL, MODULE_IRI; RESOURCE => LITERAL).into(),
        rdf!(property MEMBER, MODULE_IRI; RESOURCE => RESOURCE).into(),
        rdf!(property RANGE, MODULE_IRI;
             (rdf::MODULE_NAME, rdf::PROPERTY) => CLASS)
        .into(),
        rdf!(property SEE_ALSO, MODULE_IRI; RESOURCE => RESOURCE).into(),
        rdf!(property SUB_CLASS_OF, MODULE_IRI; CLASS => CLASS).into(),
        rdf!(property SUB_PROPERTY_OF, MODULE_IRI;
             (rdf::MODULE_NAME, rdf::PROPERTY) =>
             (rdf::MODULE_NAME, rdf::PROPERTY))
        .into(),
    ]);

    module
}
