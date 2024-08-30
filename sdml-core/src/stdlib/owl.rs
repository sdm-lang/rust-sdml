/*!
This Rust module contains the SDML model of the SDML library module `owl` for OWL.
*/

use crate::model::annotations::AnnotationBuilder;
use crate::model::modules::Module;
use crate::model::HasBody;
use crate::stdlib::{rdf, rdfs, xsd};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "owl";
pub const MODULE_URL: &str = "http://www.w3.org/2002/07/owl#";

pub const ALL_DIFFERENT: &str = "AllDifferent";
pub const ALL_DISJOINT_CLASSES: &str = "AllDisjointClasses";
pub const ALL_DISJOINT_PROPERTIES: &str = "AllDisjointProperties";
pub const ANNOTATION: &str = "Annotation";
pub const ANNOTATION_PROPERTY: &str = "AnnotationProperty";
pub const ASYMMETRIC_PROPERTY: &str = "AsymmetricProperty";
pub const AXIOM: &str = "Axiom";
pub const CLASS: &str = "Class";
pub const DATA_RANGE: &str = "DataRange";
pub const DATATYPE_PROPERTY: &str = "DatatypeProperty";
pub const DEPRECATED_CLASS: &str = "DeprecatedClass";
pub const DEPRECATED_PROPERTY: &str = "DeprecatedProperty";
pub const FUNCTIONAL_PROPERTY: &str = "FunctionalProperty";
pub const INVERSE_FUNCTIONAL_PROPERTY: &str = "InverseFunctionalProperty";
pub const IRREFLEXIVE_PROPERTY: &str = "IrreflexiveProperty";
pub const NAMED_INDIVIDUAL: &str = "NamedIndividual";
pub const NEGATIVE_PROPERTY_ASSERTION: &str = "NegativePropertyAssertion";
pub const NOTHING: &str = "Nothing";
pub const OBJECT_PROPERTY: &str = "ObjectProperty";
pub const ONTOLOGY: &str = "Ontology";
pub const ONTOLOGY_PROPERTY: &str = "OntologyProperty";
pub const REFLEXIVE_PROPERTY: &str = "ReflexiveProperty";
pub const RESTRICTION: &str = "Restriction";
pub const SYMMETRIC_PROPERTY: &str = "SymmetricProperty";
pub const TRANSITIVE_PROPERTY: &str = "TransitiveProperty";
pub const THING: &str = "Thing";

