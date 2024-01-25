/*
This Rust module contains the SDML model of the SDML library module `sdml`.
*/

use crate::model::annotations::AnnotationBuilder;
use crate::model::modules::Module;
use crate::model::HasBody;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "sdml";
pub const MODULE_URL: &str = "http://sdml.io/sdml-owl.ttl#";

pub const CLASS_ANNOTATION_NAME: &str = "Annotation";
pub const CLASS_ANNOTATION_PROPERTY_NAME: &str = "AnnotationProperty";
pub const CLASS_CARDINALITY_NAME: &str = "Cardinality";
pub const CLASS_CONSTRAINT_NAME: &str = "Constraint";
pub const CLASS_DEFINITION_NAME: &str = "Definition";
pub const CLASS_ENTITY_NAME: &str = "Entity";
pub const CLASS_ENUMERATION_NAME: &str = "Enumeration";
pub const CLASS_EVENT_NAME: &str = "Event";
pub const CLASS_FORMAL_CONSTRAINT_NAME: &str = "FormalConstraint";
pub const CLASS_IDENTIFIER_REFERENCE_NAME: &str = "IdentifierReference";
pub const CLASS_IMPORT_NAME: &str = "Import";
pub const CLASS_IMPORT_STATEMENT_NAME: &str = "ImportStatement";
pub const CLASS_INFORMAL_CONSTRAINT_NAME: &str = "InformalConstraint";
pub const CLASS_MEMBER_NAME: &str = "Member";
pub const CLASS_MEMBER_IMPORT_NAME: &str = "MemberImport";
pub const CLASS_MODULE_NAME: &str = "Module";
pub const CLASS_MODULE_IMPORT_NAME: &str = "ModuleImport";
pub const CLASS_PROPERTY_NAME: &str = "Property";
pub const CLASS_ROLE_NAME: &str = "Role";
pub const CLASS_QUALIFIED_IDENTIFIER_NAME: &str = "QualifiedIdentifier";
pub const CLASS_STRUCTURE_NAME: &str = "Structure";
pub const CLASS_TYPE_VARIANT_NAME: &str = "TypeVariant";
pub const CLASS_UNION_NAME: &str = "Union";
pub const CLASS_VALUE_VARIANT_NAME: &str = "ValueVariant";

pub const DT_IDENTIFIER_NAME: &str = "Identifier";

pub const DT_BINARY_NAME: &str = "binary";
pub const DT_BOOLEAN_NAME: &str = "boolean";
pub const DT_DECIMAL_NAME: &str = "decimal";
pub const DT_DOUBLE_NAME: &str = "double";
pub const DT_INTEGER_NAME: &str = "string";
pub const DT_IRI_NAME: &str = "iri";
pub const DT_LANGUAGE_NAME: &str = "language";
pub const DT_STRING_NAME: &str = "string";
pub const DT_UNSIGNED_NAME: &str = "unsigned";

pub const PROP_HAS_NAME_NAME: &str = "hasName";
pub const PROP_HAS_ANNOTATION_NAME: &str = "hasAnnotation";
pub const PROP_HAS_CARDINALITY_NAME: &str = "hasCardinality";
pub const PROP_HAS_DEFINITION_NAME: &str = "hasDefinition";
pub const PROP_HAS_IMPORT_STATEMENT_NAME: &str = "hasImportStatement";
pub const PROP_HAS_MEMBER_NAME: &str = "hasMember";
pub const PROP_HAS_TYPE_VARIANT_NAME: &str = "hasTypeVariant";
pub const PROP_HAS_VALUE_VARIANT_NAME: &str = "hasValueVariant";

