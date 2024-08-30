/*!
This Rust module contains the SDML model of the SDML library module `sdml`.
*/

use crate::model::annotations::AnnotationBuilder;
use crate::model::modules::Module;
use crate::model::HasBody;
use crate::stdlib::{owl, rdf, rdfs, skos, xsd};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "sdml";
pub const MODULE_URL: &str = "http://sdml.io/sdml-owl.ttl#";

pub const ANNOTATION: &str = "Annotation";
pub const ANNOTATION_PROPERTY: &str = "AnnotationProperty";
pub const CARDINALITY: &str = "Cardinality";
pub const CONSTRAINT: &str = "Constraint";
pub const DEFINITION: &str = "Definition";
pub const ENTITY: &str = "Entity";
pub const ENUMERATION: &str = "Enumeration";
pub const EVENT: &str = "Event";
pub const FEATURE_SET: &str = "FeatureSet";
pub const FORMAL_CONSTRAINT: &str = "FormalConstraint";
pub const IDENTIFIER_REFERENCE: &str = "IdentifierReference";
pub const IMPORT: &str = "Import";
pub const IMPORT_STATEMENT: &str = "ImportStatement";
pub const INFORMAL_CONSTRAINT: &str = "InformalConstraint";
pub const CLASS_MAP_TYPE_NAME: &str = "MapType";
pub const MEMBER: &str = "Member";
pub const MEMBER_IMPORT: &str = "MemberImport";
pub const MODULE: &str = "Module";
pub const MODULE_IMPORT: &str = "ModuleImport";
pub const ORDERING_CONSTRAINT: &str = "OrderingConstraint";
pub const PROPERTY: &str = "Property";
pub const ROLE: &str = "Role";
pub const ROLE_REFERENCE: &str = "RoleReference";
pub const QUALIFIED_IDENTIFIER: &str = "QualifiedIdentifier";
pub const STRUCTURE: &str = "Structure";
pub const TYPE_CLASS: &str = "TypeClass";
pub const TYPE_VARIANT: &str = "TypeVariant";
pub const UNION: &str = "Union";
pub const UNKNOWN: &str = "Unknown";
pub const UNIQUENESS_CONSTRAINT: &str = "UniquenessConstraint";
pub const VALUE_VARIANT: &str = "ValueVariant";

pub const IDENTIFIER: &str = "Identifier";

pub const BINARY: &str = "binary";
pub const BOOLEAN: &str = "boolean";
pub const DECIMAL: &str = "decimal";
pub const DOUBLE: &str = "double";
pub const INTEGER: &str = "integer";
pub const IRI: &str = "iri";
pub const LANGUAGE: &str = "language";
pub const STRING: &str = "string";
pub const UNSIGNED: &str = "unsigned";

pub const HAS_NAME: &str = "hasName";
pub const HAS_ANNOTATION: &str = "hasAnnotation";
pub const HAS_CARDINALITY: &str = "hasCardinality";
pub const HAS_DEFINITION: &str = "hasDefinition";
pub const HAS_DOMAIN_VALUE: &str = "hasDomainValue";
pub const HAS_IMPORT_STATEMENT: &str = "hasImportStatement";
pub const HAS_MEMBER: &str = "hasMember";
pub const HAS_RANGE_VALUE: &str = "hasRangeValue";
pub const HAS_SOURCE_ENTITY: &str = "hasSourceEntity";
pub const HAS_TYPE_VARIANT: &str = "hasTypeVariant";
pub const HAS_VALUE_VARIANT: &str = "hasValueVariant";
pub const MAX_OCCURS: &str = "maxOccurs";
pub const MIN_OCCURS: &str = "minOccurs";
pub const ORDERING: &str = "ordering";
pub const SRC_LABEL: &str = "srcLabel";
pub const UNIQUENESS: &str = "uniqueness";

