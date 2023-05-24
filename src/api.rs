/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::Error;
use std::{borrow::Cow, fmt::Display, fs::File, path::Path, str::FromStr};
use tree_sitter::{Point, Tree};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

pub trait NodeWrapper<'a> {
    fn text(&self) -> Result<&'a str, Error>;

    fn start_byte(&self) -> usize;

    fn end_byte(&self) -> usize;

    fn start_position(&self) -> Point;

    fn end_position(&self) -> Point;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Parse Tree
// ------------------------------------------------------------------------------------------------

tree_wrapper!();
tree_wrapper_impl!(module, Module<'_>);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Identifiers
// ------------------------------------------------------------------------------------------------

node_wrapper!(Identifier);
node_wrapper_impl!(Identifier);
node_as_str_impl!(Identifier);

// ------------------------------------------------------------------------------------------------

node_wrapper!(QualifiedIdentifier);
node_wrapper_impl!(QualifiedIdentifier);
node_as_str_impl!(QualifiedIdentifier);

impl<'a> QualifiedIdentifier<'a> {
    node_wrapper_field_single!(module, "module", Identifier<'a>);
    node_wrapper_field_single!(member, "member", Identifier<'a>);
}

// ------------------------------------------------------------------------------------------------

choice_wrapper!(IdentifierReference, Identifier, QualifiedIdentifier);
choice_wrapper_impl!(
    "identifier_reference" => IdentifierReference,
    "identifier" => Identifier,
    "qualified_identifier" => QualifiedIdentifier);
node_as_str_impl!(IdentifierReference);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Modules
// ------------------------------------------------------------------------------------------------

node_wrapper!(Module);
node_wrapper_impl!(Module);

impl<'a> Module<'a> {
    node_wrapper_field_single!(name, "name", Identifier<'a>);
    node_wrapper_field_single!(body, "body", ModuleBody<'a>);
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(ModuleBody);
node_wrapper_impl!(ModuleBody);
node_has_annotations_impl!(ModuleBody);

impl<'a> ModuleBody<'a> {
    node_wrapper_child_list!(imports, "import", ImportStatement<'a>);
    node_wrapper_child_list!(
        definitions,
        ["data_type_def", "entity_def", "enum_def", "event_def", "structure_def"],
        TypeDefinition<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Imports
// ------------------------------------------------------------------------------------------------

node_wrapper!(ImportStatement);
node_wrapper_impl!(ImportStatement);

impl<'a> ImportStatement<'a> {
   node_wrapper_child_list!(imported, Import<'a>);
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(ModuleImport);
node_wrapper_impl!(ModuleImport);
node_as_str_impl!(ModuleImport);

// ------------------------------------------------------------------------------------------------

node_wrapper!(MemberImport);
node_wrapper_impl!(MemberImport);
node_as_str_impl!(MemberImport);

// ------------------------------------------------------------------------------------------------

choice_wrapper!(Import, ModuleImport, MemberImport);
choice_wrapper_impl!(
    Import,
    "module_import" => ModuleImport,
    "member_import" => MemberImport);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Annotations
// ------------------------------------------------------------------------------------------------

node_wrapper!(Annotation);
node_wrapper_impl!(Annotation);

impl<'a> Annotation<'a> {
    node_wrapper_field_single!(name, "name", IdentifierReference<'a>);
    node_wrapper_field_single!(value, "value", Value<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Annotations ❱ Values ❱ Strings
// ------------------------------------------------------------------------------------------------

node_wrapper!(QuotedString);
node_wrapper_impl!(QuotedString);
node_as_str_impl!(QuotedString);

impl QuotedString<'_> {
    pub fn value(&self) -> &str {
        let text = self.text().unwrap();
        &text[1..(text.len() - 1)]
    }
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(LanguageTag);
node_wrapper_impl!(LanguageTag);

impl LanguageTag<'_> {
    pub fn value(&self) -> &str {
        let text = self.text().unwrap();
        &text[1..]
    }
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(String);
node_wrapper_impl!(String);

impl<'a> String<'a> {
    node_wrapper_child_single!(string, "quoted_string", QuotedString<'a>);
    node_wrapper_field_single_opt!(language, "language", LanguageTag<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Annotations ❱ Values ❱ Numbers
// ------------------------------------------------------------------------------------------------

node_wrapper!(Double);
node_wrapper_impl!(Double);

impl Double<'_> {
    pub fn value(&self) -> f64 {
        f64::from_str(&self.text().unwrap()).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(Decimal);
node_wrapper_impl!(Decimal);

impl Decimal<'_> {
    pub fn value(&self) -> rust_decimal::Decimal {
        rust_decimal::Decimal::from_str(&self.text().unwrap()).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(Integer);
node_wrapper_impl!(Integer);

impl Integer<'_> {
    pub fn value(&self) -> i64 {
        i64::from_str(&self.text().unwrap()).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Annotations ❱ Values ❱ Booleans
// ------------------------------------------------------------------------------------------------

node_wrapper!(Boolean);
node_wrapper_impl!(Boolean);

impl Boolean<'_> {
    pub fn value(&self) -> bool {
        bool::from_str(&self.text().unwrap()).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Annotations ❱ Values ❱ IRI/URI/URL
// ------------------------------------------------------------------------------------------------

node_wrapper!(IriReference);
node_wrapper_impl!(IriReference);

impl IriReference<'_> {
    pub fn value(&self) -> Url {
        let text = self.text().unwrap();
        Url::from_str(&text[1..(text.len() - 1)]).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Annotations ❱ Values ❱ Lists
// ------------------------------------------------------------------------------------------------

node_wrapper!(ListOfValues);
node_wrapper_impl!(ListOfValues);

impl<'a> ListOfValues<'a> {
    node_wrapper_child_list!(values, SimpleValue<'a>);
}

// ------------------------------------------------------------------------------------------------

choice_wrapper!(
    SimpleValue,
    String,
    Double,
    Decimal,
    Integer,
    Boolean,
    IriReference
);
choice_wrapper_impl!(
    SimpleValue,
    "string" => String,
    "double" => Double,
    "decimal" => Decimal,
    "integer" => Integer,
    "boolean" => Boolean,
    "iri_reference" => IriReference);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Annotations ❱ Values ❱ Value Constructor
// ------------------------------------------------------------------------------------------------

node_wrapper!(ValueConstructor);
node_wrapper_impl!(ValueConstructor);

impl<'a> ValueConstructor<'a> {
    node_wrapper_field_single!(name, "name", IdentifierReference<'a>);
    node_wrapper_field_single!(value, "value", SimpleValue<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Annotations ❱ Values
// ------------------------------------------------------------------------------------------------

choice_wrapper!(
    Value,
    String,
    Double,
    Decimal,
    Integer,
    Boolean,
    IriReference,
    ListOfValues,
    ValueConstructor
);
choice_wrapper_impl!(
    "value" => Value,
    "string" => String,
    "double" => Double,
    "decimal" => Decimal,
    "integer" => Integer,
    "boolean" => Boolean,
    "iri_reference" => IriReference,
    "list_of_values" => ListOfValues,
    "value_constructor" => ValueConstructor);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Types ❱ Data Types
// ------------------------------------------------------------------------------------------------

node_wrapper!(Datatype);
node_wrapper_impl!(Datatype);

impl<'a> Datatype<'a> {
    node_wrapper_field_single!(name, "name", Identifier<'a>);
    node_wrapper_field_single!(base_type, "base", IdentifierReference<'a>);
    node_wrapper_field_single_opt!(body, "annotation_only_body", AnnotationOnlyBody<'a>);
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(AnnotationOnlyBody);
node_wrapper_impl!(AnnotationOnlyBody);
node_has_annotations_impl!(AnnotationOnlyBody);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Types ❱ Entities
// ------------------------------------------------------------------------------------------------

node_wrapper!(Entity);
node_wrapper_impl!(Entity);

impl<'a> Entity<'a> {
    node_wrapper_field_single!(name, "name", Identifier<'a>);
    node_wrapper_field_single_opt!(body, "body", EntityBody<'a>);
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(EntityBody);
node_wrapper_impl!(EntityBody);
node_has_annotations_impl!(EntityBody);

impl<'a> EntityBody<'a> {
    node_wrapper_field_single!(identity, "identity", IdentityMember<'a>);
    node_wrapper_child_list!(groups, "entity_group", EntityGroup<'a>);
    node_wrapper_child_list!(value_members, "member_by_value", MemberByValue<'a>);
    node_wrapper_child_list!(ref_members, "member_by_reference", MemberByReference<'a>);
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(EntityGroup);
node_wrapper_impl!(EntityGroup);
node_has_annotations_impl!(EntityGroup);

impl<'a> EntityGroup<'a> {
    node_wrapper_child_list!(value_members, "member_by_value", MemberByValue<'a>);
    node_wrapper_child_list!(ref_members, "member_by_reference", MemberByReference<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Types ❱ Enumerations
// ------------------------------------------------------------------------------------------------

node_wrapper!(Enum);
node_wrapper_impl!(Enum);

impl<'a> Enum<'a> {
    node_wrapper_field_single!(name, "name", Identifier<'a>);
    node_wrapper_field_single_opt!(body, "body", EnumBody<'a>);
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(EnumBody);
node_wrapper_impl!(EnumBody);
node_has_annotations_impl!(EnumBody);

impl<'a> EnumBody<'a> {
    node_wrapper_child_list!(variants, "enum_variant", EnumVariant<'a>);
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(EnumVariant);
node_wrapper_impl!(EnumVariant);

impl<'a> EnumVariant<'a> {
    node_wrapper_field_single!(name, "name", Identifier<'a>);
    node_wrapper_field_single_from_str!(value, "value", u64);
    node_wrapper_field_single_opt!(body, "annotation_only_body", AnnotationOnlyBody<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Types ❱ Events
// ------------------------------------------------------------------------------------------------

node_wrapper!(Event);
node_wrapper_impl!(Event);

impl<'a> Event<'a> {
    node_wrapper_field_single!(name, "name", Identifier<'a>);
    node_wrapper_field_single!(event_source, "source", IdentifierReference<'a>);
    node_wrapper_field_single_opt!(body, "body", StructureBody<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Types ❱ Structures
// ------------------------------------------------------------------------------------------------

node_wrapper!(Structure);
node_wrapper_impl!(Structure);

impl<'a> Structure<'a> {
    node_wrapper_field_single!(name, "name", Identifier<'a>);
    node_wrapper_field_single_opt!(body, "body", StructureBody<'a>);
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(StructureBody);
node_wrapper_impl!(StructureBody);
node_has_annotations_impl!(StructureBody);

impl<'a> StructureBody<'a> {
    node_wrapper_child_list!(groups, "entity_group", StructureGroup<'a>);
    node_wrapper_child_list!(members, "member_by_value", MemberByValue<'a>);
}

// ------------------------------------------------------------------------------------------------

node_wrapper!(StructureGroup);
node_wrapper_impl!(StructureGroup);
node_has_annotations_impl!(StructureGroup);

impl<'a> StructureGroup<'a> {
    node_wrapper_child_list!(members, "member_by_value", MemberByValue<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Types
// ------------------------------------------------------------------------------------------------

choice_wrapper!(TypeDefinition, Datatype, Entity, Enum, Event, Structure);
choice_wrapper_impl!(
    TypeDefinition,
    "data_type_def" => Datatype,
    "entity_def" => Entity,
    "enum_def" => Enum,
    "event_def" => Event,
    "structure_def" => Structure);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Identity
// ------------------------------------------------------------------------------------------------

node_wrapper!(IdentityMember);
node_wrapper_impl!(IdentityMember);

impl<'a> IdentityMember<'a> {
    node_wrapper_field_single!(name, "name", Identifier<'a>);
    node_wrapper_field_single!(target_type, "target", MemberTypeTarget<'a>);
    node_wrapper_field_single_opt!(body, "annotation_only_body", AnnotationOnlyBody<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ By Value
// ------------------------------------------------------------------------------------------------

node_wrapper!(MemberByValue);
node_wrapper_impl!(MemberByValue);

impl<'a> MemberByValue<'a> {
    node_wrapper_field_single!(name, "name", Identifier<'a>);
    node_wrapper_field_single!(target_type, "target", MemberTypeTarget<'a>);
    node_wrapper_field_single_opt!(target_cardinality, "targetCardinality", MemberCardinality<'a>);
    node_wrapper_field_single_opt!(body, "annotation_only_body", AnnotationOnlyBody<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ By Reference
// ------------------------------------------------------------------------------------------------

node_wrapper!(MemberByReference);
node_wrapper_impl!(MemberByReference);

impl<'a> MemberByReference<'a> {
    node_wrapper_field_single!(name, "name", Identifier<'a>);
    node_wrapper_field_single_opt!(source_cardinality, "sourceCardinality", MemberCardinality<'a>);
    node_wrapper_field_single!(target_type, "target", MemberTypeTarget<'a>);
    node_wrapper_field_single_opt!(target_cardinality, "targetCardinality", MemberCardinality<'a>);
    node_wrapper_field_single_opt!(body, "annotation_only_body", AnnotationOnlyBody<'a>);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Type Target
// ------------------------------------------------------------------------------------------------

choice_wrapper!(MemberTypeTarget, UnknownType, IdentifierReference);
choice_wrapper_impl!(
    MemberTypeTarget,
    "unknown_type" => UnknownType,
    "identifier_reference" => IdentifierReference);

node_wrapper!(UnknownType);
node_wrapper_impl!(UnknownType);

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Members ❱ Cardinality
// ------------------------------------------------------------------------------------------------

node_wrapper!(MemberCardinality);
node_wrapper_impl!(MemberCardinality);

impl<'a> MemberCardinality<'a> {
    node_wrapper_field_single_from_str!(min_occurs, "min", u32);
    node_wrapper_child_single_opt!(range, "cardinality_range", CardinalityRange<'a>);
}

node_wrapper!(CardinalityRange);
node_wrapper_impl!(CardinalityRange);

impl CardinalityRange<'_> {
    node_wrapper_field_single_from_str_opt!(max_occurs, "max", u32);
}
