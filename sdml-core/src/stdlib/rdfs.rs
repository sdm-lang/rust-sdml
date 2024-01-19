/*!
Standard library module for namespace `rdfs`.

*/

use crate::model::{identifiers::Identifier, modules::{ImportStatement, Module}, definitions::{RdfDef, RdfDefBody}};
use url::Url;
use crate::model::HasBody;
use crate::stdlib::rdf::MODULE_NAME as RDF_MODULE_NAME;

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
    let mut module = Module::empty(Identifier::new_unchecked(MODULE_NAME))
        .with_base_uri(Url::parse(MODULE_URL).unwrap());

    module.body_mut().add_to_imports(ImportStatement::new_module(Identifier::new_unchecked(RDF_MODULE_NAME)));

    module.body_mut().extend_definitions(vec![
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_CLASS_NAME))
                .with_type(Identifier::new_unchecked(CLASS_CLASS_NAME).into())
        ).into(),
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_CONTAINER_NAME))
        ).into(),
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_CONTAINER_MEMBERSHIP_PROPERTY_NAME))
        ).into(),
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_DATATYPE_NAME))
                .with_type(Identifier::new_unchecked(CLASS_CLASS_NAME).into())
                .with_super_class(Identifier::new_unchecked(CLASS_CLASS_NAME).into())
        ).into(),
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_LITERAL_NAME))
                .with_type(Identifier::new_unchecked(CLASS_CLASS_NAME).into())
                .with_super_class(Identifier::new_unchecked(CLASS_RESOURCE_NAME).into())
        ).into(),
        RdfDef::Class(
            RdfDefBody::new(Identifier::new_unchecked(CLASS_RESOURCE_NAME))
                .with_type(Identifier::new_unchecked(CLASS_CLASS_NAME).into())
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_COMMENT_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_DOMAIN_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_IS_DEFINED_BY_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_LABEL_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_MEMBER_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_RANGE_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_SEE_ALSO_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_SUB_CLASS_OF_NAME))
        ).into(),
        RdfDef::Property(
            RdfDefBody::new(Identifier::new_unchecked(PROP_SUB_PROPERTY_OF_NAME))
        ).into(),
    ]);

    module
}