pub const PROP_MAX_OCCURS_NAME: &str = "maxOccurs";
pub const PROP_MIN_OCCURS_NAME: &str = "minOccurs";
pub const PROP_ORDERING_NAME: &str = "ordering";
pub const PROP_UNIQUENESS_NAME: &str = "uniqueness";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(Url::parse(MODULE_URL).unwrap());

    module
        .body_mut()
        .add_to_imports(
            import!(
                id!(super::owl::MODULE_NAME),
                id!(super::rdf::MODULE_NAME),
                id!(super::rdfs::MODULE_NAME),
                id!(super::skos::MODULE_NAME),
                id!(super::xsd::MODULE_NAME)
            )
        );

    module.body_mut().extend_definitions(vec![
        // Classes
        rdf!(class CLASS_ANNOTATION_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_ANNOTATION_PROPERTY_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_CARDINALITY_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_CONSTRAINT_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_DEFINITION_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_ENTITY_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_ENUMERATION_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_EVENT_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_FORMAL_CONSTRAINT_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_IDENTIFIER_REFERENCE_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_IMPORT_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_IMPORT_STATEMENT_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_INFORMAL_CONSTRAINT_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_MEMBER_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_MEMBER_IMPORT_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_MODULE_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_MODULE_IMPORT_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_PROPERTY_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_ROLE_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_QUALIFIED_IDENTIFIER_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_STRUCTURE_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_TYPE_VARIANT_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_UNION_NAME, MODULE_IRI).into(),
        rdf!(class CLASS_VALUE_VARIANT_NAME, MODULE_IRI).into(),
        // Data types
        rdf!(datatype DT_IDENTIFIER_NAME, MODULE_IRI).into(),
        rdf!(datatype DT_BINARY_NAME, MODULE_IRI)
            .with_equivalent_class(qualid!(
                super::xsd::MODULE_NAME,
                super::xsd::DT_HEX_BINARY_NAME
            ))
            .into(),
        rdf!(datatype DT_BOOLEAN_NAME, MODULE_IRI)
            .with_equivalent_class(qualid!(
                super::xsd::MODULE_NAME,
                super::xsd::DT_BOOLEAN_NAME
            ))
            .into(),
        rdf!(datatype DT_DECIMAL_NAME, MODULE_IRI)
            .with_equivalent_class(qualid!(
                super::xsd::MODULE_NAME,
                super::xsd::DT_DECIMAL_NAME
            ))
            .into(),
        rdf!(datatype DT_DOUBLE_NAME, MODULE_IRI)
            .with_equivalent_class(qualid!(super::xsd::MODULE_NAME, super::xsd::DT_DOUBLE_NAME))
            .into(),
        rdf!(datatype DT_INTEGER_NAME, MODULE_IRI)
            .with_equivalent_class(qualid!(
                super::xsd::MODULE_NAME,
                super::xsd::DT_INTEGER_NAME
            ))
            .into(),
        rdf!(datatype DT_UNSIGNED_NAME, MODULE_IRI)
            .with_equivalent_class(qualid!(
                super::xsd::MODULE_NAME,
                super::xsd::DT_NONNEGATIVE_INTEGER_NAME
            ))
            .into(),
        rdf!(datatype DT_IRI_NAME, MODULE_IRI)
            .with_equivalent_class(qualid!(
                super::xsd::MODULE_NAME,
                super::xsd::DT_ANY_URI_NAME
            ))
            .into(),
        rdf!(datatype DT_LANGUAGE_NAME, MODULE_IRI)
            .with_equivalent_class(qualid!(
                super::xsd::MODULE_NAME,
                super::xsd::DT_LANGUAGE_NAME
            ))
            .into(),
        // Properties
        rdf!(property PROP_HAS_ANNOTATION_NAME, MODULE_IRI).into(),
        rdf!(property PROP_HAS_CARDINALITY_NAME, MODULE_IRI).into(),
        rdf!(property PROP_HAS_DEFINITION_NAME, MODULE_IRI).into(),
        rdf!(property PROP_HAS_IMPORT_STATEMENT_NAME, MODULE_IRI).into(),
        rdf!(property PROP_HAS_MEMBER_NAME, MODULE_IRI).into(),
        rdf!(property PROP_HAS_NAME_NAME, MODULE_IRI).into(),
        rdf!(property PROP_HAS_TYPE_VARIANT_NAME, MODULE_IRI).into(),
        rdf!(property PROP_HAS_VALUE_VARIANT_NAME, MODULE_IRI).into(),
        rdf!(property PROP_MAX_OCCURS_NAME, MODULE_IRI).into(),
        rdf!(property PROP_MIN_OCCURS_NAME, MODULE_IRI).into(),
        rdf!(property PROP_ORDERING_NAME, MODULE_IRI).into(),
        rdf!(property PROP_UNIQUENESS_NAME, MODULE_IRI).into(),
    ]);

    module
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
