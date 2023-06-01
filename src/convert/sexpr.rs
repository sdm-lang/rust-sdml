/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::Error;
use crate::model::{
    Annotation, AnnotationOnlyBody, ByReferenceMember, ByValueMember, Cardinality, DatatypeDef,
    EntityBody, EntityDef, EntityGroup, EntityMember, EnumBody, EnumDef, EnumVariant, EventDef,
    Identifier, IdentifierReference, IdentityMember, Import, ImportStatement, LanguageTag,
    ListMember, ListOfValues, Module, ModuleBody, QualifiedIdentifier, SimpleValue, Span,
    StructureBody, StructureDef, StructureGroup, TypeDefinition, TypeReference, Value,
    ValueConstructor,
};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_as_sexpr<W: Write>(module: &Module, w: &mut W) -> Result<(), Error> {
    write_module(module, "", w)
}

write_to_string!(to_sexpr_string, write_as_sexpr);

write_to_file!(to_sexpr_file, write_as_sexpr);

print_to_stdout!(print_sexpr, write_as_sexpr);

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

const NEW_LINE: &[u8] = "\n".as_bytes();
const CLOSE_PAREN: &[u8] = ")".as_bytes();
const CLOSE_PAREN_NEW_LINE: &[u8] = ")\n".as_bytes();

fn write_identifier<W: Write>(me: &Identifier, _: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("(identifier '{})", me.as_ref()).as_bytes())?;
    Ok(())
}

