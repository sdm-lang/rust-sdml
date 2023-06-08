/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_value_for_type, module_parse_error, unexpected_node_kind, Error};
use crate::model::{
    Annotation, AnnotationOnlyBody, ByReferenceMember, ByValueMember, Cardinality, DatatypeDef,
    EntityBody, EntityDef, EntityGroup, EnumBody, EnumDef, EnumVariant, EventDef, Identifier,
    IdentifierReference, IdentityMember, Import, ImportStatement, LanguageString, LanguageTag,
    ListOfValues, Module, ModuleBody, QualifiedIdentifier, SimpleValue, StructureBody,
    StructureDef, StructureGroup, TypeDefinition, TypeReference, TypeVariant, UnionBody, UnionDef,
    Value, ValueConstructor,
};
use ariadne::Source;
use rust_decimal::Decimal;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use tracing::{error, trace};
use tree_sitter::Parser;
use tree_sitter::{Node, TreeCursor};
use tree_sitter_sdml::language;
use url::Url;

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

macro_rules! rule_fn {
    ($name:literal, $node:expr) => {
        const RULE_NAME: &str = $name;
        trace!("{}: {:?}", RULE_NAME, $node);
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct ParseContext<'a> {
    source: &'a str,
    ariadne: Option<Source>,
    imports: HashSet<Import>,
    type_names: HashSet<Identifier>,
    member_names: HashSet<Identifier>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> ParseContext<'a> {
    fn new(source: &'a str) -> Self {
        Self  {
            source,
            imports: Default::default(),
            type_names: Default::default(),
            member_names: Default::default(),
        }
    }

   fn node_source(&self, node: &'a Node<'a>) -> Result<&'a str, Error> {
        Ok(node.utf8_text(self.source.as_bytes())?)
    }

    fn check_if_error(&self, node: &Node<'a>, rule: &str) -> Result<(), Error> {
        if node.is_error() {
            //         ariadne::Report::build(ariadne::ReportKind::Error, source, 1)
            //             .finish()
            //             .eprint(source)?;
            Err(module_parse_error(
                node.kind(),
                node.start_byte(),
                node.end_byte(),
                Some(rule),
            ))
        } else {
            Ok(())
        }
    }

    fn add_import(&mut self, import: &Import) {
        if self.imports.contains(import) {
            error!("Duplicate import: {}", import);
        } else {
            self.imports.insert(import.clone());
        }
    }

    fn start_type(&mut self, name: &Identifier) {
        if self.type_names.contains(name) {
            error!("Duplicate type: {}", name);
        } else {
            self.type_names.insert(name.clone());
        }
    }

    fn start_member(&mut self, name: &Identifier) {
        if self.member_names.contains(name) {
            error!("Duplicate member: {}", name);
        } else {
            self.member_names.insert(name.clone());
        }
    }

    fn end_type(&mut self) {
        self.member_names.clear()
    }
}

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
        let mut context = ParseContext::new(source);
        let mut cursor = tree.walk();
        parse_module(&mut context, &mut cursor)
    } else {
        Err(unexpected_node_kind(
            "parse_str_inner",
            "module",
            node.kind(),
        ))
    }
}

fn parse_module<'a>(context: &mut ParseContext<'a>, cursor: &mut TreeCursor<'a>) -> Result<Module, Error> {
    let node = cursor.node();
    rule_fn!("parse_module", node);
    context.check_if_error(&node, RULE_NAME)?;

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name("body").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let body = parse_module_body(context, &mut child.walk())?;

    Ok(Module::new(name, body).with_ts_span(node.into()))
}