pub const ORDERED: &str = "Ordered";
pub const UNIQUE: &str = "Unique";
pub const NONUNIQUE: &str = "NonUnique";
pub const UNORDERED: &str = "Unordered";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(Url::parse(MODULE_URL).unwrap());

    module.body_mut().add_to_imports(import!(
        id!(owl::MODULE_NAME),
        id!(rdf::MODULE_NAME),
        id!(rdfs::MODULE_NAME),
        id!(skos::MODULE_NAME),
        id!(xsd::MODULE_NAME)
    ));

    module
        .body_mut()
        .extend_definitions(vec![
            // Classes
            rdf!(class ANNOTATION, MODULE_IRI).into(),
            rdf!(class ANNOTATION_PROPERTY, MODULE_IRI).into(),
            rdf!(class CARDINALITY, MODULE_IRI).into(),
            rdf!(class CONSTRAINT, MODULE_IRI).into(),
            rdf!(class DEFINITION, MODULE_IRI).into(),
            rdf!(class ENTITY, MODULE_IRI).into(),
            rdf!(class ENUMERATION, MODULE_IRI).into(),
            rdf!(class EVENT, MODULE_IRI).into(),
            rdf!(class FEATURE_SET, MODULE_IRI).into(), // subClassOf :Union
            rdf!(class FORMAL_CONSTRAINT, MODULE_IRI).into(),
            rdf!(class IDENTIFIER_REFERENCE, MODULE_IRI).into(),
            rdf!(class IMPORT, MODULE_IRI).into(),
            rdf!(class IMPORT_STATEMENT, MODULE_IRI).into(),
            rdf!(class INFORMAL_CONSTRAINT, MODULE_IRI).into(),
            rdf!(class ORDERING_CONSTRAINT, MODULE_IRI).into(),
            rdf!(class PROPERTY, MODULE_IRI).into(),
            rdf!(class ROLE, MODULE_IRI).into(),
            rdf!(class ROLE_REFERENCE, MODULE_IRI).into(),
            rdf!(class QUALIFIED_IDENTIFIER, MODULE_IRI).into(),
            rdf!(class STRUCTURE, MODULE_IRI).into(),
            rdf!(class TYPE_CLASS, MODULE_IRI).into(),
            rdf!(class TYPE_VARIANT, MODULE_IRI).into(),
            rdf!(class UNION, MODULE_IRI).into(),
            rdf!(class UNKNOWN, MODULE_IRI).into(), // subClassOf owl:Nothing
            rdf!(class UNIQUENESS_CONSTRAINT, MODULE_IRI).into(),
            rdf!(class VALUE_VARIANT, MODULE_IRI).into(),
            // Data types
            rdf!(datatype IDENTIFIER, MODULE_IRI).into(),
            rdf!(datatype BINARY, MODULE_IRI)
                .with_equivalent_class(qualid!(xsd::MODULE_NAME, xsd::HEX_BINARY))
                .into(),
            rdf!(datatype BOOLEAN, MODULE_IRI)
                .with_equivalent_class(qualid!(xsd::MODULE_NAME, xsd::BOOLEAN))
                .into(),
            rdf!(datatype DECIMAL, MODULE_IRI)
                .with_equivalent_class(qualid!(xsd::MODULE_NAME, xsd::DECIMAL))
                .into(),
            rdf!(datatype DOUBLE, MODULE_IRI)
                .with_equivalent_class(qualid!(xsd::MODULE_NAME, xsd::DOUBLE))
                .into(),
            rdf!(datatype INTEGER, MODULE_IRI)
                .with_equivalent_class(qualid!(xsd::MODULE_NAME, xsd::INTEGER))
                .into(),
            rdf!(datatype UNSIGNED, MODULE_IRI)
                .with_equivalent_class(qualid!(xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER))
                .into(),
            rdf!(datatype IRI, MODULE_IRI)
                .with_equivalent_class(qualid!(xsd::MODULE_NAME, xsd::ANY_URI))
                .into(),
            rdf!(datatype STRING, MODULE_IRI)
                .with_equivalent_class(qualid!(xsd::MODULE_NAME, xsd::STRING))
                .into(),
            rdf!(datatype LANGUAGE, MODULE_IRI)
                .with_equivalent_class(qualid!(xsd::MODULE_NAME, xsd::LANGUAGE))
                .into(),
            // Properties
            rdf!(property HAS_ANNOTATION, MODULE_IRI).into(),
            rdf!(property HAS_CARDINALITY, MODULE_IRI).into(),
            rdf!(property HAS_DEFINITION, MODULE_IRI).into(),
            rdf!(property HAS_DOMAIN_VALUE, MODULE_IRI).into(),
            rdf!(property HAS_IMPORT_STATEMENT, MODULE_IRI).into(),
            rdf!(property HAS_MEMBER, MODULE_IRI).into(),
            rdf!(property HAS_NAME, MODULE_IRI).into(),
            rdf!(property HAS_RANGE_VALUE, MODULE_IRI).into(),
            rdf!(property HAS_TYPE_VARIANT, MODULE_IRI).into(),
            rdf!(property HAS_VALUE_VARIANT, MODULE_IRI).into(),
            rdf!(property MAX_OCCURS, MODULE_IRI).into(),
            rdf!(property MIN_OCCURS, MODULE_IRI).into(),
            rdf!(property ORDERING, MODULE_IRI).into(),
            rdf!(property SRC_LABEL, MODULE_IRI).into(),
            rdf!(property UNIQUENESS, MODULE_IRI).into(),
            // Individuals
            rdf!(thing ORDERED, MODULE_IRI, ORDERING_CONSTRAINT).into(),
            rdf!(thing NONUNIQUE, MODULE_IRI, UNIQUENESS_CONSTRAINT).into(),
            rdf!(thing UNIQUE, MODULE_IRI, UNIQUENESS_CONSTRAINT).into(),
            rdf!(thing UNORDERED, MODULE_IRI, ORDERING_CONSTRAINT).into(),
        ])
        .unwrap();

    module
}
