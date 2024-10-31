/**
This Rust module contains the SDML model of the SDML library module `skos` for SKOS.
*/
use crate::model::annotations::HasAnnotations;
use crate::model::definitions::{StructureBody, UnionBody};
use crate::model::modules::{Module, ModuleBody};
use crate::model::values::SimpleValue;
use std::str::FromStr;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros ❱  Identifiers
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! id {
    (unchecked $id1:ident : $id2:ident) => {
        $crate::model::identifiers::QualifiedIdentifier::new_unchecked(
            stringify!($id1),
            stringify!($id2),
        )
    };
    (unchecked $id1:expr, $id2:expr) => {
        $crate::model::identifiers::QualifiedIdentifier::new_unchecked($id1, $id2)
    };
    (unchecked $id:ident) => {
        $crate::model::identifiers::Identifier::new_unchecked(stringify!($id))
    };
    (unchecked $id:expr) => {
        $crate::model::identifiers::Identifier::new_unchecked($id)
    };
    ($id1:ident, $id2:ident) => {
        $crate::model::identifiers::QualifiedIdentifier::from_str(&format!("{}:{}", $id1, $id2))
            .unwrap()
    };
    ($id1:expr, $id2:expr) => {
        $crate::model::identifiers::QualifiedIdentifier::from_str(&format!(
            "{}:{}",
            stringify!($id1),
            stringify!($id2)
        ))
        .unwrap()
    };
    ($id:ident) => {
        $crate::model::identifiers::Identifier::from_str(stringify!($id))
    };
    ($id:expr) => {
        $crate::model::identifiers::Identifier::from_str($id)
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱  Values
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! lang {
    ($name:ident) => {
        $crate::model::values::LanguageTag::from_str(stringify!($name)).unwrap()
    };
    ($name:expr) => {
        $crate::model::values::LanguageTag::from_str($name).unwrap()
    };
}

#[macro_export]
macro_rules! rdf_str {
    ($text:literal @ $lang:ident) => {
        $crate::model::values::LanguageString::new($text, Some(lang!($lang)))
    };
    ($text:literal @ $lang:ident) => {
        $crate::model::values::LanguageString::new($text, Some(lang!($lang)))
    };
}

#[macro_export]
macro_rules! v {
    ($value:literal) => {
        $crate::model::values::SimpleValue::from($value)
    };
    ($key:literal => $value:literal) => {
        $crate::model::values::MappingValue::new(SimpleValue::from($key), Value::from($value))
    };
    ($name:expr, $value:expr) => {
        $crate::model::values::ValueConstructor::new(
            IdentifierReference::from($name),
            SimpleValue::from($value),
        )
    };
    ($name:expr) => {
        $crate::model::values::IdentifierReference::from($name)
    };
}

#[macro_export]
macro_rules! vs {
    ($( $value:expr ),*) => {
        vs!($( $value, )*)
    };
    ($( $value:expr, )*) => {
        $crate::model::values::SequenceOfValues::from_iter([
            $(
                $crate::model::values::SequenceMember::from($value),
            )*
        ])
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱  Cardinalities
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! range {
    () => {
        $crate::model::members::CardinalityRange::default()
    };
    ($only:literal) => {
        $crate::model::members::CardinalityRange::new_single($only)
    };
    ($min:literal .. ) => {
        $crate::model::members::CardinalityRange::new_unbounded($min)
    };
    ($min:literal .. $max:literal) => {
        $crate::model::members::CardinalityRange::new_range($min, $max)
    };
}

#[macro_export]
macro_rules! cardinality {
    (ordered) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            None,
            $crate::model::members::CardinalityRange::default(),
        )
    };
    (ordered, $only:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            None,
            $crate::model::members::CardinalityRange::new_single($only),
        )
    };
    (ordered, $min:literal .. ) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            None,
            $crate::model::members::CardinalityRange::new_unbounded($min),
        )
    };
    (ordered, $min:literal .. $max:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            None,
            $crate::model::members::CardinalityRange::new_range($min, $max),
        )
    };
    // ===============================================
    (ordered, unique) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            Some($crate::model::members::Uniqueness::Unique),
            $crate::model::members::CardinalityRange::default(),
        )
    };
    (ordered, unique, $only:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            Some($crate::model::members::Uniqueness::Unique),
            $crate::model::members::CardinalityRange::new_single($only),
        )
    };
    (ordered, unique, $min:literal .. ) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            Some($crate::model::members::Uniqueness::Unique),
            $crate::model::members::CardinalityRange::new_unbounded($min),
        )
    };
    (ordered, unique, $min:literal .. $max:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            Some($crate::model::members::Uniqueness::Unique),
            $crate::model::members::CardinalityRange::new_range($min, $max),
        )
    };
    // ===============================================
    (ordered, nonunique) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::default(),
        )
    };
    (ordered, nonunique, $only:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::ardinalityRange::new_single($only),
        )
    };
    (ordered, nonunique, $min:literal .. ) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::new_unbounded($min),
        )
    };
    (ordered, nonunique, $min:literal .. $max:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Ordered),
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::new_range($min, $max),
        )
    };
    // ===============================================
    // ===============================================
    (unordered) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            None,
            $crate::model::members::CardinalityRange::default(),
        )
    };
    (unordered, $only:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            None,
            $crate::model::members::CardinalityRange::new_single($only),
        )
    };
    (unordered, $min:literal .. ) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            None,
            $crate::model::members::CardinalityRange::new_unbounded($min),
        )
    };
    (unordered, $min:literal .. $max:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            None,
            $crate::model::members::CardinalityRange::new_range($min, $max),
        )
    };
    // ===============================================
    (unordered, unique) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            Some($crate::model::members::Uniqueness::Unique),
            C$crate::model::members::ardinalityRange::default(),
        )
    };
    (unordered, unique, $only:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            Some($crate::model::members::Uniqueness::Unique),
            CardinalityRange::new_single($only),
        )
    };
    (unordered, unique, $min:literal .. ) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            Some($crate::model::members::Uniqueness::Unique),
            CardinalityRange::new_unbounded($min),
        )
    };
    (unordered, unique, $min:literal .. $max:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            Some($crate::model::members::Uniqueness::Unique),
            $crate::model::members::CardinalityRange::new_range($min, $max),
        )
    };
    // ===============================================
    (unordered, nonunique) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::default(),
        )
    };
    (unordered, nonunique, $only:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::new_single($only),
        )
    };
    (unordered, nonunique, $min:literal .. ) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::new_unbounded($min),
        )
    };
    (unordered, nonunique, $min:literal .. $max:literal) => {
        $crate::model::members::Cardinality::new(
            Some($crate::model::members::Ordering::Unordered),
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::new_range($min, $max),
        )
    };
    // ===============================================
    // ===============================================
    (unique) => {
        $crate::model::members::Cardinality::new(
            None,
            Some($crate::model::members::Uniqueness::Unique),
            $crate::model::members::CardinalityRange::default(),
        )
    };
    (unique, $only:literal) => {
        $crate::model::members::Cardinality::new(
            None,
            Some($crate::model::members::Uniqueness::Unique),
            $crate::model::members::CardinalityRange::new_single($only),
        )
    };
    (unique, $min:literal .. ) => {
        $crate::model::members::Cardinality::new(
            None,
            Some($crate::model::members::Uniqueness::Unique),
            $crate::model::members::CardinalityRange::new_unbounded($min),
        )
    };
    (unique, $min:literal .. $max:literal) => {
        $crate::model::members::Cardinality::new(
            None,
            Some($crate::model::members::Uniqueness::Unique),
            $crate::model::members::CardinalityRange::new_range($min, $max),
        )
    };
    // ===============================================
    // ===============================================
    (nonunique) => {
        $crate::model::members::Cardinality::new(
            None,
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::default(),
        )
    };
    (nonunique, $only:literal) => {
        $crate::model::members::Cardinality::new(
            None,
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::new_single($only),
        )
    };
    (nonunique, $min:literal .. ) => {
        $crate::model::members::Cardinality::new(
            None,
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::new_unbounded($min),
        )
    };
    (nonunique, $min:literal .. $max:literal) => {
        $crate::model::members::Cardinality::new(
            None,
            Some($crate::model::members::Uniqueness::NonUnique),
            $crate::model::members::CardinalityRange::new_range($min, $max),
        )
    };
    // ===============================================
    // ===============================================
    () => {
        $crate::model::members::Cardinality::new(
            None,
            None,
            $crate::model::members::CardinalityRange::default(),
        )
    };
    ($only:literal) => {
        $crate::model::members::Cardinality::new(
            None,
            None,
            $crate::model::members::CardinalityRange::new_single($only),
        )
    };
    ($min:literal .. ) => {
        $crate::model::members::Cardinality::new(
            None,
            None,
            $crate::model::members::CardinalityRange::new_unbounded($min),
        )
    };
    ($min:literal .. $max:literal) => {
        $crate::model::members::Cardinality::new(
            None,
            None,
            $crate::model::members::CardinalityRange::new_range($min, $max),
        )
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Annotations
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! annotation {
    ($name:expr, $value:literal) => {
        $crate::model::annotations::AnnotationProperty::new($name, SimpleValue::from($value)).into()
    };
    ($name:expr, $value:expr) => {
        $crate::model::annotations::AnnotationProperty::new($name, $value).into()
    };
    ($name:literal => $value:literal) => {
        $crate::model::annotations::AnnotationProperty::new(id!(unchecked $name), SimpleValue::from($value)).into()
    };
    ($name:expr => $value:literal) => {
        $crate::model::annotations::AnnotationProperty::new($name, SimpleValue::from($value)).into()
    };
    ($name:expr => $value:expr) => {
        $crate::model::annotations::AnnotationProperty::new($name, $value).into()
    };
}

#[macro_export]
macro_rules! annotation_body {
    ($( $annotation:expr, )*) => {
        $crate::model::annotations::AnnotationOnlyBody::from(vec![
            $(
                $annotation,
            )*
        ])
    };
    ($( $annotation:expr ),*) => {
        annotation_body!($( $annotation, )*)
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Modules & Imports
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! module {
    ($name:expr, $base_uri:expr ; call $body_fn:expr) => {{
        let body = $body_fn($crate::model::modules::ModuleBody::default());
        Module::new($name, body).with_base_uri($base_uri)
    }};
    ($name:expr, $base_uri:expr ; $body:expr) => {
        Module::new($name, $body).with_base_uri($base_uri)
    };
    ($name:expr ; call $body_fn:expr) => {{
        let body = $body_fn($crate::model::modules::ModuleBody::default());
        Module::new($name, body)
    }};
    ($name:expr ; $body:expr) => {
        Module::new($name, $body)
    };
}

#[macro_export]
macro_rules! import_statement {
    ($( $import:expr ),*) => {
        import_statement!($( $import, )*)
    };
    ($( $import:expr, )*) => {
        $crate::model::modules::ImportStatement::from_iter([
            $(
                $crate::model::modules::Import::from($import),
            )*
        ])
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Members & Variants
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! member {
    ($name:expr => unknown) => {
        $crate::model::members::MemberDef::new_unknown($name)
    };
    ($name:expr => $target:expr) => {
        $crate::model::members::MemberDef::new(
            $name,
            $crate::model::members::TypeReference::from($target)
        )
    };
    ($name:expr => $domain:expr => $range:expr) => {
        member!($name => $crate::model::members::MappingType::new($domain, $range))
    };
    // ======================================
    ($name:expr => { $card:expr } unknown) => {
        $crate::model::members::MemberDef::new_unknown($name).with_target_cardinality($card)
    };
    ($name:expr => { $card:expr } $target:expr) => {
        $crate::model::members::MemberDef::new(
            $name,
            $crate::model::members::TypeReference::from($target)
        ).with_target_cardinality($card)
    };
    ($name:expr => { $card:expr } $domain:expr => $range:expr) => {
        member!($name => { $card } $crate::model::members::MappingType::new($domain, $range))
    };
}

#[macro_export]
macro_rules! unvar {
    ($type_name:expr) => {
        $crate::model::definitions::TypeVariant::new(
            $crate::model::identifiers::IdentifierReference::from($type_name),
        )
    };
    ($type_name:expr ; $body:expr) => {
        $crate::model::definitions::TypeVariant::new(
            $crate::model::identifiers::IdentifierReference::from($type_name),
        )
        .with_body($body)
    };
    ($type_name:expr, $rename:expr) => {
        $crate::model::definitions::TypeVariant::new(
            $crate::model::identifiers::IdentifierReference::from($type_name),
        )
        .with_rename($rename)
    };
    ($type_name:expr, $rename:expr ; $body:expr) => {
        $crate::model::definitions::TypeVariant::new(
            $crate::model::identifiers::IdentifierReference::from($type_name),
        )
        .with_rename($rename)
        .with_body($body)
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Definitions
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! property {
    ($member_def:expr) => {
        $crate::model::definitions::PropertyDef::from($member_def)
    };
    ($member_def:expr ; $body:expr) => {
        property!($member_def.with_body($body))
    };
    // ======================================
    ($name:expr => unknown) => {
        property!(member!($name => unknown))
    };
    ($name:expr => unknown ; $body:expr) => {
        property!(member!($name => unknown).with_body($body))
    };
    ($name:expr => $target:expr) => {
        property!(member!($name => $target))
    };
    ($name:expr => $target:expr ; $body:expr) => {
        property!(member!($name => $target).with_body($body))
    };
    ($name:expr => $domain:expr => $range:expr) => {
        property!(member!($name => $domain => $range))
    };
    ($name:expr => $domain:expr => $range:expr ; $body:expr) => {
        property!(member!($name => $domain => $range).with_body($body))
    };
    // ======================================
    ($name:expr => { $card:expr } unknown) => {
        property!(member!($name => { $card } unknown))
    };
    ($name:expr => { $card:expr } unknown ; $body:expr) => {
        property!(member!($name => { $card } unknown).with_body($body))
    };
    ($name:expr => { $card:expr } $target:expr) => {
        property!(member!($name => { $card } $target))
    };
    ($name:expr => { $card:expr } $target:expr ; $body:expr) => {
        property!(member!($name => { $card } $target).with_body($body))
    };
    ($name:expr => { $card:expr } $domain:expr => $range:expr) => {
        property!(member!($name => { $card } $domain => $range))
    };
    ($name:expr => { $card:expr } $domain:expr => $range:expr ; $body:expr) => {
        property!(member!($name => { $card } $domain => $range).with_body($body))
    };
}

#[macro_export]
macro_rules! structure {
    ($name:expr ; call $body_fn:expr) => {
        structure!($name ; $body_fn($crate::model::definitions::StructureBody::default()))
    };
    ($name:expr ; $body:expr) => {
        structure!($name).with_body($body)
    };
    ($name:expr) => {
        $crate::model::definitions::StructureDef::new($name)
    };
}

#[macro_export]
macro_rules! union {
    ($name:expr ; call $body_fn:expr) => {
        union!($name ; $body_fn($crate::model::definitions::UnionBody::default()))
    };
    ($name:expr ; $body:expr) => {
        union!($name).with_body($body)
    };
    ($name:expr) => {
        $crate::model::definitions::UnionDef::new($name)
    };
}

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
    let module_uri: url::Url = url::Url::parse(MODULE_URL).unwrap();

    module!(
        id!(unchecked MODULE_NAME), module_uri ; call |body: ModuleBody|
        body.with_imports([import_statement!(
            id!(unchecked dc),
            id!(unchecked owl),
            id!(unchecked rdf),
            id!(unchecked rdfs)
        )])
            .with_annotations([
                annotation!(id!(unchecked rdf:type), id!(unchecked owl:Ontology)),
                annotation!(id!(unchecked dcterms:title), "SKOS Vocabulary"),
                annotation!(
                    id!(unchecked dcterms:contributor),
                    vs!(
                        "Dave Beckett",
                        "Nikki Rogers",
                        "Participants in W3C's Semantic Web Deployment Working Group."
                    )
                ),
                annotation!(
                    id!(unchecked dcterms:creator),
                    vs!("Alistair Miles", "Sean Bechhofer")
                ),
                annotation!(
                    id!(unchecked rdfs:seeAlso),
                    Url::parse("http://www.w3.org/TR/skos-reference/").unwrap()
                ),
            ])
            .with_definitions([
                structure!(
                    id!(unchecked Collection) ; call |body: StructureBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked owl:Class)),
                        annotation!(
                            id!(unchecked owl:disjointWith),
                            vs!(id!(unchecked Concept), id!(unchecked ConceptScheme))
                        ),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Collection"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("A meaningful collection of concepts."@en)
                        ),
                        annotation!(
                            id!(unchecked scopeNote),
                            rdf_str!(
                                "Labelled collections can be used where you would like a set of concepts to be displayed under a 'node label' in the hierarchy."@en
                            )
                        ),
                    ])).into(),
                structure!(
                    id!(unchecked Concept) ; call |body: StructureBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked owl:Class)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Concept"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("An idea or notion; a unit of thought."@en)
                        ),
                    ])).into(),
                structure!(
                    id!(unchecked ConceptScheme) ; call |body: StructureBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked owl:Class)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Concept Scheme"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("A set of concepts, optionally including statements about semantic relationships between those concepts."@en)
                        ),
                        annotation!(
                            id!(unchecked example),
                            rdf_str!("Thesauri, classification schemes, subject heading lists, taxonomies, 'folksonomies', and other types of controlled vocabulary are all examples of concept schemes. Concept schemes are also embedded in glossaries and terminologies."@en)
                        ),
                        annotation!(
                            id!(unchecked scopeNote),
                            rdf_str!("A concept scheme may be defined to include concepts from different sources."@en)
                        ),
                    ])).into(),
                structure!(
                    id!(unchecked OrderedCollection) ; call |body: StructureBody|
                    body.with_annotations([
                        annotation!(id!(unchecked rdf:type), id!(unchecked owl:Class)),
                        annotation!(id!(unchecked rdfs:subClassOf), id!(unchecked Collection)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("Ordered Collection"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("An ordered collection of concepts, where both the grouping and the ordering are meaningful."@en)
                        ),
                        annotation!(
                            id!(unchecked scopeNote),
                            rdf_str!("Ordered collections can be used where you would like a set of concepts to be displayed in a specific order, and optionally under a 'node label'."@en)
                        ),
                    ])).into(),
                property!(
                    id!(unchecked altLabel) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:AnnotationProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked rdfs:label)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("alternative label"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("An alternative lexical label for a resource."@en)
                        ),
                        annotation!(
                            id!(unchecked rdfs:comment),
                            vs!(
                                rdf_str!("The range of skos:altLabel is the class of RDF plain literals."@en),
                                rdf_str!("skos:prefLabel, skos:altLabel and skos:hiddenLabel are pairwise disjoint properties."@en),
                            )
                        ),
                        annotation!(
                            id!(unchecked example),
                            rdf_str!("Acronyms, abbreviations, spelling variants, and irregular plural/singular forms may be included among the alternative labels for a concept. Mis-spelled terms are normally included as hidden labels (see skos:hiddenLabel)."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked broadMatch) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked broader)),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked mappingRelation)),
                        annotation!(id!(unchecked owl:inverseOf), id!(unchecked narrowMatch)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has broader match"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("skos:broadMatch is used to state a hierarchical mapping link between two conceptual resources in different concept schemes."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked broader) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked broaderTransitive)),
                        annotation!(id!(unchecked owl:inverseOf), id!(unchecked narrower)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has broader"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("Relates a concept to a concept that is more general in meaning."@en)
                        ),
                        annotation!(
                            id!(unchecked scopeNote),
                            rdf_str!("By convention, skos:broader is only used to assert an immediate (i.e. direct) hierarchical link between two conceptual resources."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked broaderTransitive) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                            id!(unchecked owl:TransitiveProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked semanticRelation)),
                        annotation!(id!(unchecked owl:inverseOf), id!(unchecked narrowerTransitive)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has broader transitive"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("skos:broaderTransitive is a transitive superproperty of skos:broader."@en)
                        ),
                        annotation!(
                            id!(unchecked scopeNote),
                            rdf_str!("By convention, skos:broaderTransitive is not used to make assertions. Rather, the properties can be used to draw inferences about the transitive closure of the hierarchical relation, which is useful e.g. when implementing a simple query expansion algorithm in a search application."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked changeNote) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:AnnotationProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked note)),
                        annotation!(id!(unchecked owl:inverseOf), id!(unchecked narrowerTransitive)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("change note"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("A note about a modification to a concept."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked closeMatch) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                            id!(unchecked owl:SymmetricProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked mappingRelation)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has close match"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("skos:closeMatch is used to link two concepts that are sufficiently similar that they can be used interchangeably in some information retrieval applications. In order to avoid the possibility of \"compound errors\" when combining mappings across more than two concept schemes, skos:closeMatch is not declared to be a transitive property."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked definition) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:AnnotationProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked note)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("definition"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("A statement or formal explanation of the meaning of a concept."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked editorialNote) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:AnnotationProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked note)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("editorial note"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("A note for an editor, translator or maintainer of the vocabulary."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked exactMatch) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                            id!(unchecked owl:SymmetricProperty),
                            id!(unchecked owl:TransitiveProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked closeMatch)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has exact match"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("skos:exactMatch is used to link two concepts, indicating a high degree of confidence that the concepts can be used interchangeably across a wide range of information retrieval applications. skos:exactMatch is a transitive property, and is a sub-property of skos:closeMatch."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked hasTopConcept) => { cardinality!(0..) } id!(unchecked ConceptScheme) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                        )),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Concept)),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked closeMatch)),
                        annotation!(id!(unchecked owl:inverseOf), id!(unchecked topConceptOf)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has top concept"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("Relates, by convention, a concept scheme to a concept which is topmost in the broader/narrower concept hierarchies for that scheme, providing an entry point to these hierarchies."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked hiddenLabel) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:AnnotationProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked rdfs:label)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("hidden label"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("A lexical label for a resource that should be hidden when generating visual displays of the resource, but should still be accessible to free text search operations."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked historyNote) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:AnnotationProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked note)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("history note"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("A note about the past state/use/meaning of a concept."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked inScheme) => { cardinality!(0..) } id!(unchecked ConceptScheme) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                        )),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("is in scheme"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("Relates a resource (for example a concept) to a concept scheme in which it is included."@en)
                        ),
                        annotation!(
                            id!(unchecked scopeNote),
                            rdf_str!("A concept may be a member of more than one concept scheme."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked mappingRelation) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked semanticRelation)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("is in mapping relation with"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("Relates two concepts coming, by convention, from different schemes, and that have comparable meanings."@en)
                        ),
                        annotation!(
                            id!(unchecked rdfs:comment),
                            rdf_str!("These concept mapping relations mirror semantic relations, and the data model defined below is similar (with the exception of skos:exactMatch) to the data model defined for semantic relations. A distinct vocabulary is provided for concept mapping relations, to provide a convenient way to differentiate links within a concept scheme from links between concept schemes. However, this pattern of usage is not a formal requirement of the SKOS data model, and relies on informal definitions of best practice."@en)
                        ),
                    )).into(),
                union!(
                    id!(unchecked CollectionMember) ; call |body: UnionBody|
                    body.with_variants([
                        unvar!(id!(unchecked Concept)),
                        unvar!(id!(unchecked Collection)),
                    ])).into(),
                property!(
                    id!(unchecked member) => { cardinality!(0..) } id!(unchecked CollectionMember) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                        )),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Collection)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has member"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("Relates a collection to one of its members."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked memberList) => { cardinality!(0..) } id!(unchecked rdf:List) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:FunctionalProperty),
                            id!(unchecked owl:ObjectProperty),
                        )),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked OrderedCollection)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has member list"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("Relates an ordered collection to the RDF list containing its member."@en)
                        ),
                        annotation!(
                            id!(unchecked rdfs:comment),
                            rdf_str!("For any resource, every item in the list given as the value of the skos:memberList property is also a value of the skos:member property."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked narrowMatch) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), vs!(
                            id!(unchecked mappingRelation),
                            id!(unchecked narrower)
                        )),
                        annotation!(id!(unchecked owl:inverseOf), id!(unchecked broadMatch)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has narrower match"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("skos:narrowMatch is used to state a hierarchical mapping link between two conceptual resources in different concept schemes."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked narrower) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked narrowerTransitive)),
                        annotation!(id!(unchecked owl:inverseOf), id!(unchecked broader)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has narrower"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("Relates a concept to a concept that is more specific in meaning."@en)
                        ),
                        annotation!(
                            id!(unchecked rdfs:comment),
                            rdf_str!("Narrower concepts are typically rendered as children in a concept hierarchy (tree)."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked narrowerTransitive) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                            id!(unchecked owl:TransitiveProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked semanticRelation)),
                        annotation!(id!(unchecked owl:inverseOf), id!(unchecked broaderTransitive)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has narrower transitive"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("skos:narrowerTransitive is a transitive superproperty of skos:narrower."@en)
                        ),
                        annotation!(
                            id!(unchecked scopeNote),
                            rdf_str!("By convention, skos:narrowerTransitive is not used to make assertions. Rather, the properties can be used to draw inferences about the transitive closure of the hierarchical relation, which is useful e.g. when implementing a simple query expansion algorithm in a search application."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked notation) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:DatatypeProperty),
                        )),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("notation"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("A notation, also known as classification code, is a string of characters such as \"T58.5\" or \"303.4833\" used to uniquely identify a concept within the scope of a given concept scheme."@en)
                        ),
                        annotation!(
                            id!(unchecked scopeNote),
                            rdf_str!("By convention, skos:notation is used with a typed literal in the object position of the triple."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked note) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:AnnotationProperty),
                        )),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("note"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("A general note, for any purpose."@en)
                        ),
                        annotation!(
                            id!(unchecked scopeNote),
                            rdf_str!("This property may be used directly, or as a super-property for more specific note types."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked prefLabel) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:AnnotationProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked rdfs:label)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("preferred label"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("The preferred lexical label for a resource, in a given language."@en)
                        ),
                        annotation!(
                            id!(unchecked rdfs:comment),
                            vs!(
                                rdf_str!("A resource has no more than one value of skos:prefLabel per language tag, and no more than one value of skos:prefLabel without language tag."@en),
                                rdf_str!("The range of skos:prefLabel is the class of RDF plain literals."@en),
                                rdf_str!("skos:prefLabel, skos:altLabel and skos:hiddenLabel are pairwise disjoint properties."@en),
                            )),
                    )).into(),
                property!(
                    id!(unchecked related) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                            id!(unchecked owl:SymmetricProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked semanticRelation)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has related"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("Relates a concept to a concept with which there is an associative semantic relationship."@en)
                        ),
                        annotation!(
                            id!(unchecked rdfs:comment),
                            rdf_str!("skos:related is disjoint with skos:broaderTransitive."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked relatedMatch) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                            id!(unchecked owl:SymmetricProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), vs!(
                            id!(unchecked mappingRelation),
                            id!(unchecked related),
                        )),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("has related match"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("skos:relatedMatch is used to state an associative mapping link between two conceptual resources in different concept schemes."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked scopeNote) => { cardinality!(0..) } id!(unchecked owl:Thing) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:AnnotationProperty),
                        )),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked note)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("scope note"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("A note that helps to clarify the meaning and/or the use of a concept."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked semanticRelation) => { cardinality!(0..) } id!(unchecked Concept) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                        )),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Concept)),
                        annotation!(id!(unchecked owl:inverseOf), id!(unchecked broaderTransitive)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("is in semantic relation with"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("Links a concept to a concept related by meaning."@en)
                        ),
                        annotation!(
                            id!(unchecked scopeNote),
                            rdf_str!("This property should not be used directly, but as a super-property for all properties denoting a relationship of meaning between concepts."@en)
                        ),
                    )).into(),
                property!(
                    id!(unchecked topConceptOf) => { cardinality!(0..) } id!(unchecked ConceptScheme) ;
                    annotation_body!(
                        annotation!(id!(unchecked rdf:type), vs!(
                            id!(unchecked rdf:Property),
                            id!(unchecked owl:ObjectProperty),
                            id!(unchecked owl:TransitiveProperty),
                        )),
                        annotation!(id!(unchecked rdfs:domain), id!(unchecked Concept)),
                        annotation!(id!(unchecked rdfs:subPropertyOf), id!(unchecked inScheme)),
                        annotation!(id!(unchecked owl:inverseOf), id!(unchecked hasTopConcept)),
                        annotation!(id!(unchecked rdfs:isDefinedBy), id!(unchecked skos)),
                        annotation!(id!(unchecked rdfs:label), rdf_str!("is top concept in scheme"@en)),
                        annotation!(
                            id!(unchecked definition),
                            rdf_str!("Relates a concept to the concept scheme that it is a top level concept of."@en)
                        ),
                    )).into(),
            ])
    )
}
