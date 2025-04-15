/*!
This Rust module contains the SDML model of the SDML library module `owl` for OWL.
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

pub const RATIONAL: &str = "rational";
pub const REAL: &str = "real";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

module_function!(|| {
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked owl), module_uri ; call |module: Module|
        module.with_imports([import_statement!(
            id!(unchecked rdf),
            id!(unchecked rdfs),
            id!(unchecked xsd)
        )])
            .with_definitions([
                // ---------------------------------------------------------------------------------
                // Classes
                // ---------------------------------------------------------------------------------
                rdf!(id!(unchecked AllDifferent) ; class id!(unchecked rdfs:Resource) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("All Different"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of collections of pairwise different individuals."@en)),
                    ])).into(),
                rdf!(id!(unchecked AllDisjointClasses) ; class id!(unchecked rdfs:Resource) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("All Disjoint Classes"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of collections of pairwise disjoint classes."@en)),
                    ])).into(),
                rdf!(id!(unchecked AllDisjointProperties) ; class id!(unchecked rdfs:Resource) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("All Disjoint Properties"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of collections of pairwise disjoint properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked Annotation) ; class id!(unchecked rdfs:Resource) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Annotation"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of annotated annotations for which the RDF serialization consists of an annotated subject, predicate and object."@en)),
                    ])).into(),
                rdf!(id!(unchecked AnnotationProperty) ; class id!(unchecked rdf:Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Annotation Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of annotation properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked AsymmetricProperty) ; class id!(unchecked ObjectProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Asymmetric Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of asymmetric properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked Axiom) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Axiom"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of annotated axioms for which the RDF serialization consists of an annotated subject, predicate and object."@en)),
                    ])).into(),
                rdf!(id!(unchecked Class) ; class id!(unchecked rdfs:Class) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Class"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of OWL classes."@en)),
                    ])).into(),
                rdf!(id!(unchecked DataRange) ; class id!(unchecked rdfs:Datatype) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Data Range"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of OWL data ranges, which are special kinds of datatypes. Note: The use of the IRI owl:DataRange has been deprecated as of OWL 2. The IRI rdfs:Datatype SHOULD be used instead."@en)),
                    ])).into(),
                rdf!(id!(unchecked DatatypeProperty) ; class id!(unchecked rdf:Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Datatype Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of data properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked DeprecatedClass) ; class id!(unchecked rdfs:Class) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Deprecated Class"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of deprecated classes."@en)),
                    ])).into(),
                rdf!(id!(unchecked DeprecatedProperty) ; class id!(unchecked rdf:Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Deprecated Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of deprecated properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked FunctionalProperty) ; class id!(unchecked rdf:Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Functional Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of functional properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked InverseFunctionalProperty) ; class id!(unchecked rdf:Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Inverse Functional Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of inverse-functional properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked IrreflexiveFunctionalProperty) ; class id!(unchecked rdf:Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Irreflexive Functional Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of Irreflexive properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked NamedIndividual) ; class id!(unchecked owl:Thing) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Named Individual"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of named individuals."@en)),
                    ])).into(),
                rdf!(id!(unchecked NegativePropertyAssertion) ; class id!(unchecked rdfs:Resource) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Negative Property Assertion"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of negative property assertions."@en)),
                    ])).into(),
                rdf!(id!(unchecked NegativePropertyAssertion) ; class id!(unchecked rdfs:Resource) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Negative Property Assertion"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of negative property assertions."@en)),
                    ])).into(),
                rdf!(id!(unchecked Nothing) ; class id!(unchecked owl:Thing) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Nothing"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("This is the empty class."@en)),
                    ])).into(),
                rdf!(id!(unchecked ObjectProperty) ; class id!(unchecked rdf:Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Object Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of object properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked Ontology) ; class id!(unchecked rdfs:Resource) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Ontology"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of ontologies."@en)),
                    ])).into(),
                rdf!(id!(unchecked OntologyProperty) ; class id!(unchecked rdf:Property) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Ontology Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of ontology properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked Restriction) ; class id!(unchecked Class) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Restriction"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of property restrictions."@en)),
                    ])).into(),
                rdf!(id!(unchecked SymmetricProperty) ; class id!(unchecked ObjectProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Symmetric Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of symmetric properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked TransitiveProperty) ; class id!(unchecked ObjectProperty) ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Transitive Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of transitive properties."@en)),
                    ])).into(),
                rdf!(id!(unchecked Thing) ; class ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Thing"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The class of OWL individuals."@en)),
                    ])).into(),
                rdf!(id!(unchecked allValuesFrom) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Class)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("All Values From"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the class that a universal property restriction refers to."@en)),
                    ])).into(),
                rdf!(id!(unchecked annotatedProperty) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Annotated Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the predicate of an annotated axiom or annotated annotation."@en)),
                    ])).into(),
                rdf!(id!(unchecked annotatedSource) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Annotated Source"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the subject of an annotated axiom or annotated annotation."@en)),
                    ])).into(),
                rdf!(id!(unchecked annotatedTarget) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Annotated Target"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the object of an annotated axiom or annotated annotation."@en)),
                    ])).into(),
                rdf!(id!(unchecked assertionProperty) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked NegativePropertyAssertion)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:Property)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Assertion Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the predicate of a negative property assertion."@en)),
                    ])).into(),
                rdf!(id!(unchecked backwardCompatibleWith) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked AnnotationProperty)),
                        annotation!(id!(unchecked rdf:type), id!(unchecked OntologyProperty)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Ontology)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Ontology)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Backward Compatible With"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The annotation property that indicates that a given ontology is backward compatible with another ontology."@en)),
                    ])).into(),
                rdf!(id!(unchecked bottomDataProperty) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked DatatypeProperty)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Bottom Data Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The data property that does not relate any individual to any data value."@en)),
                    ])).into(),
                rdf!(id!(unchecked bottomObjectProperty) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked ObjectProperty)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Bottom Object Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The object property that does not relate any two individuals."@en)),
                    ])).into(),
                rdf!(id!(unchecked cardinality) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked xsd:nonNegativeInteger)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Cardinality"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the cardinality of an exact cardinality restriction."@en)),
                    ])).into(),
                rdf!(id!(unchecked complementOf) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Class)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Class)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Complement Of"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines that a given class is the complement of another class."@en)),
                    ])).into(),
                rdf!(id!(unchecked datatypeComplementOf) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Datatype)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Datatype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Datatype Complement Of"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines that a given data range is the complement of another data range with respect to the data domain."@en)),
                    ])).into(),
                rdf!(id!(unchecked deprecated) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Deprecated"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The annotation property that indicates that a given entity has been deprecated."@en)),
                    ])).into(),
                rdf!(id!(unchecked differentFrom) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Different From"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines that two given individuals are different."@en)),
                    ])).into(),
                rdf!(id!(unchecked disjointUnionOf) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Class)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:List)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Disjoint Union Of"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines that a given class is equivalent to the disjoint union of a collection of other classes."@en)),
                    ])).into(),
                rdf!(id!(unchecked disjointWith) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Class)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Class)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Disjoint With"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines that two given classes are disjoint."@en)),
                    ])).into(),
                rdf!(id!(unchecked distinctMembers) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked AllDifferent)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:List)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Distinct Members"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the collection of pairwise different individuals in a owl:AllDifferent axiom."@en)),
                    ])).into(),
                rdf!(id!(unchecked equivalentClass) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Class)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Class)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Equivalent Class"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines that two given classes are equivalent, and that is used to specify datatype definitions."@en)),
                    ])).into(),
                rdf!(id!(unchecked equivalentProperty) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdf:Property)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:Property)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Equivalent Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines that two given properties are equivalent."@en)),
                    ])).into(),
                rdf!(id!(unchecked hasKey) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Class)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:List)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Has Key"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the collection of properties that jointly build a key."@en)),
                    ])).into(),
                rdf!(id!(unchecked hasSelf) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Has Self"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the property that a self restriction refers to."@en)),
                    ])).into(),
                rdf!(id!(unchecked hasValue) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Has Value"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the individual that a has-value restriction refers to."@en)),
                    ])).into(),
                rdf!(id!(unchecked imports) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Ontology)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Ontology)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Improts"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that is used for importing other ontologies into a given ontology."@en)),
                    ])).into(),
                rdf!(id!(unchecked incompatibleWith) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked AnnotationProperty)),
                        annotation!(id!(unchecked rdf:type), id!(unchecked OntologyProperty)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Ontology)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Ontology)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Imports"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The annotation property that indicates that a given ontology is incompatible with another ontology."@en)),
                    ])).into(),
                rdf!(id!(unchecked intersectionOf) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Class)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:List)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Intersection Of"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the collection of classes or data ranges that build an intersection."@en)),
                    ])).into(),
                rdf!(id!(unchecked inverseOf) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked ObjectProperty)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked ObjectProperty)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Inverse Of"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines that two given properties are inverse."@en)),
                    ])).into(),
                rdf!(id!(unchecked maxCardinality) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked xsd:nonNegativeInteger)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Max Cardinality"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the cardinality of a maximum qualified cardinality restriction."@en)),
                    ])).into(),
                rdf!(id!(unchecked maxQualifiedCardinality) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked xsd:nonNegativeInteger)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Max Qualified Cardinality"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the cardinality of a maximum qualified cardinality restriction."@en)),
                    ])).into(),
                rdf!(id!(unchecked members) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:List)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Members"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the collection of members in either a owl:AllDifferent, owl:AllDisjointClasses or owl:AllDisjointProperties axiom."@en)),
                    ])).into(),
                rdf!(id!(unchecked minCardinality) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked xsd:nonNegativeInteger)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Min Cardinality"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the cardinality of a minimum cardinality restriction."@en)),
                    ])).into(),
                rdf!(id!(unchecked minQualifiedCardinality) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked xsd:nonNegativeInteger)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Min Qualified Cardinality"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the cardinality of a minimum qualified cardinality restriction."@en)),
                    ])).into(),
                rdf!(id!(unchecked onClass) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Class)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("On Class"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the class that a qualified object cardinality restriction refers to."@en)),
                    ])).into(),
                rdf!(id!(unchecked onDataRange) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Datatype)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("On Data Range"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the data range that a qualified data cardinality restriction refers to."@en)),
                    ])).into(),
                rdf!(id!(unchecked oneOf) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Class)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:List)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("One Of"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the collection of individuals or data values that build an enumeration."@en)),
                    ])).into(),
                rdf!(id!(unchecked onProperties) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:List)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("On Properties"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the n-tuple of properties that a property restriction on an n-ary data range refers to."@en)),
                    ])).into(),
                rdf!(id!(unchecked onProperty) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:Property)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("On Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the n-tuple of properties that a property restriction on an n-ary data range refers to."@en)),
                    ])).into(),
                rdf!(id!(unchecked priorVersion) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked AnnotationProperty)),
                        annotation!(id!(unchecked rdf:type), id!(unchecked OntologyProperty)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Ontology)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Ontology)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Prior Version"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The annotation property that indicates the predecessor ontology of a given ontology."@en)),
                    ])).into(),
                rdf!(id!(unchecked propertyChainAxiom) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked ObjectProperty)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:List)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Property Chain Axiom"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the n-tuple of properties that build a sub property chain of a given property."@en)),
                    ])).into(),
                rdf!(id!(unchecked propertyDisjointWith) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdf:Property)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:Property)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Property Disjoint With"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines that two given properties are disjoint."@en)),
                    ])).into(),
                rdf!(id!(unchecked qualifiedCardinality) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked xsd:nonNegativeInteger)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Qualified Cardinality"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the cardinality of an exact qualified cardinality restriction."@en)),
                    ])).into(),
                rdf!(id!(unchecked sameAs) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Same As"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines that two given individuals are equal."@en)),
                    ])).into(),
                rdf!(id!(unchecked someValuesFrom) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Restriction)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Class)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Some Values From"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the class that an existential property restriction refers to."@en)),
                    ])).into(),
                rdf!(id!(unchecked sourceIndividual) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked NegativePropertyAssertion)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Source Individual"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the subject of a negative property assertion."@en)),
                    ])).into(),
                rdf!(id!(unchecked targetIndividual) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked NegativePropertyAssertion)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Target Individual"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the object of a negative object property assertion."@en)),
                    ])).into(),
                rdf!(id!(unchecked targetValue) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked NegativePropertyAssertion)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Target Value"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the value of a negative data property assertion."@en)),
                    ])).into(),
                rdf!(id!(unchecked topDataProperty) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Literal)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Top Data Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The data property that relates every individual to every data value."@en)),
                    ])).into(),
                rdf!(id!(unchecked topObjectProperty) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Thing)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Top Object Property"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The object property that relates every two individuals."@en)),
                    ])).into(),
                rdf!(id!(unchecked unionOf) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Class)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:List)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Union Of"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the collection of classes or data ranges that build a union."@en)),
                    ])).into(),
                rdf!(id!(unchecked versionInfo) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked AnnotationProperty)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdfs:Resource)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Version Info"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The annotation property that provides version information for an ontology or another OWL construct."@en)),
                    ])).into(),
                rdf!(id!(unchecked versionIRI) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked OntologyProperty)),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Ontology)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked Ontology)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Version IRI"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that identifies the version IRI of an ontology."@en)),
                    ])).into(),
                rdf!(id!(unchecked withRestrictions) ; property ;
                    call |body: AnnotationOnlyBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked rdfs:Datatype)),
                        annotation!(id!(unchecked rdfs:range), id!(unchecked rdf:List)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("With Restrictions"@en)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked owl)),
                        annotation!(id!(unchecked rdfs:comment), rdf_str!("The property that determines the collection of facet-value pairs that define a datatype restriction."@en)),
                    ])).into(),
             ])
    )
});
