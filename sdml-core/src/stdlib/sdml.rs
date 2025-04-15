/*!
This Rust module contains the SDML model of the SDML library module `sdml`.
*/

use crate::model::annotations::{AnnotationOnlyBody, HasAnnotations};
use crate::model::modules::Module;
use crate::model::HasBody;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "sdml";
pub const MODULE_URL: &str = "http://sdml.io/sdml-owl.ttl#";

/* Classes */
pub const ANNOTATION_PROPERTY: &str = "AnnotationProperty";
pub const CARDINALITY: &str = "Cardinality";
pub const MAP_TYPE: &str = "MapType";
pub const CONSTRAINT: &str = "Constraint";
pub const DEFINITION: &str = "Definition";
pub const DATATYPE: &str = "Datatype";
pub const DIMENSION: &str = "Dimension";
pub const ENTITY: &str = "Entity";
pub const ENUMERATION: &str = "Enumeration";
pub const EVENT: &str = "Event";
pub const FORMAL_CONSTRAINT: &str = "FormalConstraint";
pub const IMPORT: &str = "Import";
pub const IMPORT_STATEMENT: &str = "ImportStatement";
pub const INFORMAL_CONSTRAINT: &str = "InformalConstraint";
pub const MEMBER: &str = "Member";
pub const MEMBER_IMPORT: &str = "MemberImport";
pub const MODULE: &str = "Module";
pub const MODULE_IMPORT: &str = "ModuleImport";
pub const ORDERING_CONSTRAINT: &str = "OrderingConstraint";
pub const PROPERTY: &str = "Property";
pub const RDF: &str = "Rdf";
pub const STRUCTURE: &str = "Structure";
pub const SEQUENCE: &str = "Sequence";
pub const TYPE_CLASS: &str = "TypeClass";
pub const TYPE_VARIANT: &str = "TypeVariant";
pub const UNION: &str = "Union";
pub const UNKNOWN: &str = "Unknown";
pub const VALUE_VARIANT: &str = "ValueVariant";
pub const ANNOTATION: &str = "Annotation";
pub const UNIQUENESS_CONSTRAINT: &str = "UniquenessConstraint";

/* Datatypes */
pub const BINARY: &str = "binary";
pub const BOOLEAN: &str = "boolean";
pub const CONTROLLED_LANGUAGE: &str = "controlledLanguage";
pub const DECIMAL: &str = "decimal";
pub const DOUBLE: &str = "double";
pub const INTEGER: &str = "integer";
pub const IRI: &str = "iri";
pub const LANGUAGE: &str = "language";
pub const STRING: &str = "string";
pub const UNSIGNED: &str = "unsigned";

pub const IDENTIFIER: &str = "identifier";
pub const QUALIFIED_IDENTIFIER: &str = "qualifiedIdentifier";
pub const IDENTIFIER_REFERENCE: &str = "identifierReference";

/* Properties */
pub const HAS_ANNOTATION: &str = "hasAnnotation";
pub const HAS_CARDINALITY: &str = "hasCardinality";
pub const CONTROLLED_LANG_STRING: &str = "controlledLangString";
pub const HAS_CONSTRAINT: &str = "hasConstraint";
pub const HAS_DEFINITION: &str = "hasDefinition";
pub const DOMAIN_TYPE: &str = "domainType";
pub const DOMAIN_VALUE: &str = "domainValue";
pub const HAS_ELEMENT: &str = "hasElement";
pub const HAS_IMPORT_STATEMENT: &str = "hasImportStatement";
pub const HAS_MEMBER: &str = "hasMember";
pub const IDENTITY_MEMBER: &str = "identityMember";
pub const NAME: &str = "name";
pub const RANGE_TYPE: &str = "rangeType";
pub const RANGE_VALUE: &str = "rangeValue";
pub const SOURCE_ENTITY: &str = "sourceEntity";
pub const SOURCE_LOCATION: &str = "sourceLocation";
pub const HAS_TYPE: &str = "hasType";
pub const HAS_TYPE_VARIANT: &str = "hasTypeVariant";
pub const HAS_VALUE_VARIANT: &str = "hasValueVariant";
pub const END_BYTE: &str = "endByte";
pub const START_BYTE: &str = "startByte";
pub const MAX_OCCURS: &str = "maxOccurs";
pub const MIN_OCCURS: &str = "minOccurs";
pub const ORDERING: &str = "ordering";
pub const SRC_LABEL: &str = "srcLabel";
pub const UNIQUENESS: &str = "uniqueness";

/* Values */
pub const ORDERED: &str = "Ordered";
pub const UNIQUE: &str = "Unique";
pub const NONUNIQUE: &str = "NonUnique";
pub const UNORDERED: &str = "Unordered";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