fn write_qualified_identifier<W: Write>(
    me: &QualifiedIdentifier,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!("{}(qualified_identifier\n", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("{}module: ", prefix).as_bytes())?;
    write_identifier(me.module(), &prefix, w)?;
    w.write_all(NEW_LINE)?;

    w.write_all(format!("{}member: ", prefix).as_bytes())?;
    write_identifier(me.module(), &prefix, w)?;
    w.write_all(NEW_LINE)?;

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_identifier_reference<W: Write>(
    me: &IdentifierReference,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!("{}(identifier_reference\n", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    match me {
        IdentifierReference::Identifier(v) => write_identifier(v, &prefix, w)?,
        IdentifierReference::QualifiedIdentifier(v) => write_qualified_identifier(v, &prefix, w)?,
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_module<W: Write>(me: &Module, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(module", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
    write_identifier(me.name(), &prefix, w)?;
    w.write_all(NEW_LINE)?;

    write_module_body(me.body(), &prefix, w)?;

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_module_body<W: Write>(me: &ModuleBody, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(module_body\n", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    for import in me.imports() {
        write_import_statement(import, &prefix, w)?;
    }
    for annotation in me.annotations() {
        write_annotation(annotation, &prefix, w)?;
    }
    for definition in me.definitions() {
        write_type_definition(definition, &prefix, w)?;
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_import_statement<W: Write>(
    me: &ImportStatement,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!("{}(import\n", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    for import in me.imports() {
        write_import(import, &prefix, w)?;
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_import<W: Write>(me: &Import, prefix: &str, w: &mut W) -> Result<(), Error> {
    match me {
        Import::Module(v) => {
            w.write_all(format!("{}(module_import", prefix).as_bytes())?;
            let prefix = format!("{}  ", prefix);

            w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
            write_identifier(v, &prefix, w)?;
        }
        Import::Member(v) => {
            w.write_all(format!("{}(member_import", prefix).as_bytes())?;
            let prefix = format!("{}  ", prefix);

            w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
            write_qualified_identifier(v, &prefix, w)?;
        }
    };

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_annotation<W: Write>(me: &Annotation, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(annotation", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
    write_identifier_reference(me.name(), &prefix, w)?;

    w.write_all(format!("\n{}value: ", prefix).as_bytes())?;
    write_value(me.value(), &prefix, w)?;

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_value<W: Write>(me: &Value, prefix: &str, w: &mut W) -> Result<(), Error> {
    match me {
        Value::Simple(v) => write_simple_value(v, prefix, w)?,
        Value::ValueConstructor(v) => write_value_constructor(v, prefix, w)?,
        Value::Reference(v) => write_identifier_reference(v, prefix, w)?,
        Value::List(vs) => write_list_of_values(vs, prefix, w)?,
    }

    Ok(())
}

fn write_simple_value<W: Write>(me: &SimpleValue, prefix: &str, w: &mut W) -> Result<(), Error> {
    match me {
        SimpleValue::String(v) => {
            w.write_all(format!("{}(string", prefix).as_bytes())?;
            let prefix = format!("{}  ", prefix);

            w.write_all(format!("\n{}(quoted_string {:?})", prefix, v.value()).as_bytes())?;

            if let Some(language) = v.language() {
                w.write_all(format!("\n{}language: ", prefix).as_bytes())?;
                write_language_tag(language, &prefix, w)?;
            }
            w.write_all(CLOSE_PAREN)?
        }
        SimpleValue::Double(v) => w.write_all(format!("{}(double {})", prefix, v).as_bytes())?,
        SimpleValue::Decimal(v) => w.write_all(format!("{}(decimal {})", prefix, v).as_bytes())?,
        SimpleValue::Integer(v) => w.write_all(format!("{}(integer {})", prefix, v).as_bytes())?,
        SimpleValue::Boolean(v) => w.write_all(format!("{}(boolean {})", prefix, v).as_bytes())?,
        SimpleValue::IriReference(v) => {
            w.write_all(format!("{}(iri_reference <{}>)", prefix, v).as_bytes())?
        }
    }

    Ok(())
}

fn write_list_of_values<W: Write>(me: &ListOfValues, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(list_of_values", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    for value in me.values() {
        w.write_all(NEW_LINE)?;
        write_list_member(value, &prefix, w)?;
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_language_tag<W: Write>(me: &LanguageTag, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(language_tag {})", prefix, me.as_ref()).as_bytes())?;
    Ok(())
}

fn write_list_member<W: Write>(me: &ListMember, prefix: &str, w: &mut W) -> Result<(), Error> {
    match me {
        ListMember::Simple(v) => write_simple_value(v, prefix, w)?,
        ListMember::ValueConstructor(v) => write_value_constructor(v, prefix, w)?,
        ListMember::Reference(v) => write_identifier_reference(v, prefix, w)?,
    }

    Ok(())
}

fn write_value_constructor<W: Write>(
    me: &ValueConstructor,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!("{}(value_constructor", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
    write_identifier_reference(me.type_name(), &prefix, w)?;

    w.write_all(format!("\n{}value: ", prefix).as_bytes())?;
    write_simple_value(me.value(), &prefix, w)?;

    w.write_all(CLOSE_PAREN_NEW_LINE)?;
    Ok(())
}

fn write_type_definition<W: Write>(
    me: &TypeDefinition,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    match me {
        TypeDefinition::Datatype(v) => write_data_type_def(v, prefix, w)?,
        TypeDefinition::Entity(v) => write_entity_def(v, prefix, w)?,
        TypeDefinition::Enum(v) => write_enum_def(v, prefix, w)?,
        TypeDefinition::Event(v) => write_event_def(v, prefix, w)?,
        TypeDefinition::Structure(v) => write_structure_def(v, prefix, w)?,
    }

    Ok(())
}

fn write_data_type_def<W: Write>(me: &DatatypeDef, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(data_type_def", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
    write_identifier(me.name(), &prefix, w)?;

    w.write_all(format!("\n{}base: ", prefix).as_bytes())?;
    write_identifier_reference(me.base_type(), &prefix, w)?;

    if let Some(body) = &me.body() {
        write_annotation_only_body(body, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_annotation_only_body<W: Write>(
    me: &AnnotationOnlyBody,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!("{}(annotation_only_body", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    for annotation in me.annotations() {
        w.write_all(NEW_LINE)?;
        write_annotation(annotation, &prefix, w)?;
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_entity_def<W: Write>(me: &EntityDef, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(entity_def", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
    write_identifier(me.name(), &prefix, w)?;

    if let Some(body) = me.body() {
        write_entity_body(body, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_entity_body<W: Write>(me: &EntityBody, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(entity_body", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}identity:\n", prefix).as_bytes())?;
    w.write_all(prefix.as_bytes())?;
    write_identity_member(me.identity(), &prefix, w)?;

    for annotation in me.annotations() {
        write_annotation(annotation, &prefix, w)?;
    }

    for member in me.members() {
        write_entity_member(member, &prefix, w)?;
    }

    for group in me.groups() {
        write_entity_group(group, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_entity_member<W: Write>(me: &EntityMember, prefix: &str, w: &mut W) -> Result<(), Error> {
    match me {
        EntityMember::ByValue(v) => write_by_value_member(v, prefix, w)?,
        EntityMember::ByReference(v) => write_by_reference_member(v, prefix, w)?,
    }

    Ok(())
}

fn write_entity_group<W: Write>(me: &EntityGroup, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(entity_group", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    for annotation in me.annotations() {
        write_annotation(annotation, &prefix, w)?;
    }

    for member in me.members() {
        write_entity_member(member, &prefix, w)?;
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_enum_def<W: Write>(me: &EnumDef, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(enum_def", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
    write_identifier(me.name(), &prefix, w)?;

    if let Some(body) = &me.body() {
        write_enum_body(body, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_enum_body<W: Write>(me: &EnumBody, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(enum_body", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    for annotation in me.annotations() {
        write_annotation(annotation, &prefix, w)?;
    }

    for variant in me.variants() {
        write_enum_variant(variant, &prefix, w)?;
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_enum_variant<W: Write>(me: &EnumVariant, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(enum_variant", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
    write_identifier(me.name(), &prefix, w)?;

    w.write_all(format!("\n{}value: {}", prefix, me.value()).as_bytes())?;

    if let Some(body) = &me.body() {
        write_annotation_only_body(body, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_event_def<W: Write>(me: &EventDef, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(event_def", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
    write_identifier(me.name(), &prefix, w)?;

    w.write_all(format!("\n{}source: ", prefix).as_bytes())?;
    write_identifier_reference(me.event_source(), &prefix, w)?;

    if let Some(body) = me.body() {
        write_structure_body(body, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_structure_def<W: Write>(me: &StructureDef, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(structure_def", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
    write_identifier(me.name(), &prefix, w)?;

    if let Some(body) = me.body() {
        write_structure_body(body, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_structure_body<W: Write>(
    me: &StructureBody,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!("{}(structure_body", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    for annotation in me.annotations() {
        write_annotation(annotation, &prefix, w)?;
    }

    for member in me.members() {
        write_by_value_member(member, &prefix, w)?;
    }

    for group in me.groups() {
        write_structure_group(group, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_structure_group<W: Write>(
    me: &StructureGroup,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!("{}(structure_group", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    for annotation in me.annotations() {
        write_annotation(annotation, &prefix, w)?;
    }

    for member in me.members() {
        write_by_value_member(member, &prefix, w)?;
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_identity_member<W: Write>(
    me: &IdentityMember,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!("{}(identity_member", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name: ", prefix).as_bytes())?;
    write_identifier(me.name(), &prefix, w)?;

    w.write_all(format!("\n{}target: ", prefix).as_bytes())?;
    write_type_reference(me.target_type(), &prefix, w)?;

    if let Some(body) = &me.body() {
        write_annotation_only_body(body, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_by_value_member<W: Write>(
    me: &ByValueMember,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!("{}(member_by_value", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name:", prefix).as_bytes())?;
    write_identifier(me.name(), &prefix, w)?;

    w.write_all(format!("\n{}target:", prefix).as_bytes())?;
    write_type_reference(me.target_type(), &prefix, w)?;

    if let Some(card) = &me.target_cardinality() {
        w.write_all(format!("\n{}targetCardinality:", prefix).as_bytes())?;
        write_cardinality(card, &prefix, w)?;
    }

    if let Some(body) = &me.body() {
        write_annotation_only_body(body, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_by_reference_member<W: Write>(
    me: &ByReferenceMember,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    w.write_all(format!("{}(member_by_reference", prefix).as_bytes())?;
    let prefix = format!("{}  ", prefix);

    w.write_all(format!("\n{}name:", prefix).as_bytes())?;
    write_identifier(me.name(), &prefix, w)?;

    w.write_all(format!("\n{}target:", prefix).as_bytes())?;
    write_type_reference(me.target_type(), &prefix, w)?;

    if let Some(card) = &me.source_cardinality() {
        w.write_all(format!("\n{}sourceCardinality:", prefix).as_bytes())?;
        write_cardinality(card, &prefix, w)?;
    }

    if let Some(card) = &me.target_cardinality() {
        w.write_all(format!("\n{}targetCardinality:", prefix).as_bytes())?;
        write_cardinality(card, &prefix, w)?;
    }

    if let Some(body) = &me.body() {
        write_annotation_only_body(body, &prefix, w)?
    }

    w.write_all(CLOSE_PAREN_NEW_LINE)?;

    Ok(())
}

fn write_type_reference<W: Write>(
    me: &TypeReference,
    prefix: &str,
    w: &mut W,
) -> Result<(), Error> {
    if let TypeReference::Reference(reference) = me {
        write_identifier_reference(reference, &prefix, w)?;
    } else {
        w.write_all("(unknown_type)".as_bytes())?;
    }

    Ok(())
}

fn write_cardinality<W: Write>(me: &Cardinality, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(cardinality_expression min: {}", prefix, me.min_occurs()).as_bytes())?;

    if let Some(max) = me.max_occurs() {
        w.write_all(format!("max: {})", max).as_bytes())?;
    } else {
        w.write_all(CLOSE_PAREN_NEW_LINE)?;
    }

    Ok(())
}

#[allow(dead_code)]
fn write_span<W: Write>(me: &Span, prefix: &str, w: &mut W) -> Result<(), Error> {
    w.write_all(format!("{}(span start: {} end: {})", prefix, me.start(), me.end()).as_bytes())?;

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
