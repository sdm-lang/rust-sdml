/*!
Macros used to build standard library modules.
 */

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ URLs
// ------------------------------------------------------------------------------------------------

macro_rules! library_module_url {
    ($authority:expr, $path:expr) => {
        pub const MODULE_URL: &str =
            concat!("https://sdml.io/stdlib/", $authority, "/", $path, "#");
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Identifiers
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

#[macro_export]
macro_rules! idref {
    (unchecked $id1:ident : $id2:ident) => {
        $crate::model::identifiers::QualifiedIdentifier::new_unchecked(
            stringify!($id1),
            stringify!($id2),
        )
        .into()
    };
    (unchecked $id1:expr, $id2:expr) => {
        $crate::model::identifiers::QualifiedIdentifier::new_unchecked($id1, $id2).into()
    };
    (unchecked $id:ident) => {
        $crate::model::identifiers::Identifier::new_unchecked(stringify!($id)).into()
    };
    (unchecked $id:expr) => {
        $crate::model::identifiers::Identifier::new_unchecked($id).into()
    };
    ($id1:ident, $id2:ident) => {
        $crate::model::identifiers::QualifiedIdentifier::from_str(&format!("{}:{}", $id1, $id2))
            .unwrap()
            .into()
    };
    ($id1:expr, $id2:expr) => {
        $crate::model::identifiers::QualifiedIdentifier::from_str(&format!(
            "{}:{}",
            stringify!($id1),
            stringify!($id2)
        ))
        .unwrap()
        .into()
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
    ($name:ident @ $lang:ident) => {
        $crate::model::values::LanguageString::new(stringify!($name), Some(lang!($lang)))
    };
    ($text:literal @ $lang:ident) => {
        $crate::model::values::LanguageString::new($text, Some(lang!($lang)))
    };
    ($text:literal) => {
        $crate::model::values::LanguageString::new($text, None)
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
            $crate::model::identifiers::IdentifierReference::from($name),
            $crate::model::values::SimpleValue::from($value),
        )
    };
    ($name:expr) => {
        $crate::model::identifiers::IdentifierReference::from($name)
    };
}

#[macro_export]
macro_rules! url {
    ($value:literal) => {
        url::Url::parse($value).unwrap()
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
        let module = $crate::model::modules::Module::new($name).with_base_uri($base_uri);
        let module = $body_fn(module);
        module
    }};
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
// Public Macros ❱ Definitions ❱ Datatype
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! datatype {
    ($name:expr, $base:expr ; call $body_fn:expr) => {
        datatype!($name, $base ; $body_fn($crate::model::annotations::AnnotationOnlyBody::default()))
    };
    ($name:expr, $base:expr ; $body:expr) => {
        datatype!($name, $base).with_body($body)
    };
    ($name:expr, $base:expr) => {
        $crate::model::definitions::DatatypeDef::new($name, $base)
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Definitions ❱ Property
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

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Definitionss ❱ RDF
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! rdf {
    ($name:expr ; class $( $super:expr ),* ; call $body_fn:expr) => {{
        let mut body = $crate::model::annotations::AnnotationOnlyBody::default();
        body.extend_annotations([
            annotation!(id!(unchecked rdf:type), id!(unchecked rdfs:Resource)),
            annotation!(id!(unchecked rdf:type), id!(unchecked rdfs:Class)),
            $(
                annotation!(id!(unchecked rdfs:subClassOf), $super),
            )*
        ]);
        rdf!($name ; $body_fn(body))
    }};
    ($name:expr ; property $( $super:expr ),* ; call $body_fn:expr) => {{
        let mut body = $crate::model::annotations::AnnotationOnlyBody::default();
        body.extend_annotations([
            annotation!(id!(unchecked rdf:type), id!(unchecked rdf:Property)),
            $(
                annotation!(id!(unchecked rdfs:subPropertyOf), $super),
            )*
        ]);
        rdf!($name ; $body_fn(body))
    }};
    ($name:expr ; individual $( $type:expr ),* ; call $body_fn:expr) => {{
        let mut body = $crate::model::annotations::AnnotationOnlyBody::default();
        body.extend_annotations([
            annotation!(id!(unchecked rdf:type), id!(unchecked owl:NamedIndividual)),
            $(
                annotation!(id!(unchecked rdf:type), $type),
            )*
        ]);
        rdf!($name ; $body_fn(body))
    }};
    ($name:expr ; unnamed individual $( $type:expr ),* ; call $body_fn:expr) => {{
        let mut body = $crate::model::annotations::AnnotationOnlyBody::default();
        body.extend_annotations([
            $(
                annotation!(id!(unchecked rdf:type), $type),
            )*
        ]);
        rdf!($name ; $body_fn(body))
    }};
    ($name:expr ; datatype ; call $body_fn:expr) => {{
        let mut body = $crate::model::annotations::AnnotationOnlyBody::default();
        body.extend_annotations([
            annotation!(id!(unchecked rdf:type), id!(unchecked rdfs:Resource)),
            annotation!(id!(unchecked rdf:type), id!(unchecked rdfs:Datatype)),
        ]);
        rdf!($name ; $body_fn(body))
    }};
    ($name:expr ; call $body_fn:expr) => {
        rdf!($name ; $body_fn($crate::model::annotations::AnnotationOnlyBody::default()))
    };
    ($name:expr ; $body:expr) => {
        rdf!($name).with_body($body)
    };
    ($name:expr) => {
        $crate::model::definitions::RdfDef::new($name)
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Definitions ❱ Structure
// ------------------------------------------------------------------------------------------------

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

// ------------------------------------------------------------------------------------------------
// Public Macros ❱ Definitions ❱ Union
// ------------------------------------------------------------------------------------------------

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
// Public Macros ❱ Modules
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! module_function {
    ($init_fn:expr) => {
        static __MODULE: ::std::sync::OnceLock<$crate::model::modules::Module> =
            ::std::sync::OnceLock::new();

        pub fn module() -> &'static Module {
            __MODULE.get_or_init($init_fn)
        }
    };
}
