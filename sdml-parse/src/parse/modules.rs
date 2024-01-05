use std::str::FromStr;

use super::ParseContext;
use crate::parse::annotations::parse_annotation;
use crate::parse::definitions::parse_definition;
use crate::parse::identifiers::{parse_identifier, parse_qualified_identifier};
use sdml_core::error::Error;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::modules::{Import, ImportStatement, ModuleImport};
use sdml_core::model::modules::{Module, ModuleBody};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_BASE, FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_VERSION_INFO,
    FIELD_NAME_VERSION_URI, NODE_KIND_ANNOTATION, NODE_KIND_DEFINITION, NODE_KIND_IMPORT_STATEMENT,
    NODE_KIND_LINE_COMMENT, NODE_KIND_MEMBER_IMPORT, NODE_KIND_MODULE_IMPORT,
};
use tree_sitter::{Node, TreeCursor};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_module<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Module, Error> {
    let node = cursor.node();
    rule_fn!("module", node);
    context.check_if_error(&node, RULE_NAME)?;

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_BODY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let body = parse_module_body(context, &mut child.walk())?;

    let mut module = Module::new(name, body).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BASE) {
        context.check_if_error(&child, RULE_NAME)?;
        let uri = parse_uri(context, &child)?;
        module.set_base_uri(uri);
    }

    if let Some(child) = node.child_by_field_name(FIELD_NAME_VERSION_INFO) {
        context.check_if_error(&child, RULE_NAME)?;
        let info = parse_quoted_string(context, &child)?;
        module.set_version_info(info);
    }

    if let Some(child) = node.child_by_field_name(FIELD_NAME_VERSION_URI) {
        context.check_if_error(&child, RULE_NAME)?;
        let uri = parse_uri(context, &child)?;
        module.set_version_uri(uri);
    }

    Ok(module)
}

#[inline(always)]
fn parse_uri<'a>(context: &mut ParseContext<'a>, node: &Node<'a>) -> Result<Url, Error> {
    let value = context.node_source(node)?;
    Ok(Url::from_str(&value[1..(value.len() - 1)]).expect("Invalid value for IriReference"))
}

#[inline(always)]
fn parse_quoted_string<'a>(
    context: &mut ParseContext<'a>,
    node: &Node<'a>,
) -> Result<String, Error> {
    let node_value = context.node_source(node)?;
    Ok(node_value[1..(node_value.len() - 1)].to_string())
}

fn parse_module_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ModuleBody, Error> {
    rule_fn!("module_body", cursor.node());

    let mut body = ModuleBody::default().with_source_span(cursor.node().into());

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_IMPORT_STATEMENT => {
                body.add_to_imports(parse_import_statement(context, &mut node.walk())?)
            }
            NODE_KIND_ANNOTATION => {
                body.add_to_annotations(parse_annotation(context, &mut node.walk())?)
            }
            NODE_KIND_DEFINITION => {
                body.add_to_definitions(parse_definition(context, &mut node.walk())?)
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [
                        NODE_KIND_IMPORT_STATEMENT,
                        NODE_KIND_ANNOTATION,
                        NODE_KIND_DEFINITION,
                    ]
                );
            }
        }
    }
    Ok(body)
}

fn parse_import_statement<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ImportStatement, Error> {
    rule_fn!("import_statement", cursor.node());

    let mut import = ImportStatement::default().with_source_span(cursor.node().into());

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_MODULE_IMPORT => {
                let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
                context.check_if_error(&child, RULE_NAME)?;
                let mut imported: ModuleImport = parse_identifier(context, &child)?.into();

                if let Some(child) = node.child_by_field_name(FIELD_NAME_VERSION_URI) {
                    context.check_if_error(&child, RULE_NAME)?;
                    let uri = parse_uri(context, &child)?;
                    imported.set_version_uri(uri);
                }

                let imported: Import = imported.into();

                context.add_import(&imported)?;
                import.add_to_imports(imported);
            }
            NODE_KIND_MEMBER_IMPORT => {
                let node = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
                context.check_if_error(&node, RULE_NAME)?;
                let imported: Import =
                    parse_qualified_identifier(context, &mut node.walk())?.into();
                context.add_import(&imported)?;
                import.add_to_imports(imported);
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [NODE_KIND_MODULE_IMPORT, NODE_KIND_MEMBER_IMPORT,]
                );
            }
        }
    }
    Ok(import)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
