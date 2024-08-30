/**
This Rust module contains the SDML model of the SDML library module `skos` for SKOS.
*/

use crate::model::annotations::AnnotationBuilder;
use crate::model::modules::Module;
use crate::model::HasBody;
use crate::stdlib::{owl, rdf, rdfs};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const MODULE_NAME: &str = "skos";
pub const MODULE_URL: &str = "http://www.w3.org/2004/02/skos/core#";

pub const COLLECTION: &str = "Collection";
pub const CONCEPT: &str = "Concept";
pub const CONCEPT_SCHEME: &str = "ConceptScheme";
pub const ORDERED_COLLECTION: &str = "OrderedCollection";

pub const ALT_LABEL: &str = "altLabel";
pub const BROAD_MATCH: &str = "broadMatch";
pub const BROADER: &str = "broader";
pub const BROADER_TRANSITIVE: &str = "broaderTransitive";
pub const CHANGE_NOTE: &str = "changeNote";
pub const CLOSE_MATCH: &str = "closeMatch";
pub const DEFINITION: &str = "definition";
pub const EDITORIAL_NOTE: &str = "editorialNote";
pub const EXACT_MATCH: &str = "exactMatch";
pub const EXAMPLE: &str = "example";
pub const HAS_TOP_CONCEPT: &str = "hasTopConcept";
pub const HIDDEN_LABEL: &str = "hiddenLabel";
pub const HISTORY_NOTE: &str = "historyNote";
pub const IN_SCHEME: &str = "inScheme";
pub const MAPPING_RELATION: &str = "mappingRelation";
pub const MEMBER: &str = "member";
pub const MEMBER_LIST: &str = "memberList";
pub const NARROW_MATCH: &str = "narrowMatch";
pub const NARROWER: &str = "narrower";
pub const NARROWER_TRANSITIVE: &str = "narrowerTransitive";
pub const NOTATION: &str = "notation";
pub const NOTE: &str = "note";
pub const PREF_LABEL: &str = "prefLabel";
pub const RELATED: &str = "related";
pub const RELATED_MATCH: &str = "relatedMatch";
pub const SCOPE_NOTE: &str = "scopeNote";
pub const SEMANTIC_RELATION: &str = "semanticRelation";
pub const TOP_CONCEPT_OF: &str = "topConceptOf";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module() -> Module {
    #[allow(non_snake_case)]
    let MODULE_IRI: url::Url = url::Url::parse(MODULE_URL).unwrap();
    let mut module = Module::empty(id!(MODULE_NAME)).with_base_uri(Url::parse(MODULE_URL).unwrap());

    module
        .body_mut()
        .add_to_imports(import!(id!(rdf::MODULE_NAME), id!(rdfs::MODULE_NAME)));

    module.body_mut().extend_definitions(vec![
        rdf!(class COLLECTION, MODULE_IRI)
            .with_predicate(
                qualid!(owl::MODULE_NAME, owl::DISJOINT_WITH),
                seq!(
                    idref!(CONCEPT),
                    idref!(CONCEPT_SCHEME)
                )
            )
            .with_predicate(
                id!(DEFINITION),
                lstr!("Labelled collections can be used where you would like a set of concepts
to be displayed under a 'node label' in the hierarchy."@en)
            )
            .with_predicate(
                id!(SCOPE_NOTE),
                lstr!("Labelled collections can be used where you would like a set of concepts
to be displayed under a 'node label' in the hierarchy."@en)
            )
            .into(),
        rdf!(class CONCEPT, MODULE_IRI)
            .with_predicate(
                id!(DEFINITION),
                lstr!("An idea or notion; a unit of thought."@en)
            )
            .into(),
        rdf!(class CONCEPT_SCHEME, MODULE_IRI)
            // Label: Concept Scheme
            .with_predicate(
                qualid!(owl::MODULE_NAME, owl::DISJOINT_WITH),
                idref!(CONCEPT)
            )
            .with_predicate(
                id!(DEFINITION),
                lstr!("A set of concepts, optionally including statements about semantic
relationships between those concepts."@en)
            )
            .with_predicate(
                id!(EXAMPLE),
                lstr!("Thesauri, classification schemes, subject heading lists, taxonomies,
'folksonomies', and other types of controlled vocabulary are all examples of concept schemes.
Concept schemes are also embedded in glossaries and terminologies."@en)
            )
            .with_predicate(
                id!(SCOPE_NOTE),
                lstr!("A concept scheme may be defined to include concepts from different
sources."@en)
            )
             .into(),
        rdf!(class ORDERED_COLLECTION, MODULE_IRI; COLLECTION)
            .with_predicate(
                id!(DEFINITION),
                lstr!("An ordered collection of concepts, where both the grouping and the
ordering are meaningful."@en)
            )
            .with_predicate(
                id!(SCOPE_NOTE),
                lstr!("Ordered collections can be used where you would like a set of concepts
 to be displayed in a specific order, and optionally under a 'node label'."@en)
            )
             .into(),

        // Properties
        rdf!(property ALT_LABEL, MODULE_IRI, (rdfs::MODULE_NAME, rdfs::LABEL))
            .with_type(qualid!(owl::MODULE_NAME, owl::ANNOTATION_PROPERTY))
            .with_comment(lstr!("skos:prefLabel, skos:altLabel and skos:hiddenLabel are
pairwise disjoint properties."@en))
            .with_predicate(
                id!(DEFINITION),
                lstr!("An alternative lexical label for a resource."@en)
            )
            .with_predicate(
                id!(EXAMPLE),
                lstr!("Acronyms, abbreviations, spelling variants, and irregular plural/singular
forms may be included among the alternative labels for a concept. Mis-spelled terms are normally
 included as hidden labels (see skos:hiddenLabel)"@en)
            )
            .into(),
        rdf!(property BROAD_MATCH, MODULE_IRI, BROADER, MAPPING_RELATION)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_predicate(
                qualid!(owl::MODULE_NAME, owl::INVERSE_OF),
                idref!(NARROW_MATCH)
            )
            .with_predicate(
                id!(DEFINITION),
                lstr!("skos:broadMatch is used to state a hierarchical mapping link between
two conceptual resources in different concept schemes."@en)
            )
            .into(),
        rdf!(property BROADER, MODULE_IRI, BROADER_TRANSITIVE)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_predicate(
                qualid!(owl::MODULE_NAME, owl::INVERSE_OF),
                idref!(NARROWER)
            )
            .with_comment(lstr!("Broader concepts are typically rendered as parents in a
concept hierarchy (tree)."@en))
            .with_predicate(
                id!(DEFINITION),
                lstr!("Relates a concept to a concept that is more general in meaning."@en)
            )
            .with_predicate(
                id!(SCOPE_NOTE),
                lstr!("By convention, skos:broader is only used to assert an immediate
(i.e. direct) hierarchical link between two conceptual resources."@en)
            )
            .into(),
        rdf!(property BROADER_TRANSITIVE, MODULE_IRI, SEMANTIC_RELATION)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_type(qualid!(owl::MODULE_NAME, owl::TRANSITIVE_PROPERTY))
            .with_predicate(
                qualid!(owl::MODULE_NAME, owl::INVERSE_OF),
                idref!(NARROWER_TRANSITIVE)
            )
            .with_predicate(
                id!(DEFINITION),
                lstr!("skos:broaderTransitive is a transitive superproperty of skos:broader."@en)
            )
            .with_predicate(
                id!(SCOPE_NOTE),
                lstr!("By convention, skos:broaderTransitive is not used to make assertions.
 Rather, the properties can be used to draw inferences about the transitive closure of the
 hierarchical relation, which is useful e.g. when implementing a simple query expansion
 algorithm in a search application."@en)
            )
            .into(),
        rdf!(property CHANGE_NOTE, MODULE_IRI, NOTE)
            .with_type(qualid!(owl::MODULE_NAME, owl::ANNOTATION_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("A note about a modification to a concept."@en)
            )
            .into(),
        rdf!(property CLOSE_MATCH, MODULE_IRI, MAPPING_RELATION)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_type(qualid!(owl::MODULE_NAME, owl::SYMMETRIC_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("skos:closeMatch is used to link two concepts that are sufficiently similar
 that they can be used interchangeably in some information retrieval applications. In order to
avoid the possibility of \"compound errors\" when combining mappings across more than two concept
 schemes, skos:closeMatch is not declared to be a transitive property."@en)
            )
             .into(),
        rdf!(property DEFINITION, MODULE_IRI, NOTE)
            .with_type(qualid!(owl::MODULE_NAME, owl::ANNOTATION_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("A statement or formal explanation of the meaning of a concept."@en)
            )
            .into(),
        rdf!(property EDITORIAL_NOTE, MODULE_IRI, NOTE)
            .with_type(qualid!(owl::MODULE_NAME, owl::ANNOTATION_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("A note for an editor, translator or maintainer of the vocabulary."@en)
            )
            .into(),
        rdf!(property EXACT_MATCH, MODULE_IRI, CLOSE_MATCH)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_type(qualid!(owl::MODULE_NAME, owl::SYMMETRIC_PROPERTY))
            .with_type(qualid!(owl::MODULE_NAME, owl::TRANSITIVE_PROPERTY))
            .with_comment(lstr!("skos:exactMatch is disjoint with each of the properties
skos:broadMatch and skos:relatedMatch."@en))
            .with_predicate(
                id!(DEFINITION),
                lstr!("skos:exactMatch is used to link two concepts, indicating a high
degree of confidence that the concepts can be used interchangeably across a wide range
of information retrieval applications. skos:exactMatch is a transitive property, and is
a sub-property of skos:closeMatch."@en)
            )
            .into(),
        rdf!(property EXAMPLE, MODULE_IRI, NOTE)
            .with_type(qualid!(owl::MODULE_NAME, owl::ANNOTATION_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("An example of the use of a concept."@en)
            )
            .into(),
        rdf!(property HAS_TOP_CONCEPT, MODULE_IRI;
             CONCEPT_SCHEME => CONCEPT)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_predicate(
                qualid!(owl::MODULE_NAME, owl::INVERSE_OF),
                idref!(TOP_CONCEPT_OF)
            )
            .with_predicate(
                id!(DEFINITION),
                lstr!("Relates, by convention, a concept scheme to a concept which is
topmost in the broader/narrower concept hierarchies for that scheme, providing an entry
 point to these hierarchies."@en)
            )
            .into(),
        rdf!(property HIDDEN_LABEL, MODULE_IRI, (rdfs::MODULE_NAME, rdfs::LABEL))
            .with_type(qualid!(owl::MODULE_NAME, owl::ANNOTATION_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("A lexical label for a resource that should be hidden when generating
visual displays of the resource, but should still be accessible to free text search operations."@en)
            )
            .into(),
        rdf!(property HISTORY_NOTE, MODULE_IRI, NOTE)
            .with_type(qualid!(owl::MODULE_NAME, owl::ANNOTATION_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("A note about the past state/use/meaning of a concept."@en)
            )
            .into(),
        rdf!(property IN_SCHEME, MODULE_IRI => CONCEPT_SCHEME)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("Relates a resource (for example a concept) to a concept scheme in which
 it is included."@en)
            )
           .with_predicate(
                id!(SCOPE_NOTE),
                lstr!("A concept may be a member of more than one concept scheme."@en)
            )
            .into(),
        rdf!(property MAPPING_RELATION, MODULE_IRI, SEMANTIC_RELATION)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_comment(lstr!("These concept mapping relations mirror semantic relations,
 and the data model defined below is similar (with the exception of skos:exactMatch) to the
 data model defined for semantic relations. A distinct vocabulary is provided for concept
 mapping relations, to provide a convenient way to differentiate links within a concept scheme
 from links between concept schemes. However, this pattern of usage is not a formal requirement
 of the SKOS data model, and relies on informal definitions of best practice."@en))
             .with_predicate(
                 id!(DEFINITION),
                 lstr!("Relates two concepts coming, by convention, from different schemes,
 and that have comparable meanings."@en)
            )
            .into(),
        rdf!(property MEMBER, MODULE_IRI; COLLECTION)
            // rdfs:range [ a owl:Class ; owl:unionOf (skos:Concept skos:Collection) ]
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("Relates a collection to one of its members."@en)
            )
            .into(),
        rdf!(property MEMBER_LIST, MODULE_IRI;
             ORDERED_COLLECTION => (rdf::MODULE_NAME, rdf::LIST))
            .with_type(qualid!(owl::MODULE_NAME, owl::FUNCTIONAL_PROPERTY))
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("Relates an ordered collection to the RDF list containing its members."@en)
            )
            .into(),
        rdf!(property NARROW_MATCH, MODULE_IRI, MAPPING_RELATION, NARROWER)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_predicate(
                qualid!(owl::MODULE_NAME, owl::INVERSE_OF),
                idref!(BROAD_MATCH)
            )
            .with_predicate(
                id!(DEFINITION),
                lstr!("skos:narrowMatch is used to state a hierarchical mapping link between two
conceptual resources in different concept schemes."@en)
            )
            .into(),
        rdf!(property NARROWER, MODULE_IRI, NARROWER_TRANSITIVE)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_predicate(
                qualid!(owl::MODULE_NAME, owl::INVERSE_OF),
                idref!(BROADER)
            )
            .with_comment(lstr!("Narrower concepts are typically rendered as children in a
 concept hierarchy (tree)."@en))
            .with_predicate(
                id!(DEFINITION),
                lstr!("Relates a concept to a concept that is more specific in meaning."@en)
            )
            .with_predicate(
                id!(SCOPE_NOTE),
                lstr!("By convention, skos:broader is only used to assert an immediate (i.e.
 direct) hierarchical link between two conceptual resources."@en)
            )
            .into(),
        rdf!(property NARROWER_TRANSITIVE, MODULE_IRI, SEMANTIC_RELATION)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_type(qualid!(owl::MODULE_NAME, owl::TRANSITIVE_PROPERTY))
            .with_predicate(
                qualid!(owl::MODULE_NAME, owl::INVERSE_OF),
                idref!(BROADER_TRANSITIVE)
            )
            .with_predicate(
                id!(DEFINITION),
                lstr!("skos:narrowerTransitive is a transitive superproperty of skos:narrower."@en)
            )
            .with_predicate(
                id!(SCOPE_NOTE),
                lstr!("By convention, skos:narrowerTransitive is not used to make assertions.
Rather, the properties can be used to draw inferences about the transitive closure of the
hierarchical relation, which is useful e.g. when implementing a simple query expansion
algorithm in a search application."@en)
            )
            .into(),
        rdf!(property NOTATION, MODULE_IRI)
            .with_type(qualid!(owl::MODULE_NAME, owl::DATATYPE_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("A notation, also known as classification code, is a string of
characters such as \"T58.5\" or \"303.4833\" used to uniquely identify a concept within
the scope of a given concept scheme."@en)
            )
            .with_predicate(
                id!(SCOPE_NOTE),
                lstr!("By convention, skos:notation is used with a typed literal in the
object position of the triple."@en)
            )
            .into(),
        rdf!(property NOTE, MODULE_IRI)
            .with_type(qualid!(owl::MODULE_NAME, owl::ANNOTATION_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("A general note, for any purpose."@en)
            )
             .with_predicate(
                id!(SCOPE_NOTE),
                 lstr!("This property may be used directly, or as a super-property for
more specific note types."@en)
            )
             .into(),
        rdf!(property PREF_LABEL, MODULE_IRI, (rdfs::MODULE_NAME, rdfs::LABEL))
            .with_type(qualid!(owl::MODULE_NAME, owl::ANNOTATION_PROPERTY))
            .with_comment(lstr!("A resource has no more than one value of skos:prefLabel per
language tag, and no more than one value of skos:prefLabel without language tag."@en))
            .with_comment(lstr!("The range of skos:prefLabel is the class of RDF plain literals."@en))
            .with_comment(lstr!("skos:prefLabel, skos:altLabel and skos:hiddenLabel are pairwise
disjoint properties."@en))
            .with_predicate(
                id!(DEFINITION),
                lstr!("The preferred lexical label for a resource, in a given language."@en)
            )
            .into(),
        rdf!(property RELATED, MODULE_IRI, SEMANTIC_RELATION)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_type(qualid!(owl::MODULE_NAME, owl::SYMMETRIC_PROPERTY))
            .with_comment(lstr!("skos:related is disjoint with skos:broaderTransitive"@en))
            .with_predicate(
                id!(DEFINITION),
                lstr!("Relates a concept to a concept with which there is an associative
semantic relationship."@en)
            )
             .into(),
        rdf!(property RELATED_MATCH, MODULE_IRI, MAPPING_RELATION, RELATED)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_type(qualid!(owl::MODULE_NAME, owl::SYMMETRIC_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("skos:relatedMatch is used to state an associative mapping link between
two conceptual resources in different concept schemes."@en)
            )
            .into(),
        rdf!(property SCOPE_NOTE, MODULE_IRI, NOTE)
            .with_type(qualid!(owl::MODULE_NAME, owl::ANNOTATION_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("A note that helps to clarify the meaning and/or the use of a concept."@en)
            )
            .into(),
        rdf!(property SEMANTIC_RELATION, MODULE_IRI;
             CONCEPT => CONCEPT)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_predicate(
                id!(DEFINITION),
                lstr!("Links a concept to a concept related by meaning."@en)
            )
            .with_predicate(
                id!(SCOPE_NOTE),
                lstr!("This property should not be used directly, but as a super-property for
all properties denoting a relationship of meaning between concepts."@en)
            )
            .into(),
        rdf!(property TOP_CONCEPT_OF, MODULE_IRI, IN_SCHEME;
             CONCEPT => CONCEPT_SCHEME)
            .with_type(qualid!(owl::MODULE_NAME, owl::OBJECT_PROPERTY))
            .with_predicate(
                qualid!(owl::MODULE_NAME, owl::INVERSE_OF),
                idref!(HAS_TOP_CONCEPT)
            )
            .with_predicate(
                id!(DEFINITION),
                lstr!("Relates a concept to the concept scheme that it is a top level concept of."@en)
            )
            .into(),
    ]).unwrap();

    module
}
