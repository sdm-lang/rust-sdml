/*!
This Rust module contains the SDML model of the SDML library module `rdfs` for RDF Schema.
*/

use crate::model::{
    annotations::{AnnotationOnlyBody, HasAnnotations},
    modules::Module,
    HasBody,
};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_PATH: &str = "::org::w3";
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

module_function!(|| {
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked rdfs), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked rdf),
        )])
            .with_definitions([
                // ---------------------------------------------------------------------------------
                // Classes
                // ---------------------------------------------------------------------------------
                rdf!(id!(unchecked Class) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(Class@en)),
                    ])).into(),
                rdf!(id!(unchecked Container) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(Container@en)),
                    ])).into(),
                rdf!(id!(unchecked Datatype) ; class id!(unchecked Class) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(Datatype@en)),
                    ])).into(),
                rdf!(id!(unchecked Literal) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(Literal@en)),
                    ])).into(),
                rdf!(id!(unchecked Resource) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(Resource@en)),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Properties
                // ---------------------------------------------------------------------------------
                rdf!(id!(unchecked comment) ; property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(comment@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Literal)),
                    ])).into(),
                rdf!(id!(unchecked domain) ; property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(domain@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdf:Property)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Class)),
                    ])).into(),
                rdf!(id!(unchecked isDefinedBy) ; property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(isDefinedBy@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Resource)),
                    ])).into(),
                rdf!(id!(unchecked label) ; property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(label@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Literal)),
                    ])).into(),
                rdf!(id!(unchecked range) ; property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(range@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdf:Property)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Class)),
                    ])).into(),
                rdf!(id!(unchecked member) ; property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(member@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Resource)),
                    ])).into(),
                rdf!(id!(unchecked seeAlso) ; property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(seeAlso@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Resource)),
                    ])).into(),
                rdf!(id!(unchecked subPropertyOf) ; property id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(subPropertyOf@en)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdf:Property)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:Property)),
                    ])).into(),
                rdf!(id!(unchecked ContainerMembershipProperty) ; class id!(unchecked Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked rdfs)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!(subPropertyOfContainerMembershipProperty@en)),
                    ])).into(),
            ])
    )
});
