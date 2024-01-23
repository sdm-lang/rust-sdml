/*
This Rust module contains the SDML model of the SDML library module `owl`.
*/

use crate::model::modules::{ImportStatement, Module};
use crate::model::HasBody;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "owl";
pub const MODULE_URL: &str = "http://www.w3.org/2002/07/owl#";

pub const CLASS_ALL_DIFFERENT_NAME: &str = "AllDifferent";
pub const CLASS_ALL_DISJOINT_CLASSES_NAME: &str = "AllDisjointClasses";
pub const CLASS_ALL_DISJOINT_PROPERTIES_NAME: &str = "AllDisjointProperties";
pub const CLASS_ANNOTATION_NAME: &str = "Annotation";
pub const CLASS_ANNOTATION_PROPERTY_NAME: &str = "AnnotationProperty";
pub const CLASS_ASYMMETRIC_PROPERTY_NAME: &str = "AsymmetricProperty";
pub const CLASS_AXIOM_NAME: &str = "Axiom";
pub const CLASS_CLASS_NAME: &str = "Class";
pub const CLASS_DATA_RANGE_NAME: &str = "DataRange";
pub const CLASS_DATATYPE_PROPERTY_NAME: &str = "DatatypeProperty";
pub const CLASS_DEPRECATED_CLASS_NAME: &str = "DeprecatedClass";
pub const CLASS_DEPRECATED_PROPERTY_NAME: &str = "DeprecatedProperty";
pub const CLASS_FUNCTIONAL_PROPERTY_NAME: &str = "FunctionalProperty";
pub const CLASS_INVERSE_FUNCTIONAL_PROPERTY_NAME: &str = "InverseFunctionalProperty";
pub const CLASS_IRREFLEXIVE_PROPERTY_NAME: &str = "IrreflexiveProperty";
pub const CLASS_NAMED_INDIVIDUAL_NAME: &str = "NamedIndividual";
pub const CLASS_NEGATIVE_PROPERTY_ASSERTION_NAME: &str = "NegativePropertyAssertion";
pub const CLASS_NOTHING_NAME: &str = "Nothing";
pub const CLASS_OBJECT_PROPERTY_NAME: &str = "ObjectProperty";
pub const CLASS_ONTOLOGY_NAME: &str = "Ontology";
pub const CLASS_ONTOLOGY_PROPERTY_NAME: &str = "OntologyProperty";
pub const CLASS_REFLEXIVE_PROPERTY_NAME: &str = "ReflexiveProperty";
pub const CLASS_RESTRICTION_NAME: &str = "Restriction";
pub const CLASS_SYMMETRIC_PROPERTY_NAME: &str = "SymmetricProperty";
pub const CLASS_TRANSITIVE_PROPERTY_NAME: &str = "TransitiveProperty";
pub const CLASS_THING_NAME: &str = "Thing";

