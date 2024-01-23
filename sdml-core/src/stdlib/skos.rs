/*
This Rust module contains the SDML model of the SDML library module `skos`.
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

pub const MODULE_NAME: &str = "skos";
pub const MODULE_URL: &str = "http://www.w3.org/2004/02/skos/core#";

pub const CLASS_COLLECTION_NAME: &str = "Collection";
pub const CLASS_CONCEPT_NAME: &str = "Concept";
pub const CLASS_CONCEPT_SCHEME_NAME: &str = "ConceptScheme";
pub const CLASS_ORDERED_COLLECTION_NAME: &str = "OrderedCollection";

pub const PROP_ALT_LABEL_NAME: &str = "altLabel";
pub const PROP_BROAD_MATCH_NAME: &str = "broadMatch";
pub const PROP_BROADER_NAME: &str = "broader";
pub const PROP_BROADER_TRANSITIVE_NAME: &str = "broaderTransitive";
pub const PROP_CHANGE_NOTE_NAME: &str = "changeNote";
pub const PROP_CLOSE_MATCH_NAME: &str = "closeMatch";
pub const PROP_DEFINITION_NAME: &str = "definition";
pub const PROP_EDITORIAL_NOTE_NAME: &str = "editorialNote";
pub const PROP_EXACT_MATCH_NAME: &str = "exactMatch";
pub const PROP_EXAMPLE_NAME: &str = "example";
pub const PROP_HAS_TOP_CONCEPT_NAME: &str = "hasTopConcept";
pub const PROP_HIDDEN_LABEL_NAME: &str = "hiddenLabel";
pub const PROP_HISTORY_NOTE_NAME: &str = "historyNote";
pub const PROP_IN_SCHEME_NAME: &str = "inScheme";
pub const PROP_MAPPING_RELATION_NAME: &str = "mappingRelation";
pub const PROP_MEMBER_NAME: &str = "member";
pub const PROP_MEMBER_LIST_NAME: &str = "memberList";
pub const PROP_NARROW_MATCH_NAME: &str = "narrowMatch";
pub const PROP_NARROWER_NAME: &str = "narrower";
pub const PROP_NARROWER_TRANSITIVE_NAME: &str = "narrowerTransitive";
pub const PROP_NOTATION_NAME: &str = "notation";
pub const PROP_NOTE_NAME: &str = "note";
pub const PROP_PREF_LABEL_NAME: &str = "prefLabel";
pub const PROP_RELATED_NAME: &str = "related";
pub const PROP_RELATED_MATCH_NAME: &str = "relatedMatch";
pub const PROP_SCOPE_NOTE_NAME: &str = "scopeNote";
pub const PROP_SEMANTIC_RELATION_NAME: &str = "semanticRelation";
pub const PROP_TOP_CONCEPT_OF_NAME: &str = "topConceptOf";

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

    module.body_mut().extend_definitions(vec![
        rdf!(class CLASS_COLLECTION_NAME, MODULE_IRI)
            .with_predicate(
                qualid!(super::owl::MODULE_NAME, super::owl::PROP_DISJOINT_WITH_NAME),
                seq!(
                    idref!(CLASS_CONCEPT_NAME),
                    idref!(CLASS_CONCEPT_SCHEME_NAME)
                )
            )
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("Labelled collections can be used where you would like a set of concepts
to be displayed under a 'node label' in the hierarchy."@en)
            )
            .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                lstr!("Labelled collections can be used where you would like a set of concepts
to be displayed under a 'node label' in the hierarchy."@en)
            )
            .into(),
        rdf!(class CLASS_CONCEPT_NAME, MODULE_IRI)
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("An idea or notion; a unit of thought."@en)
            )
            .into(),
        rdf!(class CLASS_CONCEPT_SCHEME_NAME, MODULE_IRI)
            // Label: Concept Scheme
            .with_predicate(
                qualid!(super::owl::MODULE_NAME, super::owl::PROP_DISJOINT_WITH_NAME),
                idref!(CLASS_CONCEPT_NAME)
            )
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("A set of concepts, optionally including statements about semantic
relationships between those concepts."@en)
            )
            .with_predicate(
                id!(PROP_EXAMPLE_NAME),
                lstr!("Thesauri, classification schemes, subject heading lists, taxonomies,
'folksonomies', and other types of controlled vocabulary are all examples of concept schemes.
Concept schemes are also embedded in glossaries and terminologies."@en)
            )
            .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                lstr!("A concept scheme may be defined to include concepts from different
sources."@en)
            )
             .into(),
        rdf!(class CLASS_ORDERED_COLLECTION_NAME, MODULE_IRI; CLASS_COLLECTION_NAME)
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("An ordered collection of concepts, where both the grouping and the
ordering are meaningful."@en)
            )
            .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                lstr!("Ordered collections can be used where you would like a set of concepts
 to be displayed in a specific order, and optionally under a 'node label'."@en)
            )
             .into(),

        // Properties
        rdf!(property PROP_ALT_LABEL_NAME, MODULE_IRI, (super::rdfs::MODULE_NAME, super::rdfs::PROP_LABEL_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_ANNOTATION_PROPERTY_NAME))
            .with_comment(lstr!("skos:prefLabel, skos:altLabel and skos:hiddenLabel are
pairwise disjoint properties."@en))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("An alternative lexical label for a resource."@en)
            )
            .with_predicate(
                id!(PROP_EXAMPLE_NAME),
                lstr!("Acronyms, abbreviations, spelling variants, and irregular plural/singular
forms may be included among the alternative labels for a concept. Mis-spelled terms are normally
 included as hidden labels (see skos:hiddenLabel)"@en)
            )
            .into(),
        rdf!(property PROP_BROAD_MATCH_NAME, MODULE_IRI, PROP_BROADER_NAME, PROP_MAPPING_RELATION_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_predicate(
                qualid!(super::owl::MODULE_NAME, super::owl::PROP_INVERSE_OF_NAME),
                idref!(PROP_NARROW_MATCH_NAME)
            )
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("skos:broadMatch is used to state a hierarchical mapping link between
two conceptual resources in different concept schemes."@en)
            )
            .into(),
        rdf!(property PROP_BROADER_NAME, MODULE_IRI, PROP_BROADER_TRANSITIVE_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_predicate(
                qualid!(super::owl::MODULE_NAME, super::owl::PROP_INVERSE_OF_NAME),
                idref!(PROP_NARROWER_NAME)
            )
            .with_comment(lstr!("Broader concepts are typically rendered as parents in a
concept hierarchy (tree)."@en))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("Relates a concept to a concept that is more general in meaning."@en)
            )
            .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                lstr!("By convention, skos:broader is only used to assert an immediate
(i.e. direct) hierarchical link between two conceptual resources."@en)
            )
            .into(),
        rdf!(property PROP_BROADER_TRANSITIVE_NAME, MODULE_IRI, PROP_SEMANTIC_RELATION_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_TRANSITIVE_PROPERTY_NAME))
            .with_predicate(
                qualid!(super::owl::MODULE_NAME, super::owl::PROP_INVERSE_OF_NAME),
                idref!(PROP_NARROWER_TRANSITIVE_NAME)
            )
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("skos:broaderTransitive is a transitive superproperty of skos:broader."@en)
            )
            .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                lstr!("By convention, skos:broaderTransitive is not used to make assertions.
 Rather, the properties can be used to draw inferences about the transitive closure of the
 hierarchical relation, which is useful e.g. when implementing a simple query expansion
 algorithm in a search application."@en)
            )
            .into(),
        rdf!(property PROP_CHANGE_NOTE_NAME, MODULE_IRI, PROP_NOTE_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_ANNOTATION_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("A note about a modification to a concept."@en)
            )
            .into(),
        rdf!(property PROP_CLOSE_MATCH_NAME, MODULE_IRI, PROP_MAPPING_RELATION_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_SYMMETRIC_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("skos:closeMatch is used to link two concepts that are sufficiently similar
 that they can be used interchangeably in some information retrieval applications. In order to
avoid the possibility of \"compound errors\" when combining mappings across more than two concept
 schemes, skos:closeMatch is not declared to be a transitive property."@en)
            )
             .into(),
        rdf!(property PROP_DEFINITION_NAME, MODULE_IRI, PROP_NOTE_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_ANNOTATION_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("A statement or formal explanation of the meaning of a concept."@en)
            )
            .into(),
        rdf!(property PROP_EDITORIAL_NOTE_NAME, MODULE_IRI, PROP_NOTE_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_ANNOTATION_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("A note for an editor, translator or maintainer of the vocabulary."@en)
            )
            .into(),
        rdf!(property PROP_EXACT_MATCH_NAME, MODULE_IRI, PROP_CLOSE_MATCH_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_SYMMETRIC_PROPERTY_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_TRANSITIVE_PROPERTY_NAME))
            .with_comment(lstr!("skos:exactMatch is disjoint with each of the properties
skos:broadMatch and skos:relatedMatch."@en))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("skos:exactMatch is used to link two concepts, indicating a high
degree of confidence that the concepts can be used interchangeably across a wide range
of information retrieval applications. skos:exactMatch is a transitive property, and is
a sub-property of skos:closeMatch."@en)
            )
            .into(),
        rdf!(property PROP_EXAMPLE_NAME, MODULE_IRI, PROP_NOTE_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_ANNOTATION_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("An example of the use of a concept."@en)
            )
            .into(),
        rdf!(property PROP_HAS_TOP_CONCEPT_NAME, MODULE_IRI;
             CLASS_CONCEPT_SCHEME_NAME => CLASS_CONCEPT_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_predicate(
                qualid!(super::owl::MODULE_NAME, super::owl::PROP_INVERSE_OF_NAME),
                idref!(PROP_TOP_CONCEPT_OF_NAME)
            )
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("Relates, by convention, a concept scheme to a concept which is
topmost in the broader/narrower concept hierarchies for that scheme, providing an entry
 point to these hierarchies."@en)
            )
            .into(),
        rdf!(property PROP_HIDDEN_LABEL_NAME, MODULE_IRI, (super::rdfs::MODULE_NAME, super::rdfs::PROP_LABEL_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_ANNOTATION_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("A lexical label for a resource that should be hidden when generating
visual displays of the resource, but should still be accessible to free text search operations."@en)
            )
            .into(),
        rdf!(property PROP_HISTORY_NOTE_NAME, MODULE_IRI, PROP_NOTE_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_ANNOTATION_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("A note about the past state/use/meaning of a concept."@en)
            )
            .into(),
        rdf!(property PROP_IN_SCHEME_NAME, MODULE_IRI => CLASS_CONCEPT_SCHEME_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("Relates a resource (for example a concept) to a concept scheme in which
 it is included."@en)
            )
           .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                lstr!("A concept may be a member of more than one concept scheme."@en)
            )
            .into(),
        rdf!(property PROP_MAPPING_RELATION_NAME, MODULE_IRI, PROP_SEMANTIC_RELATION_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_comment(lstr!("These concept mapping relations mirror semantic relations,
 and the data model defined below is similar (with the exception of skos:exactMatch) to the
 data model defined for semantic relations. A distinct vocabulary is provided for concept
 mapping relations, to provide a convenient way to differentiate links within a concept scheme
 from links between concept schemes. However, this pattern of usage is not a formal requirement
 of the SKOS data model, and relies on informal definitions of best practice."@en))
             .with_predicate(
                 id!(PROP_DEFINITION_NAME),
                 lstr!("Relates two concepts coming, by convention, from different schemes,
 and that have comparable meanings."@en)
            )
            .into(),
        rdf!(property PROP_MEMBER_NAME, MODULE_IRI; CLASS_COLLECTION_NAME)
            // rdfs:range [ a owl:Class ; owl:unionOf (skos:Concept skos:Collection) ]
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("Relates a collection to one of its members."@en)
            )
            .into(),
        rdf!(property PROP_MEMBER_LIST_NAME, MODULE_IRI;
             CLASS_ORDERED_COLLECTION_NAME => (super::rdf::MODULE_NAME, super::rdf::CLASS_LIST_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_FUNCTIONAL_PROPERTY_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("Relates an ordered collection to the RDF list containing its members."@en)
            )
            .into(),
        rdf!(property PROP_NARROW_MATCH_NAME, MODULE_IRI, PROP_MAPPING_RELATION_NAME, PROP_NARROWER_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_predicate(
                qualid!(super::owl::MODULE_NAME, super::owl::PROP_INVERSE_OF_NAME),
                idref!(PROP_BROAD_MATCH_NAME)
            )
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("skos:narrowMatch is used to state a hierarchical mapping link between two
conceptual resources in different concept schemes."@en)
            )
            .into(),
        rdf!(property PROP_NARROWER_NAME, MODULE_IRI, PROP_NARROWER_TRANSITIVE_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_predicate(
                qualid!(super::owl::MODULE_NAME, super::owl::PROP_INVERSE_OF_NAME),
                idref!(PROP_BROADER_NAME)
            )
            .with_comment(lstr!("Narrower concepts are typically rendered as children in a
 concept hierarchy (tree)."@en))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("Relates a concept to a concept that is more specific in meaning."@en)
            )
            .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                lstr!("By convention, skos:broader is only used to assert an immediate (i.e.
 direct) hierarchical link between two conceptual resources."@en)
            )
            .into(),
        rdf!(property PROP_NARROWER_TRANSITIVE_NAME, MODULE_IRI, PROP_SEMANTIC_RELATION_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_TRANSITIVE_PROPERTY_NAME))
            .with_predicate(
                qualid!(super::owl::MODULE_NAME, super::owl::PROP_INVERSE_OF_NAME),
                idref!(PROP_BROADER_TRANSITIVE_NAME)
            )
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("skos:narrowerTransitive is a transitive superproperty of skos:narrower."@en)
            )
            .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                lstr!("By convention, skos:narrowerTransitive is not used to make assertions.
Rather, the properties can be used to draw inferences about the transitive closure of the
hierarchical relation, which is useful e.g. when implementing a simple query expansion
algorithm in a search application."@en)
            )
            .into(),
        rdf!(property PROP_NOTATION_NAME, MODULE_IRI)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_DATATYPE_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("A notation, also known as classification code, is a string of
characters such as \"T58.5\" or \"303.4833\" used to uniquely identify a concept within
the scope of a given concept scheme."@en)
            )
            .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                lstr!("By convention, skos:notation is used with a typed literal in the
object position of the triple."@en)
            )
            .into(),
        rdf!(property PROP_NOTE_NAME, MODULE_IRI)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_ANNOTATION_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("A general note, for any purpose."@en)
            )
             .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                 lstr!("This property may be used directly, or as a super-property for
more specific note types."@en)
            )
             .into(),
        rdf!(property PROP_PREF_LABEL_NAME, MODULE_IRI, (super::rdfs::MODULE_NAME, super::rdfs::PROP_LABEL_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_ANNOTATION_PROPERTY_NAME))
            .with_comment(lstr!("A resource has no more than one value of skos:prefLabel per
language tag, and no more than one value of skos:prefLabel without language tag."@en))
            .with_comment(lstr!("The range of skos:prefLabel is the class of RDF plain literals."@en))
            .with_comment(lstr!("skos:prefLabel, skos:altLabel and skos:hiddenLabel are pairwise
disjoint properties."@en))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("The preferred lexical label for a resource, in a given language."@en)
            )
            .into(),
        rdf!(property PROP_RELATED_NAME, MODULE_IRI, PROP_SEMANTIC_RELATION_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_SYMMETRIC_PROPERTY_NAME))
            .with_comment(lstr!("skos:related is disjoint with skos:broaderTransitive"@en))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("Relates a concept to a concept with which there is an associative
semantic relationship."@en)
            )
             .into(),
        rdf!(property PROP_RELATED_MATCH_NAME, MODULE_IRI, PROP_MAPPING_RELATION_NAME, PROP_RELATED_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_SYMMETRIC_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("skos:relatedMatch is used to state an associative mapping link between
two conceptual resources in different concept schemes."@en)
            )
            .into(),
        rdf!(property PROP_SCOPE_NOTE_NAME, MODULE_IRI, PROP_NOTE_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_ANNOTATION_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("A note that helps to clarify the meaning and/or the use of a concept."@en)
            )
            .into(),
        rdf!(property PROP_SEMANTIC_RELATION_NAME, MODULE_IRI;
             CLASS_CONCEPT_NAME => CLASS_CONCEPT_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("Links a concept to a concept related by meaning."@en)
            )
            .with_predicate(
                id!(PROP_SCOPE_NOTE_NAME),
                lstr!("This property should not be used directly, but as a super-property for
all properties denoting a relationship of meaning between concepts."@en)
            )
            .into(),
        rdf!(property PROP_TOP_CONCEPT_OF_NAME, MODULE_IRI, PROP_IN_SCHEME_NAME;
             CLASS_CONCEPT_NAME => CLASS_CONCEPT_SCHEME_NAME)
            .with_type(qualid!(super::owl::MODULE_NAME, super::owl::CLASS_OBJECT_PROPERTY_NAME))
            .with_predicate(
                qualid!(super::owl::MODULE_NAME, super::owl::PROP_INVERSE_OF_NAME),
                idref!(PROP_HAS_TOP_CONCEPT_NAME)
            )
            .with_predicate(
                id!(PROP_DEFINITION_NAME),
                lstr!("Relates a concept to the concept scheme that it is a top level concept of."@en)
            )
            .into(),
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