pub const ALL_VALUES_FROM: &str = "allValuesFrom";
pub const ANNOTATED_PROPERTY: &str = "annotatedProperty";
pub const ANNOTATED_SOURCE: &str = "annotatedSource";
pub const ANNOTATED_TARGET: &str = "annotatedTarget";
pub const ASSERTION_PROPERTY: &str = "assertionProperty";
pub const BACKWARD_COMPATIBLE_WITH: &str = "backwardCompatibleWith";
pub const BOTTOM_DATA_PROPERTY: &str = "bottomDataProperty";
pub const BOTTOM_OBJECT_PROPERTY: &str = "bottomObjectProperty";
pub const CARDINALITY: &str = "cardinality";
pub const COMPLEMENT_OF: &str = "complementOf";
pub const DATATYPE_COMPLEMENT_OF: &str = "datatypeComplementOf";
pub const DEPRECATED: &str = "deprecated";
pub const DIFFERENT_FROM: &str = "differentFrom";
pub const DISJOINT_UNION_OF: &str = "disjointUnionOf";
pub const DISJOINT_WITH: &str = "disjointWith";
pub const DISTINCT_MEMBERS: &str = "distinctMembers";
pub const EQUIVALENT_CLASS: &str = "equivalentClass";
pub const EQUIVALENT_PROPERTY: &str = "equivalentProperty";
pub const HAS_KEY: &str = "hasKey";
pub const HAS_SELF: &str = "hasSelf";
pub const HAS_VALUE: &str = "hasValue";
pub const IMPORTS: &str = "imports";
pub const INCOMPATIBLE_WITH: &str = "incompatibleWith";
pub const INTERSECTION_OF: &str = "intersectionOf";
pub const INVERSE_OF: &str = "inverseOf";
pub const MAX_CARDINALITY: &str = "maxCardinality";
pub const MAX_QUALIFIED_CARDINALITY: &str = "maxQualifiedCardinality";
pub const MEMBERS: &str = "members";
pub const MIN_CARDINALITY: &str = "minCardinality";
pub const MIN_QUALIFIED_CARDINALITY: &str = "minQualifiedCardinality";
pub const ON_CLASS: &str = "onClass";
pub const PROP_ON_DATA_RANGE_NAME: &str = "onDataRange";
pub const ON_DATATYPE: &str = "onDatatype";
pub const ONE_OF: &str = "oneOf";
pub const ON_PROPERTIES: &str = "onProperties";
pub const ON_PROPERTY: &str = "onProperty";
pub const PRIOR_VERSION: &str = "priorVersion";
pub const PROPERTY_CHAIN_AXIOM: &str = "propertyChainAxiom";
pub const PROPERTY_DISJOINT_WITH: &str = "propertyDisjointWith";
pub const QUALIFIED_CARDINALITY: &str = "qualifiedCardinality";
pub const SAME_AS: &str = "sameAs";
pub const SOME_VALUES_FROM: &str = "someValuesFrom";
pub const SOURCE_INDIVIDUAL: &str = "sourceIndividual";
pub const TARGET_INDIVIDUAL: &str = "targetIndividual";
pub const TARGET_VALUE: &str = "targetValue";
pub const TOP_DATA_PROPERTY: &str = "topDataProperty";
pub const TOP_OBJECT_PROPERTY: &str = "topObjectProperty";
pub const UNION_OF: &str = "unionOf";
pub const VERSION_INFO: &str = "versionInfo";
pub const VERSION_IRI: &str = "versionIRI";
pub const WITH_RESTRICTIONS: &str = "withRestrictions";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(Url::parse(MODULE_URL).unwrap());

    module.body_mut().add_to_imports(import!(
        id!(rdf::MODULE_NAME),
        id!(rdfs::MODULE_NAME),
        id!(xsd::MODULE_NAME)
    ));

    module.body_mut().extend_definitions(vec![
        // Classes
        rdf!(class ALL_DIFFERENT, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The class of collections of pairwise different individuals.")
            .into(),
        rdf!(class ALL_DISJOINT_CLASSES, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The class of collections of pairwise disjoint classes.")
            .into(),
        rdf!(class ALL_DISJOINT_PROPERTIES, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The class of collections of pairwise disjoint properties.")
            .into(),
        rdf!(class ANNOTATION, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The class of annotated annotations for which the RDF serialization consists of an annotated subject, predicate and object.")
            .into(),
        rdf!(class ANNOTATION_PROPERTY, MODULE_IRI; (rdf::MODULE_NAME, rdf::PROPERTY))
            .with_comment("The class of annotation properties.")
            .into(),
        rdf!(class ASYMMETRIC_PROPERTY, MODULE_IRI; OBJECT_PROPERTY)
            .with_comment("The class of asymmetric properties.")
            .into(),
        rdf!(class AXIOM, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The class of annotated axioms for which the RDF serialization consists of an annotated subject, predicate and object.")
           .into(),
        rdf!(class CLASS, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::CLASS))
            .with_comment("The class of OWL classes.")
            .into(),
        rdf!(class DATA_RANGE, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::DATATYPE))
            .with_comment("The class of OWL data ranges, which are special kinds of datatypes. Note: The use of the IRI owl:DataRange has been deprecated as of OWL 2. The IRI rdfs:Datatype SHOULD be used instead.")
            .into(),
        rdf!(class DATATYPE_PROPERTY, MODULE_IRI; OBJECT_PROPERTY)
            .with_comment("The class of data properties.")
            .into(),
        rdf!(class DEPRECATED_CLASS, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::CLASS))
            .with_comment("The class of deprecated classes.")
            .into(),
        rdf!(class DEPRECATED_PROPERTY, MODULE_IRI; (rdf::MODULE_NAME, rdf::PROPERTY))
            .with_comment("The class of deprecated properties.")
            .into(),
        rdf!(class FUNCTIONAL_PROPERTY, MODULE_IRI; (rdf::MODULE_NAME, rdf::PROPERTY))
            .with_comment("The class of functional properties.")
            .into(),
        rdf!(class INVERSE_FUNCTIONAL_PROPERTY, MODULE_IRI; OBJECT_PROPERTY)
            .with_comment("The class of inverse-functional properties.")
            .into(),
        rdf!(class IRREFLEXIVE_PROPERTY, MODULE_IRI; OBJECT_PROPERTY)
            .with_comment("The class of irreflexive properties.")
            .into(),
        rdf!(class NAMED_INDIVIDUAL, MODULE_IRI; THING)
            .with_comment("The class of named individuals.")
           .into(),
        rdf!(class NEGATIVE_PROPERTY_ASSERTION, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The class of negative property assertions.")
            .into(),
        rdf!(class NOTHING, MODULE_IRI; THING)
            .with_comment("This is the empty class.")
            .into(),
        rdf!(class OBJECT_PROPERTY, MODULE_IRI; (rdf::MODULE_NAME, rdf::PROPERTY))
            .with_comment("The class of object properties.")
            .into(),
        rdf!(class ONTOLOGY, MODULE_IRI; (rdfs::MODULE_NAME, rdfs::CLASS))
            .with_comment("The class of ontologies.")
            .into(),
        rdf!(class ONTOLOGY_PROPERTY, MODULE_IRI; (rdf::MODULE_NAME, rdf::PROPERTY))
            .with_comment("The class of ontology properties.")
            .into(),
        rdf!(class REFLEXIVE_PROPERTY, MODULE_IRI; OBJECT_PROPERTY)
            .with_comment("The class of reflexive properties.")
            .into(),
        rdf!(class SYMMETRIC_PROPERTY, MODULE_IRI; OBJECT_PROPERTY)
            .with_comment("The class of symmetric properties.")
            .into(),
        rdf!(class TRANSITIVE_PROPERTY, MODULE_IRI; OBJECT_PROPERTY)
            .with_comment("The class of transitive properties.")
            .into(),
        rdf!(class THING, MODULE_IRI; CLASS)
            .with_comment("The class of OWL individuals.")
             .into(),

        // Properties
        rdf!(property ALL_VALUES_FROM, MODULE_IRI;
             RESTRICTION => (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The property that determines the class that a universal property restriction refers to.")
            .into(),
        rdf!(property ANNOTATED_PROPERTY, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::RESOURCE) =>
             (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The property that determines the predicate of an annotated axiom or annotated annotation.")
            .into(),
        rdf!(property ANNOTATED_SOURCE, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::RESOURCE) =>
             (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The property that determines the subject of an annotated axiom or annotated annotation.")
            .into(),
        rdf!(property ANNOTATED_TARGET, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::RESOURCE) =>
             (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The property that determines the object of an annotated axiom or annotated annotation.")
            .into(),
        rdf!(property ASSERTION_PROPERTY, MODULE_IRI;
             NEGATIVE_PROPERTY_ASSERTION => (rdf::MODULE_NAME, rdf::PROPERTY))
            .with_comment("The property that determines the predicate of a negative property assertion.")
            .into(),
        rdf!(property BACKWARD_COMPATIBLE_WITH, MODULE_IRI;
             ONTOLOGY => ONTOLOGY)
            .with_comment("The annotation property that indicates that a given ontology is backward compatible with another ontology.")
            .into(),
        rdf!(property BOTTOM_DATA_PROPERTY, MODULE_IRI;
             THING => (rdfs::MODULE_NAME, rdfs::LITERAL))
            .with_comment("The data property that does not relate any individual to any data value.")
            .into(),
        rdf!(property CARDINALITY, MODULE_IRI;
             RESTRICTION => (xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER))
            .with_comment("The property that determines the cardinality of an exact cardinality restriction.")
            .into(),
        rdf!(property COMPLEMENT_OF, MODULE_IRI;
             CLASS => CLASS)
            .with_comment("The property that determines that a given class is the complement of another class.")
            .into(),
        rdf!(property DATATYPE_COMPLEMENT_OF, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE) =>
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
            .with_comment("The property that determines that a given data range is the complement of another data range with respect to the data domain.")
            .into(),
        rdf!(property DEPRECATED, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::RESOURCE) =>
             (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The annotation property that indicates that a given entity has been deprecated.")
            .into(),
        rdf!(property DIFFERENT_FROM, MODULE_IRI; THING => THING)
            .with_comment("The property that determines that two given individuals are different.")
            .into(),
        rdf!(property DISJOINT_UNION_OF, MODULE_IRI;
             CLASS => (rdfs::MODULE_NAME, rdf::LIST))
            .with_comment("The property that determines that a given class is equivalent to the disjoint union of a collection of other classes.")
            .into(),
        rdf!(property DISJOINT_WITH, MODULE_IRI;
             CLASS => CLASS)
            .with_comment("The property that determines that two given classes are disjoint.")
            .into(),
        rdf!(property DISTINCT_MEMBERS, MODULE_IRI;
        ALL_DIFFERENT => (rdf::MODULE_NAME, rdf::LIST))
            .with_comment("The property that determines the collection of pairwise different individuals in a owl:AllDifferent axiom.")
            .into(),
        rdf!(property EQUIVALENT_CLASS, MODULE_IRI;
             CLASS => CLASS)
            .with_comment("The property that determines that two given classes are equivalent, and that is used to specify datatype definitions.")
            .into(),
        rdf!(property EQUIVALENT_PROPERTY, MODULE_IRI;
             CLASS => CLASS)
            .with_comment("The property that determines that two given properties are equivalent.")
            .into(),
        rdf!(property HAS_KEY, MODULE_IRI;
             CLASS => (rdf::MODULE_NAME, rdf::LIST))
            .with_comment("The property that determines the collection of properties that jointly build a key.")
            .into(),
        rdf!(property HAS_SELF, MODULE_IRI;
             RESTRICTION => (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The property that determines the property that a self restriction refers to.")
            .into(),
        rdf!(property HAS_VALUE, MODULE_IRI;
             RESTRICTION => (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The property that determines the property that a has-value restriction refers to.")
            .into(),
        rdf!(property IMPORTS, MODULE_IRI;
             ONTOLOGY => ONTOLOGY)
            .with_comment("The property that is used for importing other ontologies into a given ontology.")
            .into(),
        rdf!(property INCOMPATIBLE_WITH, MODULE_IRI; ONTOLOGY => ONTOLOGY)
            .with_comment("The annotation property that indicates that a given ontology is incompatible with another ontology.")
            .into(),
        rdf!(property INTERSECTION_OF, MODULE_IRI;
             CLASS => (rdf::MODULE_NAME, rdf::LIST))
            .with_comment("The property that determines the collection of classes or data ranges that build an intersection.")
            .into(),
        rdf!(property INVERSE_OF, MODULE_IRI; OBJECT_PROPERTY => OBJECT_PROPERTY)
            .with_comment("The property that determines that two given properties are inverse.")
            .into(),
        rdf!(property MAX_CARDINALITY, MODULE_IRI;
             RESTRICTION => (xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER))
            .with_comment("The property that determines the cardinality of a maximum cardinality restriction.")
            .into(),
        rdf!(property MAX_QUALIFIED_CARDINALITY, MODULE_IRI;
             RESTRICTION => (xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER))
            .with_comment("The property that determines the cardinality of a maximum qualified cardinality restriction.")
            .into(),
        rdf!(property MEMBERS, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::RESOURCE) =>
             (rdf::MODULE_NAME, rdf::LIST))
            .with_comment("The property that determines the collection of members in either a owl:AllDifferent, owl:AllDisjointClasses or owl:AllDisjointProperties axiom.")
            .into(),
        rdf!(property MIN_CARDINALITY, MODULE_IRI;
             RESTRICTION => (xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER))
            .with_comment("The property that determines the cardinality of a minimum cardinality restriction.")
            .into(),
        rdf!(property MIN_QUALIFIED_CARDINALITY, MODULE_IRI;
             RESTRICTION => (xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER))
            .with_comment("The property that determines the cardinality of a minimum qualified cardinality restriction.")
            .into(),
        rdf!(property ON_CLASS, MODULE_IRI;
             RESTRICTION => CLASS)
            .with_comment("The property that determines the class that a qualified object cardinality restriction refers to.")
            .into(),
        rdf!(property PROP_ON_DATA_RANGE_NAME, MODULE_IRI;
        RESTRICTION => (rdfs::MODULE_NAME, rdfs::DATATYPE))
            .with_comment("The property that determines the data range that a qualified data cardinality restriction refers to.")
            .into(),
        rdf!(property ON_DATATYPE, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE) =>
             (rdfs::MODULE_NAME, rdfs::DATATYPE))
            .with_comment("The property that determines the datatype that a datatype restriction refers to.")
            .into(),
        rdf!(property ONE_OF, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::CLASS) =>
             (rdf::MODULE_NAME, rdf::LIST))
            .with_comment("The property that determines the collection of individuals or data values that build an enumeration.")
            .into(),
        rdf!(property ON_PROPERTIES, MODULE_IRI;
             RESTRICTION => (rdf::MODULE_NAME, rdf::LIST))
            .with_comment("The property that determines the n-tuple of properties that a property restriction on an n-ary data range refers to.")
            .into(),
        rdf!(property ON_PROPERTY, MODULE_IRI;
             RESTRICTION => (rdf::MODULE_NAME, rdf::PROPERTY))
            .with_comment("The property that determines the property that a property restriction refers to.")
            .into(),
        rdf!(property PRIOR_VERSION, MODULE_IRI;
             ONTOLOGY => ONTOLOGY)
            .with_comment("The annotation property that indicates the predecessor ontology of a given ontology.")
            .into(),
        rdf!(property PROPERTY_CHAIN_AXIOM, MODULE_IRI;
             OBJECT_PROPERTY => (rdf::MODULE_NAME, rdf::LIST))
            .with_comment("The property that determines the n-tuple of properties that build a sub property chain of a given property.")
            .into(),
        rdf!(property PROPERTY_DISJOINT_WITH, MODULE_IRI;
             (rdf::MODULE_NAME, rdf::PROPERTY) =>
             (rdf::MODULE_NAME, rdf::PROPERTY))
            .with_comment("The property that determines that two given properties are disjoint.")
            .into(),
        rdf!(property QUALIFIED_CARDINALITY, MODULE_IRI;
             RESTRICTION => (xsd::MODULE_NAME, xsd::NONNEGATIVE_INTEGER))
            .with_comment("The property that determines the cardinality of an exact qualified cardinality restriction.")
            .into(),
        rdf!(property SAME_AS, MODULE_IRI;
             THING => THING)
            .with_comment("The property that determines that two given individuals are equal.")
            .into(),
        rdf!(property SOME_VALUES_FROM, MODULE_IRI;
             RESTRICTION => (rdfs::MODULE_NAME, rdfs::CLASS))
            .with_comment("The property that determines the class that an existential property restriction refers to.")
            .into(),
        rdf!(property SOURCE_INDIVIDUAL, MODULE_IRI;
             NEGATIVE_PROPERTY_ASSERTION => THING)
            .with_comment("The property that determines the subject of a negative property assertion.")
            .into(),
        rdf!(property TARGET_INDIVIDUAL, MODULE_IRI;
             NEGATIVE_PROPERTY_ASSERTION => THING)
            .with_comment("The property that determines the object of a negative property assertion.")
            .into(),
        rdf!(property TARGET_VALUE, MODULE_IRI;
             NEGATIVE_PROPERTY_ASSERTION => (rdfs::MODULE_NAME, rdfs::LITERAL))
            .with_comment("The property that determines the value of a negative data property assertion.")
            .into(),
        rdf!(property TOP_DATA_PROPERTY, MODULE_IRI;
             THING => (rdfs::MODULE_NAME, rdfs::LITERAL))
            .with_comment("The data property that relates every individual to every data value.")
            .into(),
        rdf!(property TOP_OBJECT_PROPERTY, MODULE_IRI;
             THING => THING)
            .with_comment("The object property that relates every two individuals.")
            .into(),
        rdf!(property UNION_OF, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::CLASS) =>
             (rdf::MODULE_NAME, rdf::LIST))
            .with_comment("The property that determines the collection of classes or data ranges that build a union.")
            .into(),
        rdf!(property VERSION_INFO, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::RESOURCE) =>
             (rdfs::MODULE_NAME, rdfs::RESOURCE))
            .with_comment("The annotation property that provides version information for an ontology or another OWL construct.")
            .into(),
        rdf!(property VERSION_IRI, MODULE_IRI;
             ONTOLOGY => ONTOLOGY)
            .with_comment("The object property that identifies the version IRI of an ontology.")
            .into(),
        rdf!(property WITH_RESTRICTIONS, MODULE_IRI;
             (rdfs::MODULE_NAME, rdfs::DATATYPE) =>
             (rdf::MODULE_NAME, rdf::LIST))
            .with_comment("The property that determines the collection of facet-value pairs that define a datatype restriction.")
             .into(),
    ]).unwrap();

    module
}