pub const PROP_ALL_VALUES_FROM_NAME: &str = "allValuesFrom";
pub const PROP_ANNOTATED_PROPERTY_NAME: &str = "annotatedProperty";
pub const PROP_ANNOTATED_SOURCE_NAME: &str = "annotatedSource";
pub const PROP_ANNOTATED_TARGET_NAME: &str = "annotatedTarget";
pub const PROP_ASSERTION_PROPERTY_NAME: &str = "assertionProperty";
pub const PROP_BACKWARD_COMPATIBLE_WITH_NAME: &str = "backwardCompatibleWith";
pub const PROP_BOTTOM_DATA_PROPERTY_NAME: &str = "bottomDataProperty";
pub const PROP_BOTTOM_OBJECT_PROPERTY_NAME: &str = "bottomObjectProperty";
pub const PROP_CARDINALITY_NAME: &str = "cardinality";
pub const PROP_COMPLEMENT_OF_NAME: &str = "complementOf";
pub const PROP_DATATYPE_COMPLEMENT_OF_NAME: &str = "datatypeComplementOf";
pub const PROP_DEPRECATED_NAME: &str = "deprecated";
pub const PROP_DIFFERENT_FROM_NAME: &str = "differentFrom";
pub const PROP_DISJOINT_UNION_OF_NAME: &str = "disjointUnionOf";
pub const PROP_DISJOINT_WITH_NAME: &str = "disjointWith";
pub const PROP_DISTINCT_MEMBERS_NAME: &str = "distinctMembers";
pub const PROP_EQUIVALENT_CLASS_NAME: &str = "equivalentClass";
pub const PROP_EQUIVALENT_PROPERTY_NAME: &str = "equivalentProperty";
pub const PROP_HAS_KEY_NAME: &str = "hasKey";
pub const PROP_HAS_SELF_NAME: &str = "hasSelf";
pub const PROP_HAS_VALUE_NAME: &str = "hasValue";
pub const PROP_IMPORTS_NAME: &str = "imports";
pub const PROP_INCOMPATIBLE_WITH_NAME: &str = "incompatibleWith";
pub const PROP_INTERSECTION_OF_NAME: &str = "intersectionOf";
pub const PROP_INVERSE_OF_NAME: &str = "inverseOf";
pub const PROP_MAX_CARDINALITY_NAME: &str = "maxCardinality";
pub const PROP_MAX_QUALIFIED_CARDINALITY_NAME: &str = "maxQualifiedCardinality";
pub const PROP_MEMBERS_NAME: &str = "members";
pub const PROP_MIN_CARDINALITY_NAME: &str = "minCardinality";
pub const PROP_MIN_QUALIFIED_CARDINALITY_NAME: &str = "minQualifiedCardinality";
pub const PROP_ON_CLASS_NAME: &str = "onClass";
pub const PROP_ON_DATA_RANGE_NAME: &str = "onDataRange";
pub const PROP_ON_DATATYPE_NAME: &str = "onDatatype";
pub const PROP_ONE_OF_NAME: &str = "oneOf";
pub const PROP_ON_PROPERTIES_NAME: &str = "onProperties";
pub const PROP_ON_PROPERTY_NAME: &str = "onProperty";
pub const PROP_PRIOR_VERSION_NAME: &str = "priorVersion";
pub const PROP_PROPERTY_CHAIN_AXIOM_NAME: &str = "propertyChainAxiom";
pub const PROP_PROPERTY_DISJOINT_WITH_NAME: &str = "propertyDisjointWith";
pub const PROP_QUALIFIED_CARDINALITY_NAME: &str = "qualifiedCardinality";
pub const PROP_SAME_AS_NAME: &str = "sameAs";
pub const PROP_SOME_VALUES_FROM_NAME: &str = "someValuesFrom";
pub const PROP_SOURCE_INDIVIDUAL_NAME: &str = "sourceIndividual";
pub const PROP_TARGET_INDIVIDUAL_NAME: &str = "targetIndividual";
pub const PROP_TARGET_VALUE_NAME: &str = "targetValue";
pub const PROP_TOP_DATA_PROPERTY_NAME: &str = "topDataProperty";
pub const PROP_TOP_OBJECT_PROPERTY_NAME: &str = "topObjectProperty";
pub const PROP_UNION_OF_NAME: &str = "unionOf";
pub const PROP_VERSION_INFO_NAME: &str = "versionInfo";
pub const PROP_VERSION_IRI_NAME: &str = "versionIRI";
pub const PROP_WITH_RESTRICTIONS_NAME: &str = "withRestrictions";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(Url::parse(MODULE_URL).unwrap());

    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(id!(super::rdf::MODULE_NAME)));
    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(id!(super::rdfs::MODULE_NAME)));
    module
        .body_mut()
        .add_to_imports(ImportStatement::new_module(id!(super::xsd::MODULE_NAME)));

    module.body_mut().extend_definitions(vec![
        // Classes
        rdf!(class CLASS_ALL_DIFFERENT_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The class of collections of pairwise different individuals.")
            .into(),
        rdf!(class CLASS_ALL_DISJOINT_CLASSES_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The class of collections of pairwise disjoint classes.")
            .into(),
        rdf!(class CLASS_ALL_DISJOINT_PROPERTIES_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The class of collections of pairwise disjoint properties.")
            .into(),
        rdf!(class CLASS_ANNOTATION_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The class of annotated annotations for which the RDF serialization consists of an annotated subject, predicate and object.")
            .into(),
        rdf!(class CLASS_ANNOTATION_PROPERTY_NAME, MODULE_IRI; (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME))
            .with_comment("The class of annotation properties.")
            .into(),
        rdf!(class CLASS_ASYMMETRIC_PROPERTY_NAME, MODULE_IRI; CLASS_OBJECT_PROPERTY_NAME)
            .with_comment("The class of asymmetric properties.")
            .into(),
        rdf!(class CLASS_AXIOM_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The class of annotated axioms for which the RDF serialization consists of an annotated subject, predicate and object.")
           .into(),
        rdf!(class CLASS_CLASS_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_CLASS_NAME))
            .with_comment("The class of OWL classes.")
            .into(),
        rdf!(class CLASS_DATA_RANGE_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
            .with_comment("The class of OWL data ranges, which are special kinds of datatypes. Note: The use of the IRI owl:DataRange has been deprecated as of OWL 2. The IRI rdfs:Datatype SHOULD be used instead.")
            .into(),
        rdf!(class CLASS_DATATYPE_PROPERTY_NAME, MODULE_IRI; CLASS_OBJECT_PROPERTY_NAME)
            .with_comment("The class of data properties.")
            .into(),
        rdf!(class CLASS_DEPRECATED_CLASS_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_CLASS_NAME))
            .with_comment("The class of deprecated classes.")
            .into(),
        rdf!(class CLASS_DEPRECATED_PROPERTY_NAME, MODULE_IRI; (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME))
            .with_comment("The class of deprecated properties.")
            .into(),
        rdf!(class CLASS_FUNCTIONAL_PROPERTY_NAME, MODULE_IRI; (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME))
            .with_comment("The class of functional properties.")
            .into(),
        rdf!(class CLASS_INVERSE_FUNCTIONAL_PROPERTY_NAME, MODULE_IRI; CLASS_OBJECT_PROPERTY_NAME)
            .with_comment("The class of inverse-functional properties.")
            .into(),
        rdf!(class CLASS_IRREFLEXIVE_PROPERTY_NAME, MODULE_IRI; CLASS_OBJECT_PROPERTY_NAME)
            .with_comment("The class of irreflexive properties.")
            .into(),
        rdf!(class CLASS_NAMED_INDIVIDUAL_NAME, MODULE_IRI; CLASS_THING_NAME)
            .with_comment("The class of named individuals.")
           .into(),
        rdf!(class CLASS_NEGATIVE_PROPERTY_ASSERTION_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The class of negative property assertions.")
            .into(),
        rdf!(class CLASS_NOTHING_NAME, MODULE_IRI; CLASS_THING_NAME)
            .with_comment("This is the empty class.")
            .into(),
        rdf!(class CLASS_OBJECT_PROPERTY_NAME, MODULE_IRI; (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME))
            .with_comment("The class of object properties.")
            .into(),
        rdf!(class CLASS_ONTOLOGY_NAME, MODULE_IRI; (super::rdfs::MODULE_NAME, super::rdfs::CLASS_CLASS_NAME))
            .with_comment("The class of ontologies.")
            .into(),
        rdf!(class CLASS_ONTOLOGY_PROPERTY_NAME, MODULE_IRI; (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME))
            .with_comment("The class of ontology properties.")
            .into(),
        rdf!(class CLASS_REFLEXIVE_PROPERTY_NAME, MODULE_IRI; CLASS_OBJECT_PROPERTY_NAME)
            .with_comment("The class of reflexive properties.")
            .into(),
        rdf!(class CLASS_SYMMETRIC_PROPERTY_NAME, MODULE_IRI; CLASS_OBJECT_PROPERTY_NAME)
            .with_comment("The class of symmetric properties.")
            .into(),
        rdf!(class CLASS_TRANSITIVE_PROPERTY_NAME, MODULE_IRI; CLASS_OBJECT_PROPERTY_NAME)
            .with_comment("The class of transitive properties.")
            .into(),
        rdf!(class CLASS_THING_NAME, MODULE_IRI; CLASS_CLASS_NAME)
            .with_comment("The class of OWL individuals.")
             .into(),

        // Properties
        rdf!(property PROP_ALL_VALUES_FROM_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The property that determines the class that a universal property restriction refers to.")
            .into(),
        rdf!(property PROP_ANNOTATED_PROPERTY_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME) =>
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The property that determines the predicate of an annotated axiom or annotated annotation.")
            .into(),
        rdf!(property PROP_ANNOTATED_SOURCE_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME) =>
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The property that determines the subject of an annotated axiom or annotated annotation.")
            .into(),
        rdf!(property PROP_ANNOTATED_TARGET_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME) =>
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The property that determines the object of an annotated axiom or annotated annotation.")
            .into(),
        rdf!(property PROP_ASSERTION_PROPERTY_NAME, MODULE_IRI;
             CLASS_NEGATIVE_PROPERTY_ASSERTION_NAME => (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME))
            .with_comment("The property that determines the predicate of a negative property assertion.")
            .into(),
        rdf!(property PROP_BACKWARD_COMPATIBLE_WITH_NAME, MODULE_IRI;
             CLASS_ONTOLOGY_NAME => CLASS_ONTOLOGY_NAME)
            .with_comment("The annotation property that indicates that a given ontology is backward compatible with another ontology.")
            .into(),
        rdf!(property PROP_BOTTOM_DATA_PROPERTY_NAME, MODULE_IRI;
             CLASS_THING_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_LITERAL_NAME))
            .with_comment("The data property that does not relate any individual to any data value.")
            .into(),
        rdf!(property PROP_CARDINALITY_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::xsd::MODULE_NAME, super::xsd::DT_NONNEGATIVE_INTEGER_NAME))
            .with_comment("The property that determines the cardinality of an exact cardinality restriction.")
            .into(),
        rdf!(property PROP_COMPLEMENT_OF_NAME, MODULE_IRI;
             CLASS_CLASS_NAME => CLASS_CLASS_NAME)
            .with_comment("The property that determines that a given class is the complement of another class.")
            .into(),
        rdf!(property PROP_DATATYPE_COMPLEMENT_OF_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME) =>
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
            .with_comment("The property that determines that a given data range is the complement of another data range with respect to the data domain.")
            .into(),
        rdf!(property PROP_DEPRECATED_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME) =>
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The annotation property that indicates that a given entity has been deprecated.")
            .into(),
        rdf!(property PROP_DIFFERENT_FROM_NAME, MODULE_IRI; CLASS_THING_NAME => CLASS_THING_NAME)
            .with_comment("The property that determines that two given individuals are different.")
            .into(),
        rdf!(property PROP_DISJOINT_UNION_OF_NAME, MODULE_IRI;
             CLASS_CLASS_NAME => (super::rdfs::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_comment("The property that determines that a given class is equivalent to the disjoint union of a collection of other classes.")
            .into(),
        rdf!(property PROP_DISJOINT_WITH_NAME, MODULE_IRI;
             CLASS_CLASS_NAME => CLASS_CLASS_NAME)
            .with_comment("The property that determines that two given classes are disjoint.")
            .into(),
        rdf!(property PROP_DISTINCT_MEMBERS_NAME, MODULE_IRI;
        CLASS_ALL_DIFFERENT_NAME => (super::rdf::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_comment("The property that determines the collection of pairwise different individuals in a owl:AllDifferent axiom.")
            .into(),
        rdf!(property PROP_EQUIVALENT_CLASS_NAME, MODULE_IRI;
             CLASS_CLASS_NAME => CLASS_CLASS_NAME)
            .with_comment("The property that determines that two given classes are equivalent, and that is used to specify datatype definitions.")
            .into(),
        rdf!(property PROP_EQUIVALENT_PROPERTY_NAME, MODULE_IRI;
             CLASS_CLASS_NAME => CLASS_CLASS_NAME)
            .with_comment("The property that determines that two given properties are equivalent.")
            .into(),
        rdf!(property PROP_HAS_KEY_NAME, MODULE_IRI;
             CLASS_CLASS_NAME => (super::rdf::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_comment("The property that determines the collection of properties that jointly build a key.")
            .into(),
        rdf!(property PROP_HAS_SELF_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The property that determines the property that a self restriction refers to.")
            .into(),
        rdf!(property PROP_HAS_VALUE_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The property that determines the property that a has-value restriction refers to.")
            .into(),
        rdf!(property PROP_IMPORTS_NAME, MODULE_IRI;
             CLASS_ONTOLOGY_NAME => CLASS_ONTOLOGY_NAME)
            .with_comment("The property that is used for importing other ontologies into a given ontology.")
            .into(),
        rdf!(property PROP_INCOMPATIBLE_WITH_NAME, MODULE_IRI; CLASS_ONTOLOGY_NAME => CLASS_ONTOLOGY_NAME)
            .with_comment("The annotation property that indicates that a given ontology is incompatible with another ontology.")
            .into(),
        rdf!(property PROP_INTERSECTION_OF_NAME, MODULE_IRI;
             CLASS_CLASS_NAME => (super::rdf::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_comment("The property that determines the collection of classes or data ranges that build an intersection.")
            .into(),
        rdf!(property PROP_INVERSE_OF_NAME, MODULE_IRI; CLASS_OBJECT_PROPERTY_NAME => CLASS_OBJECT_PROPERTY_NAME)
            .with_comment("The property that determines that two given properties are inverse.")
            .into(),
        rdf!(property PROP_MAX_CARDINALITY_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::xsd::MODULE_NAME, super::xsd::DT_NONNEGATIVE_INTEGER_NAME))
            .with_comment("The property that determines the cardinality of a maximum cardinality restriction.")
            .into(),
        rdf!(property PROP_MAX_QUALIFIED_CARDINALITY_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::xsd::MODULE_NAME, super::xsd::DT_NONNEGATIVE_INTEGER_NAME))
            .with_comment("The property that determines the cardinality of a maximum qualified cardinality restriction.")
            .into(),
        rdf!(property PROP_MEMBERS_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME) =>
             (super::rdf::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_comment("The property that determines the collection of members in either a owl:AllDifferent, owl:AllDisjointClasses or owl:AllDisjointProperties axiom.")
            .into(),
        rdf!(property PROP_MIN_CARDINALITY_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::xsd::MODULE_NAME, super::xsd::DT_NONNEGATIVE_INTEGER_NAME))
            .with_comment("The property that determines the cardinality of a minimum cardinality restriction.")
            .into(),
        rdf!(property PROP_MIN_QUALIFIED_CARDINALITY_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::xsd::MODULE_NAME, super::xsd::DT_NONNEGATIVE_INTEGER_NAME))
            .with_comment("The property that determines the cardinality of a minimum qualified cardinality restriction.")
            .into(),
        rdf!(property PROP_ON_CLASS_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => CLASS_CLASS_NAME)
            .with_comment("The property that determines the class that a qualified object cardinality restriction refers to.")
            .into(),
        rdf!(property PROP_ON_DATA_RANGE_NAME, MODULE_IRI;
        CLASS_RESTRICTION_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
            .with_comment("The property that determines the data range that a qualified data cardinality restriction refers to.")
            .into(),
        rdf!(property PROP_ON_DATATYPE_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME) =>
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME))
            .with_comment("The property that determines the datatype that a datatype restriction refers to.")
            .into(),
        rdf!(property PROP_ONE_OF_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_CLASS_NAME) =>
             (super::rdf::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_comment("The property that determines the collection of individuals or data values that build an enumeration.")
            .into(),
        rdf!(property PROP_ON_PROPERTIES_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::rdf::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_comment("The property that determines the n-tuple of properties that a property restriction on an n-ary data range refers to.")
            .into(),
        rdf!(property PROP_ON_PROPERTY_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME))
            .with_comment("The property that determines the property that a property restriction refers to.")
            .into(),
        rdf!(property PROP_PRIOR_VERSION_NAME, MODULE_IRI;
             CLASS_ONTOLOGY_NAME => CLASS_ONTOLOGY_NAME)
            .with_comment("The annotation property that indicates the predecessor ontology of a given ontology.")
            .into(),
        rdf!(property PROP_PROPERTY_CHAIN_AXIOM_NAME, MODULE_IRI;
             CLASS_OBJECT_PROPERTY_NAME => (super::rdf::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_comment("The property that determines the n-tuple of properties that build a sub property chain of a given property.")
            .into(),
        rdf!(property PROP_PROPERTY_DISJOINT_WITH_NAME, MODULE_IRI;
             (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME) =>
             (super::rdf::MODULE_NAME, super::rdf::CLASS_PROPERTY_NAME))
            .with_comment("The property that determines that two given properties are disjoint.")
            .into(),
        rdf!(property PROP_QUALIFIED_CARDINALITY_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::xsd::MODULE_NAME, super::xsd::DT_NONNEGATIVE_INTEGER_NAME))
            .with_comment("The property that determines the cardinality of an exact qualified cardinality restriction.")
            .into(),
        rdf!(property PROP_SAME_AS_NAME, MODULE_IRI;
             CLASS_THING_NAME => CLASS_THING_NAME)
            .with_comment("The property that determines that two given individuals are equal.")
            .into(),
        rdf!(property PROP_SOME_VALUES_FROM_NAME, MODULE_IRI;
             CLASS_RESTRICTION_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_CLASS_NAME))
            .with_comment("The property that determines the class that an existential property restriction refers to.")
            .into(),
        rdf!(property PROP_SOURCE_INDIVIDUAL_NAME, MODULE_IRI;
             CLASS_NEGATIVE_PROPERTY_ASSERTION_NAME => CLASS_THING_NAME)
            .with_comment("The property that determines the subject of a negative property assertion.")
            .into(),
        rdf!(property PROP_TARGET_INDIVIDUAL_NAME, MODULE_IRI;
             CLASS_NEGATIVE_PROPERTY_ASSERTION_NAME => CLASS_THING_NAME)
            .with_comment("The property that determines the object of a negative property assertion.")
            .into(),
        rdf!(property PROP_TARGET_VALUE_NAME, MODULE_IRI;
             CLASS_NEGATIVE_PROPERTY_ASSERTION_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_LITERAL_NAME))
            .with_comment("The property that determines the value of a negative data property assertion.")
            .into(),
        rdf!(property PROP_TOP_DATA_PROPERTY_NAME, MODULE_IRI;
             CLASS_THING_NAME => (super::rdfs::MODULE_NAME, super::rdfs::CLASS_LITERAL_NAME))
            .with_comment("The data property that relates every individual to every data value.")
            .into(),
        rdf!(property PROP_TOP_OBJECT_PROPERTY_NAME, MODULE_IRI;
             CLASS_THING_NAME => CLASS_THING_NAME)
            .with_comment("The object property that relates every two individuals.")
            .into(),
        rdf!(property PROP_UNION_OF_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_CLASS_NAME) =>
             (super::rdf::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_comment("The property that determines the collection of classes or data ranges that build a union.")
            .into(),
        rdf!(property PROP_VERSION_INFO_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME) =>
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_RESOURCE_NAME))
            .with_comment("The annotation property that provides version information for an ontology or another OWL construct.")
            .into(),
        rdf!(property PROP_VERSION_IRI_NAME, MODULE_IRI;
             CLASS_ONTOLOGY_NAME => CLASS_ONTOLOGY_NAME)
            .with_comment("The object property that identifies the version IRI of an ontology.")
            .into(),
        rdf!(property PROP_WITH_RESTRICTIONS_NAME, MODULE_IRI;
             (super::rdfs::MODULE_NAME, super::rdfs::CLASS_DATATYPE_NAME) =>
             (super::rdf::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_comment("The property that determines the collection of facet-value pairs that define a datatype restriction.")
             .into(),
    ]);

    module
}