module_function!(|| {
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked sdml), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked owl),
            id!(unchecked rdf),
            id!(unchecked rdfs),
            id!(unchecked skos),
            id!(unchecked xsd)
        )])
            .with_definitions([
                // ---------------------------------------------------------------------------------
                // Classes ❱ Traits
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked Annotated) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Annotated"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked annotation) ;
                    property id!(unchecked owl:ObjectProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Annotated)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Annotation)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("annotation"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked Named) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Named"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked name) ;
                    property id!(unchecked  owl:DatatypeProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Named)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Identifier)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("name"@en)),
                   ])).into(),
                // ---------------------------------------------------------------------------------
                // Classes ❱ Annotations
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked Annotation) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Annotation"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked AnnotationProperty) ;
                    class id!(unchecked Annotation),  id!(unchecked owl:AnnotationProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Annotation Property"@en)),
                        annotation!(id!(unchecked owl:disjointWith), id!(unchecked Constraint)),
                    ])).into(),
                rdf!(
                    id!(unchecked Constraint) ;
                    class  id!(unchecked Annotation) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Constraint"@en)),
                        annotation!(id!(unchecked owl:disjointWith), id!(unchecked AnnotationProperty)),
                    ])).into(),
                rdf!(
                    id!(unchecked InformalConstraint) ;
                    class  id!(unchecked Constraint) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Informal Constraint"@en)),
                        annotation!(id!(unchecked owl:disjointWith), id!(unchecked FormalConstraint)),
                    ])).into(),
                rdf!(
                    id!(unchecked FormalConstraint) ;
                    class  id!(unchecked Constraint) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Formal Constraint"@en)),
                        annotation!(id!(unchecked owl:disjointWith), id!(unchecked InformalConstraint)),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Classes ❱ Modules & Imports
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked Module) ;
                    class  id!(unchecked Annotated) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Module"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked imports) ;
                    property id!(unchecked owl:ObjectProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Module)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked ImportStatement)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("imports"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked hasDefinition) ;
                    property id!(unchecked owl:ObjectProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Module)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Definition)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("has definition"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked ImportStatement) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Import Statement"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked import) ;
                    property id!(unchecked owl:ObjectProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked ImportStatement)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Import)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("import"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked Import) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Import"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked imported) ;
                    property id!(unchecked owl:ObjectProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Import)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("imported"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked rename) ;
                    property id!(unchecked owl:ObjectProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Import)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Identifier)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("imported"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked MemberImport) ;
                    class  id!(unchecked Import) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Member Import"@en)),
                        annotation!(id!(unchecked owl:disjointWith), id!(unchecked ModuleImport)),
                    ])).into(),
                rdf!(
                    id!(unchecked importedMember) ;
                    property id!(unchecked owl:DatatypeProperty),  id!(unchecked imported) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked MemberImport)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked QualifiedIdentifier)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("imported member"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked ModuleImport) ;
                    class id!(unchecked Import) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Module Import"@en)),
                        annotation!(id!(unchecked owl:disjointWith), id!(unchecked MemberImport)),
                    ])).into(),
                rdf!(
                    id!(unchecked importedModule) ;
                    property id!(unchecked owl:DatatypeProperty),  id!(unchecked imported) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked ModuleImport)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Identifier)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("imported module"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked importedModuleVersion) ;
                    property id!(unchecked owl:DatatypeProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked ModuleImport)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked xsd:AnyURI)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("imported module's version"@en)),
                   ])).into(),
                // ---------------------------------------------------------------------------------
                // Classes ❱ Definitions
                // ---------------------------------------------------------------------------------
                 rdf!(
                    id!(unchecked AnyType) ;
                    class id!(unchecked owl:Class) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Any Type"@en)),
                       annotation!(id!(unchecked skos:altLabel), rdf_str!("Thing"@en)),
                       annotation!(id!(unchecked skos:altLabel), rdf_str!("Anything"@en)),
                    ])).into(),
                 rdf!(
                    id!(unchecked SumType) ;
                    class  id!(unchecked Anytype) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Sum Type"@en)),
                    ])).into(),
                 rdf!(
                    id!(unchecked ProductType) ;
                    class  id!(unchecked Anytype) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Product Type"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Definition) ;
                    class  id!(unchecked Annotated) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Definition"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Datatype) ;
                    class  id!(unchecked Definition),  id!(unchecked AnyType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Datatype"@en)),
                        annotation!(id!(unchecked owl:equivalentClass), id!(unchecked rdfs:Datatype)),
                    ])).into(),
                rdf!(
                    id!(unchecked Entity) ;
                    class  id!(unchecked Definition),  id!(unchecked ProductType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Entity"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Enum) ;
                    class id!(unchecked Definition),  id!(unchecked SumType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Enumeration"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Event) ;
                    class  id!(unchecked Definition),  id!(unchecked ProductType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Event"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Property) ;
                    class  id!(unchecked Definition) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Property"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Rdf) ;
                    class  id!(unchecked Definition) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Rdf"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Structure) ;
                    class  id!(unchecked Definition),  id!(unchecked ProductType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Structure"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked TypeClass) ;
                    class  id!(unchecked Definition),  id!(unchecked AnyType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("TypeClass"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Union) ;
                    class  id!(unchecked Definition),  id!(unchecked SumType) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Union"@en)),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Classes ❱ Members & Variants
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked Cardinality) ;
                    class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Cardinality"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked Ordering) ;
                    class  id!(unchecked rdfs:Literal) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Ordering"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked ordering) ;
                    property  id!(unchecked owl:DatatypeProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked owl:minCardinality), v!(id!(unchecked xsd:nonNegativeInteger), 0)),
                        annotation!(id!(unchecked owl:maxCardinality), v!(id!(unchecked xsd:nonNegativeInteger), 1)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Cardinality)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Ordering)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("ordering"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked Uniqueness) ;
                    class  id!(unchecked rdfs:Literal) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Uniqueness"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked uniqueness) ;
                    property  id!(unchecked owl:DatatypeProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked owl:minCardinality), v!(id!(unchecked xsd:nonNegativeInteger), 0)),
                        annotation!(id!(unchecked owl:maxCardinality), v!(id!(unchecked xsd:nonNegativeInteger), 1)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Cardinality)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Uniqueness)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("uniqueness"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked minCardinality) ;
                    property  id!(unchecked owl:DatatypeProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked owl:minCardinality), v!(id!(unchecked xsd:nonNegativeInteger), 1)),
                        annotation!(id!(unchecked owl:maxCardinality), v!(id!(unchecked xsd:nonNegativeInteger), 1)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Cardinality)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked xsd:nonNegativeInteger)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("minimum cardinality"@en)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("minimum occurs"@en)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("minimum count"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked maxCardinality) ;
                    property  id!(unchecked owl:DatatypeProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked owl:minCardinality), v!(id!(unchecked xsd:nonNegativeInteger), 0)),
                        annotation!(id!(unchecked owl:maxCardinality), v!(id!(unchecked xsd:nonNegativeInteger), 1)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Cardinality)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked xsd:nonNegativeInteger)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("maximum cardinality"@en)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("maximum occurs"@en)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("maximum count"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked Variant) ;
                    class  id!(unchecked Annotated), id!(unchecked Named) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Variant"@en)),
                    ])).into(),
                rdf!(
                    id!(unchecked TypeVariant) ;
                    class  id!(unchecked Variant) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Type Variant"@en)),
                        annotation!(id!(unchecked owl:disjointWith), id!(unchecked ValueVariant)),
                    ])).into(),
                rdf!(
                    id!(unchecked as) ;
                    property  id!(unchecked owl:DatatypeProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked TypeVariant)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Identifier)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("as"@en)),
                   ])).into(),
                 rdf!(
                    id!(unchecked ValueVariant) ;
                    class  id!(unchecked Variant) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Value Variant"@en)),
                        annotation!(id!(unchecked owl:disjointWith), id!(unchecked TypeVariant)),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Classes ❱ Values
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked hasElement) ;
                    property id!(unchecked owl:DatatypeProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("has element"@en)),
                   ])).into(),
                // ---------------------------------------------------------------------------------
                // Classes ❱ Values ❱ Identifiers
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked Identifier) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Datatype"@en)),
                   ])).into(),
                rdf!(
                    id!(unchecked QualifiedIdentifier) ;
                    class  ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Qualified Identifier"@en)),
                    ])).into(),
                 rdf!(
                    id!(unchecked IdentifierReference) ;
                    class  ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Identifier Reference"@en)),
                    ])).into(),
                // ---------------------------------------------------------------------------------
                // Datatypes
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked Binary) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Binary"@en)),
                        annotation!(id!(unchecked owl:equivalentClass), id!(unchecked xsd:hexBinary)),
                   ])).into(),
                rdf!(
                    id!(unchecked Boolean) ;
                    datatype ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("Boolean"@en)),
                        annotation!(id!(unchecked owl:equivalentClass), id!(unchecked xsd:boolean)),
                   ])).into(),
                // ---------------------------------------------------------------------------------
                // Individuals
                // ---------------------------------------------------------------------------------
                rdf!(
                    id!(unchecked nonunique) ;
                    individual  id!(unchecked Uniqueness) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("nonunique"@en)),
                        annotation!(id!(unchecked owl:differentFrom), id!(unchecked nunique)),
                    ])).into(),
                rdf!(
                    id!(unchecked unique) ;
                    individual  id!(unchecked Uniqueness) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("unique"@en)),
                        annotation!(id!(unchecked owl:differentFrom), id!(unchecked nonunique)),
                    ])).into(),
                rdf!(
                    id!(unchecked unordered) ;
                    individual  id!(unchecked Ordering) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("unordered"@en)),
                        annotation!(id!(unchecked owl:differentFrom), id!(unchecked ordered)),
                    ])).into(),
                rdf!(
                    id!(unchecked ordered) ;
                    individual  id!(unchecked Ordering) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked sdml)),
                        annotation!(id!(unchecked skos:prefLabel), rdf_str!("ordered"@en)),
                        annotation!(id!(unchecked owl:differentFrom), id!(unchecked unordered)),
                    ])).into(),
            ])
    )
});