fn parse_module_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ModuleBody, Error> {
    rule_fn!("parse_module_body", cursor.node());
    let mut body = ModuleBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "import_statement" => {
                        body.add_import(parse_import_statement(context, &mut node.walk())?)
                    }
                    "annotation" => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?)
                    }
                    "type_def" => {
                        body.add_definition(parse_type_definition(context, &mut node.walk())?)
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
                            "import_statement|annotation|type_def",
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

fn parse_import_statement<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ImportStatement, Error> {
    rule_fn!("parse_import_statement", cursor.node());
    let mut import = ImportStatement::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            trace!(
                "{} (child): {:?} named: {}",
                RULE_NAME,
                node,
                node.is_named()
            );
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "import" => {
                        let imported = parse_import(context, &mut node.walk())?;
                        context.add_import(&imported);
                        import.add_import(imported);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
                            "import",
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

fn parse_import<'a>(context: &mut ParseContext<'a>,  cursor: &mut TreeCursor<'a>) -> Result<Import, Error> {
    rule_fn!("parse_import", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            trace!(
                "parse_import (child): {:?} named: {}",
                node,
                node.is_named()
            );
            context.check_if_error(&node, "parse_import")?;
            if node.is_named() {
                match node.kind() {
                    "module_import" => {
                        let node = node.child_by_field_name("name").unwrap();
                        context.check_if_error(&node, "parse_import")?;
                        let name = parse_identifier(context, &node)?;
                        return Ok(name.into());
                    }
                    "member_import" => {
                        let node = node.child_by_field_name("name").unwrap();
                        context.check_if_error(&node, "parse_import")?;
                        let name = parse_qualified_identifier(context, &mut node.walk())?;
                        return Ok(name.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            "parse_import",
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
    unreachable!()
}

fn parse_identifier<'a>(
    context: &mut ParseContext<'a>,
    node: &Node<'a>,
) -> Result<Identifier, Error> {
    rule_fn!("parse_identifier",node);
    Ok(Identifier::new_unchecked(context.node_source(&node)?).with_ts_span(node.into()))
}

fn parse_qualified_identifier<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<QualifiedIdentifier, Error> {
    let node = cursor.node();
    rule_fn!("parse_qualified_identifier", node);

    let child = node.child_by_field_name("module").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let module = parse_identifier(context, &child)?;

    let child = node.child_by_field_name("member").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let member = parse_identifier(context, &child)?;

    Ok(QualifiedIdentifier::new(module, member))
}

fn parse_identifier_reference<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<IdentifierReference, Error> {
    rule_fn!("parse_identifier_reference", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "identifier" => {
                        return Ok(parse_identifier(context, &node)?.into())
                    }
                    "qualified_identifier" => {
                        return Ok(parse_qualified_identifier(context, &mut node.walk())?.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
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

fn parse_annotation<'a>(context: &mut ParseContext<'a>,  cursor: &mut TreeCursor<'a>) -> Result<Annotation, Error> {
    let node = cursor.node();
    rule_fn!("parse_annotation", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier_reference(context, &mut child.walk())?;

    let child = node.child_by_field_name("value").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let value = parse_value(context, &mut child.walk())?;

    Ok(Annotation::new(name, value).with_ts_span(node.into()))
}

fn parse_value<'a>(context: &mut ParseContext<'a>,  cursor: &mut TreeCursor<'a>) -> Result<Value, Error> {
    rule_fn!("parse_value", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "simple_value" => {
                        return Ok(parse_simple_value(context, &mut node.walk())?.into());
                    }
                    "value_constructor" => {
                        return Ok(parse_value_constructor(context, cursor)?.into());
                    }
                    "identifier_reference" => {
                        return Ok(parse_identifier_reference(context, cursor)?.into());
                    }
                    "list_of_values" => {
                        return Ok(parse_list_of_values(context, cursor)?.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
                            "string|double|decimal|integer|boolean|iri_reference|value_constructor|identifier_reference|list_of_values",
                            node.kind(),
                    ));
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
    }
    unreachable!()
}

fn parse_simple_value<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<SimpleValue, Error> {
    rule_fn!("parse_simple_value", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "string" => {
                        let value = parse_string(context, cursor)?;
                        return Ok(SimpleValue::String(value));
                    }
                    "double" => {
                        let value = context.node_source(&node)?;
                        let value = f64::from_str(value).expect("Invalid value for Double");
                        return Ok(SimpleValue::Double(value.into()));
                    }
                    "decimal" => {
                        let value = context.node_source(&node)?;
                        let value = Decimal::from_str(value).expect("Invalid value for Decimal");
                        return Ok(SimpleValue::Decimal(value));
                    }
                    "integer" => {
                        let value = context.node_source(&node)?;
                        let value = i64::from_str(value).expect("Invalid value for Integer");
                        return Ok(SimpleValue::Integer(value));
                    }
                    "boolean" => {
                        let value = context.node_source(&node)?;
                        let value = bool::from_str(value).expect("Invalid value for Boolean");
                        return Ok(SimpleValue::Boolean(value));
                    }
                    "iri_reference" => {
                        let value = context.node_source(&node)?;
                        let value = Url::from_str(&value[1..(value.len() - 1)])
                            .expect("Invalid value for IriReference");
                        return Ok(SimpleValue::IriReference(value));
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
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

fn parse_string<'a>(context: &mut ParseContext<'a>,  cursor: &mut TreeCursor<'a>) -> Result<LanguageString, Error> {
    rule_fn!("parse_string", cursor.node());
    let root_node = cursor.node();
    let mut has_next = cursor.goto_first_child();
    if has_next {
        let mut value = String::new();
        let mut language = None;
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "quoted_string" => {
                        let node_value = context.node_source(&node)?;
                        value = node_value[1..(node_value.len() - 1)].to_string();
                    }
                    "language_tag" => {
                        let node_value = context.node_source(&node)?;
                        language = Some(LanguageTag::new_unchecked(&node_value[1..]));
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
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
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ValueConstructor, Error> {
    let node = cursor.node();
    rule_fn!("parse_value_constructor", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier_reference(context, &mut child.walk())?;

    let child = node.child_by_field_name("value").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let value = parse_simple_value(context, &mut child.walk())?;

    Ok(ValueConstructor::new(name, value).with_ts_span(node.into()))
}

fn parse_list_of_values<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ListOfValues, Error> {
    rule_fn!("parse_list_of_values", cursor.node());
    let mut list_of_values = ListOfValues::default();
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "simple_value" => {
                        list_of_values
                            .add_value(parse_simple_value(context, &mut node.walk())?.into());
                    }
                    "value_constructor" => {
                        list_of_values.add_value(parse_value_constructor(context, cursor)?.into());
                    }
                    "identifier_reference" => {
                        list_of_values
                            .add_value(parse_identifier_reference(context, cursor)?.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
                        "string|double|decimal|integer|boolean|iri_reference|value_constructor|identifier_reference",
                        node.kind(),
                    ));
                    }
                }
            }
            // BUG: loop condition will lose the last element?
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(list_of_values)
}

fn parse_type_definition<'a>(
    context: &mut ParseContext<'a>, 
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeDefinition, Error> {
    rule_fn!("parse_type_definition", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "data_type_def" => {
                        return Ok(parse_data_type_def(context, &mut node.walk())?.into());
                    }
                    "entity_def" => {
                        return Ok(parse_entity_def(context, &mut node.walk())?.into());
                    }
                    "enum_def" => {
                        return Ok(parse_enum_def(context, &mut node.walk())?.into());
                    }
                    "event_def" => {
                        return Ok(parse_event_def(context, &mut node.walk())?.into());
                    }
                    "structure_def" => {
                        return Ok(parse_structure_def(context, &mut node.walk())?.into());
                    }
                    "union_def" => {
                        return Ok(parse_union_def(context, &mut node.walk())?.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
                            "data_type_def|entity_def|enum_def|event_def|structure_def",
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
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<DatatypeDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_data_type_def", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name("base").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let base_type = parse_identifier_reference(context, &mut child.walk())?;

    context.start_type(&name);
    let mut data_type = DatatypeDef::new(name, base_type).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, cursor)?;
        data_type.add_body(body);
    }

    context.end_type();
    Ok(data_type)
}

fn parse_annotation_only_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<AnnotationOnlyBody, Error> {
    rule_fn!("parse_annotation_only_body", cursor.node());
    let mut body = AnnotationOnlyBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        body.add_annotation(parse_annotation(context, cursor)?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
                            "annotation",
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

fn parse_entity_def<'a>(context: &mut ParseContext<'a>, cursor: &mut TreeCursor<'a>) -> Result<EntityDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_entity_def", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name);
    let mut entity = EntityDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_entity_body(context, &mut child.walk())?;
        entity.add_body(body);
    }

    context.end_type();
    Ok(entity)
}

fn parse_entity_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityBody, Error> {
    let node = cursor.node();
    rule_fn!("parse_entity_body", node);

    let child = node.child_by_field_name("identity").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let identity = parse_identity_member(context, &mut child.walk())?;
    let mut body = EntityBody::new(identity).with_ts_span(node.into());

    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    "member_by_value" => {
                        body.add_member(parse_by_value_member(context, &mut node.walk())?.into());
                    }
                    "member_by_reference" => {
                        body.add_member(
                            parse_by_reference_member(context, &mut node.walk())?.into(),
                        );
                    }
                    "entity_group" => {
                        body.add_group(parse_entity_group(context, &mut node.walk())?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    "identity_member" => (),
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
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
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityGroup, Error> {
    rule_fn!("parse_entity_body", cursor.node());
    let mut group = EntityGroup::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        group.add_annotation(parse_annotation(context, cursor)?);
                    }
                    "member_by_value" => {
                        group.add_member(parse_by_value_member(context, cursor)?.into());
                    }
                    "member_by_reference" => {
                        group.add_member(parse_by_reference_member(context, cursor)?.into());
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
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

fn parse_enum_def<'a>(context: &mut ParseContext<'a>,  cursor: &mut TreeCursor<'a>) -> Result<EnumDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_enum_def", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name);
    let mut new_enum = EnumDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_enum_body(context, &mut child.walk())?;
        new_enum.add_body(body);
    }

    context.end_type();
    Ok(new_enum)
}

fn parse_enum_body<'a>(context: &mut ParseContext<'a>,  cursor: &mut TreeCursor<'a>) -> Result<EnumBody, Error> {
    rule_fn!("parse_enum_body", cursor.node());
    let mut body = EnumBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    "enum_variant" => {
                        body.add_variant(parse_enum_variant(context, &mut node.walk())?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
                            "annotation|enum_variant",
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

fn parse_event_def<'a>(context: &mut ParseContext<'a>,  cursor: &mut TreeCursor<'a>) -> Result<EventDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_event_def", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name("source").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let event_source = parse_identifier_reference(context, cursor)?;

    context.start_type(&name);
    let mut event = EventDef::new(name, event_source).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_structure_body(context, &mut child.walk())?;
        event.add_body(body);
    }

    context.end_type();
    Ok(event)
}

fn parse_structure_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureBody, Error> {
    rule_fn!("parse_structure_body", cursor.node());
    let mut body = StructureBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    "member_by_value" => {
                        body.add_member(parse_by_value_member(context, &mut node.walk())?);
                    }
                    "structure_group" => {
                        body.add_group(parse_structure_group(context, &mut node.walk())?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
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
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureGroup, Error> {
    rule_fn!("parse_structure_body: {:?}", cursor.node());
    let mut group = StructureGroup::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        group.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    "member_by_value" => {
                        group.add_member(parse_by_value_member(context, &mut node.walk())?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
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
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_structure_def", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name);
    let mut structure = StructureDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_structure_body(context, &mut child.walk())?;
        structure.add_body(body);
    }

    context.end_type();
    Ok(structure)
}

fn parse_union_def<'a>(context: &mut ParseContext<'a>,  cursor: &mut TreeCursor<'a>) -> Result<UnionDef, Error> {
    let node = cursor.node();
    rule_fn!("parse_union_def", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name);
    let mut new_enum = UnionDef::new(name).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_union_body(context, &mut child.walk())?;
        new_enum.add_body(body);
    }

    context.end_type();
    Ok(new_enum)
}

fn parse_union_body<'a>(context: &mut ParseContext<'a>,  cursor: &mut TreeCursor<'a>) -> Result<UnionBody, Error> {
    rule_fn!("parse_union_body", cursor.node());
    let mut body = UnionBody::default().with_ts_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    "annotation" => {
                        body.add_annotation(parse_annotation(context, &mut node.walk())?);
                    }
                    "type_variant" => {
                        body.add_variant(parse_type_variant(context, &mut node.walk())?);
                    }
                    "line_comment" => {
                        trace!("ignoring comments");
                    }
                    _ => {
                        return Err(unexpected_node_kind(
                            RULE_NAME,
                            "annotation|type_variant",
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

fn parse_identity_member<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<IdentityMember, Error> {
    let node = cursor.node();
    rule_fn!("parse_identity_member", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name("target").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let type_reference = parse_type_reference(context, &mut child.walk())?;

    context.start_member(&name);
    let mut member = IdentityMember::new(name, type_reference).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        member.add_body(body);
    }

    Ok(member)
}

fn parse_by_value_member<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ByValueMember, Error> {
    let node = cursor.node();
    rule_fn!("parse_by_value_member", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name("target").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let type_reference = parse_type_reference(context, &mut child.walk())?;

    context.start_member(&name);
    let mut member = ByValueMember::new(name, type_reference).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("target_cardinality") {
        context.check_if_error(&child, RULE_NAME)?;
        let cardinality = parse_cardinality(context, &mut child.walk())?;
        member.set_target_cardinality(cardinality);
    }

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        member.add_body(body);
    }

    Ok(member)
}

fn parse_by_reference_member<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ByReferenceMember, Error> {
    let node = cursor.node();
    rule_fn!("parse_by_reference_member", node);

    let child = node.child_by_field_name("name").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name("target").unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let type_reference = parse_type_reference(context, &mut child.walk())?;

    context.start_member(&name);
    let mut member = ByReferenceMember::new(name, type_reference).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("source_cardinality") {
        context.check_if_error(&child, RULE_NAME)?;
        let cardinality = parse_cardinality(context, &mut child.walk())?;
        member.set_source_cardinality(cardinality);
    }

    if let Some(child) = node.child_by_field_name("target_cardinality") {
        context.check_if_error(&child, RULE_NAME)?;
        let cardinality = parse_cardinality(context, &mut child.walk())?;
        member.set_target_cardinality(cardinality);
    }

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        member.add_body(body);
    }

    Ok(member)
}

fn parse_type_reference<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeReference, Error> {
    rule_fn!("parse_type_reference", cursor.node());
    let mut has_next = cursor.goto_first_child();
    while has_next {
        let node = cursor.node();
        trace!("node {:?} {}", node, node.is_named());
        context.check_if_error(&node, RULE_NAME)?;
        if node.is_named() {
            match node.kind() {
                "unknown_type" => {
                    return Ok(TypeReference::Unknown);
                }
                "identifier_reference" => {
                    let reference = parse_identifier_reference(context, &mut node.walk())?;
                    return Ok(TypeReference::Reference(reference));
                }
                "line_comment" => {
                    trace!("ignoring comments");
                }
                _ => {
                    return Err(unexpected_node_kind(
                        RULE_NAME,
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

fn parse_enum_variant<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EnumVariant, Error> {
    let node = cursor.node();
    rule_fn!("parse_enum_variant", node);

    let child = node.child_by_field_name("name").unwrap();
        context.check_if_error(&child, RULE_NAME)?;
   let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name("value").unwrap();
        context.check_if_error(&child, RULE_NAME)?;
    let text = context.node_source(&child)?;
    let value = u32::from_str(text).map_err(|_| invalid_value_for_type(text, "unsigned"))?;

    context.start_member(&name);
    let mut enum_variant = EnumVariant::new(name, value).with_ts_span(node.into());

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        enum_variant.add_body(body);
    }

    Ok(enum_variant)
}

fn parse_type_variant<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeVariant, Error> {
    let node = cursor.node();
    rule_fn!("parse_type_variant", node);

    let child = node.child_by_field_name("name").unwrap();
        context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier_reference(context, &mut child.walk())?;

    // FIX: context.start_member(&name);
    let type_variant = TypeVariant::new(name).with_ts_span(node.into());

    let mut type_variant = if let Some(child) = node.child_by_field_name("rename") {
        context.check_if_error(&child, RULE_NAME)?;
        let rename = parse_identifier(context, &child)?;
        type_variant.with_rename(rename)
    } else {
        type_variant
    };

    if let Some(child) = node.child_by_field_name("body") {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        type_variant.add_body(body);
    }

    Ok(type_variant)
}

fn parse_cardinality<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Cardinality, Error> {
    let node = cursor.node();
    rule_fn!("parse_cardinality", node);

    let child = node.child_by_field_name("min").unwrap();
        context.check_if_error(&child, RULE_NAME)?;
    let text = context.node_source(&child)?;
    let min = u32::from_str(text).map_err(|_| invalid_value_for_type(text, "unsigned"))?;

    // TODO: check for range

    if let Some(child) = node.child_by_field_name("max") {
        context.check_if_error(&child, RULE_NAME)?;
       let text = context.node_source(&child)?;
        let max = u32::from_str(text).map_err(|_| invalid_value_for_type(text, "unsigned"))?;
        Ok(Cardinality::new_range(min, max).with_ts_span(node.into()))
    } else {
        Ok(Cardinality::new_single(min).with_ts_span(node.into()))
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
