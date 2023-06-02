/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_value_for_type, unexpected_node_kind, Error};
use crate::model::{
    Annotation, AnnotationOnlyBody, DatatypeDef, EntityBody, EntityDef, EnumBody, EnumDef,
    EventDef, Identifier, IdentifierReference, IdentityMember, ImportStatement, LanguageString,
    LanguageTag, ListOfValues, Module, ModuleBody, QualifiedIdentifier, SimpleValue, StructureBody,
    StructureDef, StructureGroup, TypeReference, Value, ValueConstructor,
};
use rust_decimal::Decimal;
use std::borrow::Cow;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use tracing::trace;
use tree_sitter::Parser;
use tree_sitter::{Node, TreeCursor};
use tree_sitter_sdml::language;
use url::Url;

use super::{ByReferenceMember, ByValueMember, Cardinality, EntityGroup, EnumVariant};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn parse_file<P>(path: P) -> Result<Module, Error>
where
    P: AsRef<Path>,
{
    let source = read_to_string(path)?;
    parse_str_inner(&Cow::Owned(source))
}

pub fn parse_str(source: &str) -> Result<Module, Error> {
    parse_str_inner(&Cow::Borrowed(source))
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

fn parse_str_inner(source: &str) -> Result<Module, Error> {
    let mut parser = Parser::new();
    parser
        .set_language(language())
        .expect("Error loading SDML grammar");
    let tree = parser.parse(source, None).unwrap();

    let node = tree.root_node();
    if node.kind() == "module" {
        let mut cursor = tree.walk();
        parse_module(source, &mut cursor)
    } else {
        Err(unexpected_node_kind("module", node.kind()))
    }
}

fn parse_module<'a>(source: &'a str, cursor: &mut TreeCursor<'a>) -> Result<Module, Error> {
    let node = cursor.node();
    trace!("parse_module: {:?}", node);
    check_if_error(source, &node)?;

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());

    let child = node.child_by_field_name("body").unwrap();
    check_if_error(source, &child)?;
    cursor.reset(child);
    let body = parse_module_body(source, cursor)?;

    Ok(Module::new(name, body).with_ts_span(node.into()))
}

fn parse_module_body<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<ModuleBody, Error> {
    trace!("parse_module_body: {:?}", cursor.node());
    let mut body = ModuleBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "import" => body.add_import(parse_import(source, cursor)?),
                    "annotation" => body.add_annotation(parse_annotation(source, cursor)?),
                    "data_type_def" => {
                        body.add_definition(parse_data_type_def(source, cursor)?.into())
                    }
                    "entity_def" => body.add_definition(parse_entity_def(source, cursor)?.into()),
                    "enum_def" => body.add_definition(parse_enum_def(source, cursor)?.into()),
                    "event_def" => body.add_definition(parse_event_def(source, cursor)?.into()),
                    "structure_def" => {
                        body.add_definition(parse_structure_def(source, cursor)?.into())
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    "identifier" => {
                        trace!("ignoring name: identifier");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            "import|data_type_def|entity_def|enum_def|event_def|structure_def",
                            node.kind(),
                        ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_import<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<ImportStatement, Error> {
    trace!("parse_import: {:?}", cursor.node());
    let mut import = ImportStatement::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            trace!(
                "parse_import (child): {:?} named: {}",
                node,
                node.is_named()
            );
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "module_import" => {
                        let node = node.child_by_field_name("name").unwrap();
                        check_if_error(source, &node)?;
                        let name = Identifier::new_unchecked(node_as_str(&node, source)?)
                            .with_ts_span(node.into());
                        import.add_import(name.into());
                    }
                    "member_import" => {
                        let node = node.child_by_field_name("name").unwrap();
                        check_if_error(source, &node)?;
                        let name = parse_qualified_identifier(source, &mut node.walk())?;
                        import.add_import(name.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            "module_import|member_import",
                            node.kind(),
                        ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(import)
}

fn parse_qualified_identifier<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<QualifiedIdentifier, Error> {
    let node = cursor.node();
    trace!("parse_qualified_identifier: {:?}", node);

    let child = node.child_by_field_name("module").unwrap();
    check_if_error(source, &child)?;
    let module = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());

    let child = node.child_by_field_name("member").unwrap();
    check_if_error(source, &child)?;
    let member = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());

    Ok(QualifiedIdentifier::new(module, member))
}

fn parse_identifier_reference<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<IdentifierReference, Error> {
    trace!("parse_identifier_reference: {:?}", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "identifier" => {
                        return Ok(Identifier::new_unchecked(node_as_str(&node, source)?)
                            .with_ts_span(node.into())
                            .into())
                    }
                    "qualified_identifier" => {
                        return Ok(parse_qualified_identifier(source, &mut node.walk())?.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            "identifier|qualified_identifier",
                            node.kind(),
                        ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    unreachable!()
}

fn parse_type_reference<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeReference, Error> {
    trace!("parse_type_reference: {:?}", cursor.node());
    let mut has_next = true;
    while has_next {
        let node = cursor.node();
        trace!("node {:?} {}", node, node.is_named());
        check_if_error(source, &node)?;
        if node.is_named() {
            match node.kind() {
                "unknown_type" => {
                    return Ok(TypeReference::Unknown);
                }
                "identifier_reference" => {
                    let reference = parse_identifier_reference(source, &mut node.walk())?;
                    return Ok(TypeReference::Reference(reference));
                }
                "line_comment" => {
                    trace!("ignoring comments");
                }
                _ => {
                    return Err(unexpected_node_kind(
                        "unknown_type|identifier_reference",
                        node.kind(),
                    ));
                }
            }
        }
        has_next = cursor.goto_next_sibling();
    }
    unreachable!()
}

fn parse_annotation<'a>(source: &'a str, cursor: &mut TreeCursor<'a>) -> Result<Annotation, Error> {
    let node = cursor.node();
    trace!("parse_annotation: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = parse_identifier_reference(source, &mut child.walk())?;

    let child = node.child_by_field_name("value").unwrap();
    check_if_error(source, &child)?;
    let value = parse_value(source, &mut child.walk())?;

    Ok(Annotation::new(name, value).with_ts_span(node.into()))
}

fn parse_value<'a>(source: &'a str, cursor: &mut TreeCursor<'a>) -> Result<Value, Error> {
    trace!("parse_value: {:?}", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "string" => {
                        return Ok(parse_string(source, cursor)?.into());
                    }
                    "double" => {
                        let value = node_as_str(&node, source)?;
                        let value = f64::from_str(value).expect("Invalid value for Double");
                        return Ok(SimpleValue::from(value).into());
                    }
                    "decimal" => {
                        let value = node_as_str(&node, source)?;
                        let value = Decimal::from_str(value).expect("Invalid value for Decimal");
                        return Ok(SimpleValue::from(value).into());
                    }
                    "integer" => {
                        let value = node_as_str(&node, source)?;
                        let value = i64::from_str(value).expect("Invalid value for Integer");
                        return Ok(SimpleValue::from(value).into());
                    }
                    "boolean" => {
                        let value = node_as_str(&node, source)?;
                        let value = bool::from_str(value).expect("Invalid value for Boolean");
                        return Ok(SimpleValue::from(value).into());
                    }
                    "iri_reference" => {
                        let value = node_as_str(&node, source)?;
                        let value = Url::from_str(&value[1..(value.len() - 1)])
                            .expect("Invalid value for IriReference");
                        return Ok(SimpleValue::from(value).into());
                    }
                    "value_constructor" => {
                        return Ok(parse_value_constructor(source, cursor)?.into());
                    }
                    "identifier_reference" => {
                        return Ok(parse_identifier_reference(source, cursor)?.into());
                    }
                    "list_of_values" => {
                        return Ok(parse_list_of_values(source, cursor)?.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                        "string|double|decimal|integer|boolean|iri_reference|value_constructor|identifier_reference|list_of_values",
                        node.kind(),
                    ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    unreachable!()
}

fn parse_simple_value<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<SimpleValue, Error> {
    trace!("parse_simple_value: {:?}", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "double" => {
                        assert!(cursor.goto_parent());
                        let value = node_as_str(&node, source)?;
                        let value = f64::from_str(value).expect("Invalid value for Double");
                        return Ok(SimpleValue::from(value).into());
                    }
                    "decimal" => {
                        assert!(cursor.goto_parent());
                        let value = node_as_str(&node, source)?;
                        let value = Decimal::from_str(value).expect("Invalid value for Decimal");
                        return Ok(SimpleValue::from(value).into());
                    }
                    "integer" => {
                        assert!(cursor.goto_parent());
                        let value = node_as_str(&node, source)?;
                        let value = i64::from_str(value).expect("Invalid value for Integer");
                        return Ok(SimpleValue::from(value).into());
                    }
                    "boolean" => {
                        assert!(cursor.goto_parent());
                        let value = node_as_str(&node, source)?;
                        let value = bool::from_str(value).expect("Invalid value for Boolean");
                        return Ok(SimpleValue::from(value).into());
                    }
                    "iri_reference" => {
                        assert!(cursor.goto_parent());
                        let value = node_as_str(&node, source)?;
                        let value = Url::from_str(&value[1..(value.len() - 1)])
                            .expect("Invalid value for IriReference");
                        return Ok(SimpleValue::from(value).into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            "string|double|decimal|integer|boolean|iri_reference",
                            node.kind(),
                        ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    unreachable!()
}

fn parse_string<'a>(source: &'a str, cursor: &mut TreeCursor<'a>) -> Result<LanguageString, Error> {
    trace!("parse_string: {:?}", cursor.node());
    let root_node = cursor.node();
    let mut has_next = cursor.goto_first_child();
    if has_next {
        let mut value = String::new();
        let mut language = None;
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "quoted_string" => {
                        let node_value = node_as_str(&node, source)?;
                        value = node_value[1..(node_value.len() - 1)].to_string();
                    }
                    "language_tag" => {
                        let node_value = node_as_str(&node, source)?;
                        language = Some(LanguageTag::new_unchecked(&node_value[1..]));
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            "quoted_string|language_tag",
                            node.kind(),
                        ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
        return Ok(LanguageString::new(&value, language).with_ts_span(root_node.into()));
    }
    unreachable!()
}

fn parse_value_constructor<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<ValueConstructor, Error> {
    let node = cursor.node();
    trace!("parse_value_constructor: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = parse_identifier_reference(source, cursor)?;

    let child = node.child_by_field_name("value").unwrap();
    check_if_error(source, &child)?;
    let value = parse_simple_value(source, cursor)?;

    Ok(ValueConstructor::new(name, value).with_ts_span(node.into()))
}

fn parse_list_of_values<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<ListOfValues, Error> {
    trace!("parse_value: {:?}", cursor.node());
    let mut list_of_values = ListOfValues::default();
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "string" => {
                        assert!(cursor.goto_parent());
                        list_of_values
                            .add_value(SimpleValue::from(parse_string(source, cursor)?).into());
                    }
                    "double" => {
                        assert!(cursor.goto_parent());
                        let value = node_as_str(&node, source)?;
                        let value = f64::from_str(value).expect("Invalid value for Double");
                        list_of_values.add_value(SimpleValue::from(value).into());
                    }
                    "decimal" => {
                        assert!(cursor.goto_parent());
                        let value = node_as_str(&node, source)?;
                        let value = Decimal::from_str(value).expect("Invalid value for Decimal");
                        list_of_values.add_value(SimpleValue::from(value).into());
                    }
                    "integer" => {
                        assert!(cursor.goto_parent());
                        let value = node_as_str(&node, source)?;
                        let value = i64::from_str(value).expect("Invalid value for Integer");
                        list_of_values.add_value(SimpleValue::from(value).into());
                    }
                    "boolean" => {
                        assert!(cursor.goto_parent());
                        let value = node_as_str(&node, source)?;
                        let value = bool::from_str(value).expect("Invalid value for Boolean");
                        list_of_values.add_value(SimpleValue::from(value).into());
                    }
                    "iri_reference" => {
                        assert!(cursor.goto_parent());
                        let value = node_as_str(&node, source)?;
                        let value = Url::from_str(&value[1..(value.len() - 1)])
                            .expect("Invalid value for IriReference");
                        list_of_values.add_value(SimpleValue::from(value).into());
                    }
                    "value_constructor" => {
                        assert!(cursor.goto_parent());
                        list_of_values.add_value(parse_value_constructor(source, cursor)?.into());
                    }
                    "identifier_reference" => {
                        assert!(cursor.goto_parent());
                        list_of_values
                            .add_value(parse_identifier_reference(source, cursor)?.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                        "string|double|decimal|integer|boolean|iri_reference|value_constructor|identifier_reference",
                        node.kind(),
                    ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    unreachable!()
}

fn parse_data_type_def<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<DatatypeDef, Error> {
    let node = cursor.node();
    trace!("parse_data_type_def: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());

    let child = node.child_by_field_name("base_type").unwrap();
    check_if_error(source, &child)?;
    let base_type = parse_identifier_reference(source, cursor)?;

    let mut data_type = DatatypeDef::new(name, base_type).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        check_if_error(source, &child)?;
        let body = parse_annotation_only_body(source, cursor)?;
        data_type.add_body(body);
    }

    Ok(data_type)
}

fn parse_annotation_only_body<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<AnnotationOnlyBody, Error> {
    trace!("parse_annotation_only_body: {:?}", cursor.node());
    let mut body = AnnotationOnlyBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        body.add_annotation(parse_annotation(source, cursor)?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind("annotation", node.kind()));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_entity_def<'a>(source: &'a str, cursor: &mut TreeCursor<'a>) -> Result<EntityDef, Error> {
    let node = cursor.node();
    trace!("parse_entity_def: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());

    let mut entity = EntityDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        check_if_error(source, &child)?;
        let body = parse_entity_body(source, &mut child.walk())?;
        entity.add_body(body);
    }

    Ok(entity)
}

fn parse_entity_body<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityBody, Error> {
    let node = cursor.node();
    trace!("parse_entity_body: {:?}", node);

    let child = node.child_by_field_name("identity").unwrap();
    check_if_error(source, &child)?;
    let identity = parse_identity_member(source, &mut child.walk())?;
    let mut body = EntityBody::new(identity).with_ts_span(node.into());

    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        body.add_annotation(parse_annotation(source, &mut node.walk())?);
                    }
                    "member_by_value" => {
                        body.add_member(parse_by_value_member(source, &mut node.walk())?.into());
                    }
                    "member_by_reference" => {
                        body.add_member(
                            parse_by_reference_member(source, &mut node.walk())?.into(),
                        );
                    }
                    "entity_group" => {
                        body.add_group(parse_entity_group(source, &mut node.walk())?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    "identity_member" => (),
                    _ => {
                        return Err(unexpected_node_kind(
                            "annotation|member_by_value|member_by_reference|entity_group",
                            node.kind(),
                        ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_entity_group<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityGroup, Error> {
    trace!("parse_entity_body: {:?}", cursor.node());
    let mut group = EntityGroup::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        group.add_annotation(parse_annotation(source, cursor)?);
                    }
                    "member_by_value" => {
                        group.add_member(parse_by_value_member(source, cursor)?.into());
                    }
                    "member_by_reference" => {
                        group.add_member(parse_by_reference_member(source, cursor)?.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            "annotation|member_by_value|member_by_reference",
                            node.kind(),
                        ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(group)
}

fn parse_enum_def<'a>(source: &'a str, cursor: &mut TreeCursor<'a>) -> Result<EnumDef, Error> {
    let node = cursor.node();
    trace!("parse_enum_def: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());
    let mut new_enum = EnumDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        check_if_error(source, &child)?;
        let body = parse_enum_body(source, &mut child.walk())?;
        new_enum.add_body(body);
    }

    Ok(new_enum)
}

fn parse_enum_body<'a>(source: &'a str, cursor: &mut TreeCursor<'a>) -> Result<EnumBody, Error> {
    trace!("parse_enum_body: {:?}", cursor.node());
    let mut body = EnumBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        body.add_annotation(parse_annotation(source, &mut node.walk())?);
                    }
                    "enum_variant" => {
                        body.add_variant(parse_enum_variant(source, &mut node.walk())?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind("annotation|enum_variant", node.kind()));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_event_def<'a>(source: &'a str, cursor: &mut TreeCursor<'a>) -> Result<EventDef, Error> {
    let node = cursor.node();
    trace!("parse_event_def: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());

    let child = node.child_by_field_name("source").unwrap();
    check_if_error(source, &child)?;
    let event_source = parse_identifier_reference(source, cursor)?;

    let mut event = EventDef::new(name, event_source).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        check_if_error(source, &child)?;
        let body = parse_structure_body(source, &mut child.walk())?;
        event.add_body(body);
    }

    Ok(event)
}

fn parse_structure_body<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureBody, Error> {
    trace!("parse_structure_body: {:?}", cursor.node());
    let mut body = StructureBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        body.add_annotation(parse_annotation(source, &mut node.walk())?);
                    }
                    "member_by_value" => {
                        body.add_member(parse_by_value_member(source, &mut node.walk())?);
                    }
                    "structure_group" => {
                        body.add_group(parse_structure_group(source, &mut node.walk())?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            "annotation|member_by_value|structure_group",
                            node.kind(),
                        ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_structure_group<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureGroup, Error> {
    trace!("parse_structure_body: {:?}", cursor.node());
    let mut group = StructureGroup::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            check_if_error(source, &node)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        group.add_annotation(parse_annotation(source, &mut node.walk())?);
                    }
                    "member_by_value" => {
                        group.add_member(parse_by_value_member(source, &mut node.walk())?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            "annotation|member_by_value",
                            node.kind(),
                        ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(group)
}

fn parse_structure_def<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureDef, Error> {
    let node = cursor.node();
    trace!("parse_structure_def: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());
    let mut structure = StructureDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        check_if_error(source, &child)?;
        let body = parse_structure_body(source, &mut child.walk())?;
        structure.add_body(body);
    }

    Ok(structure)
}

fn parse_identity_member<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<IdentityMember, Error> {
    let node = cursor.node();
    trace!("parse_identity_member: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());

    let child = node.child_by_field_name("target").unwrap();
    check_if_error(source, &child)?;
    let type_reference = parse_type_reference(source, &mut child.walk())?;

    let mut member = IdentityMember::new(name, type_reference).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        check_if_error(source, &child)?;
        let body = parse_annotation_only_body(source, &mut child.walk())?;
        member.add_body(body);
    }

    Ok(member)
}

fn parse_by_value_member<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<ByValueMember, Error> {
    let node = cursor.node();
    trace!("parse_by_value_member: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());

    let child = node.child_by_field_name("target").unwrap();
    check_if_error(source, &child)?;
    let type_reference = parse_type_reference(source, &mut child.walk())?;

    let mut member = ByValueMember::new(name, type_reference).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("targetCardinality") {
        check_if_error(source, &child)?;
        let cardinality = parse_cardinality(source, &mut child.walk())?;
        member.set_target_cardinality(cardinality);
    }

    if let Some(child) = node.child_by_field_name("body") {
        check_if_error(source, &child)?;
        let body = parse_annotation_only_body(source, &mut child.walk())?;
        member.add_body(body);
    }

    Ok(member)
}

fn parse_by_reference_member<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<ByReferenceMember, Error> {
    let node = cursor.node();
    trace!("parse_by_reference_member: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());

    let child = node.child_by_field_name("target").unwrap();
    check_if_error(source, &child)?;
    let type_reference = parse_type_reference(source, &mut child.walk())?;

    let mut member = ByReferenceMember::new(name, type_reference).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("sourceCardinality") {
        check_if_error(source, &child)?;
        let cardinality = parse_cardinality(source, &mut child.walk())?;
        member.set_source_cardinality(cardinality);
    }

    if let Some(child) = node.child_by_field_name("targetCardinality") {
        check_if_error(source, &child)?;
        let cardinality = parse_cardinality(source, &mut child.walk())?;
        member.set_target_cardinality(cardinality);
    }

    if let Some(child) = node.child_by_field_name("body") {
        check_if_error(source, &child)?;
        let body = parse_annotation_only_body(source, &mut child.walk())?;
        member.add_body(body);
    }

    Ok(member)
}

fn parse_enum_variant<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<EnumVariant, Error> {
    let node = cursor.node();
    trace!("parse_enum_variant: {:?}", node);

    let child = node.child_by_field_name("name").unwrap();
    check_if_error(source, &child)?;
    let name = Identifier::new_unchecked(node_as_str(&child, source)?).with_ts_span(child.into());

    let child = node.child_by_field_name("value").unwrap();
    check_if_error(source, &child)?;
    let text = node_as_str(&child, source)?;
    let value = u32::from_str(text).map_err(|_| invalid_value_for_type(text, "unsigned"))?;

    let mut enum_variant = EnumVariant::new(name, value).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        check_if_error(source, &child)?;
        let body = parse_annotation_only_body(source, &mut child.walk())?;
        enum_variant.add_body(body);
    }

    Ok(enum_variant)
}

fn parse_cardinality<'a>(
    source: &'a str,
    cursor: &mut TreeCursor<'a>,
) -> Result<Cardinality, Error> {
    let node = cursor.node();
    trace!("parse_cardinality: {:?}", node);

    let child = node.child_by_field_name("min").unwrap();
    check_if_error(source, &child)?;
    let text = node_as_str(&child, source)?;
    let min = u32::from_str(text).map_err(|_| invalid_value_for_type(text, "unsigned"))?;

    // TODO: check for range

    if let Some(child) = node.child_by_field_name("max") {
        check_if_error(source, &child)?;
        let text = node_as_str(&child, source)?;
        let max = u32::from_str(text).map_err(|_| invalid_value_for_type(text, "unsigned"))?;
        Ok(Cardinality::new_range(min, max).with_ts_span(node.into()))
    } else {
        Ok(Cardinality::new_single(min).with_ts_span(node.into()))
    }
}

// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn check_if_error<'a>(_source: &'a str, node: &Node<'a>) -> Result<(), Error> {
    if node.is_error() {
        //         ariadne::Report::build(ariadne::ReportKind::Error, source, 1)
        //             .finish()
        //             .eprint(source)?;
        panic!();
    }
    Ok(())
}

#[inline(always)]
fn node_as_str<'a>(node: &'a Node<'a>, source: &'a str) -> Result<&'a str, Error> {
    Ok(node.utf8_text(source.as_bytes())?)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
