use std::str::FromStr;

use super::ParseContext;
use crate::parse::annotations::parse_annotation;
use crate::parse::definitions::parse_definition;
use crate::parse::identifiers::{parse_identifier, parse_qualified_identifier};
use crate::parse::parse_comment;
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::{HeaderValue, Import, ImportStatement, MemberImport, ModuleImport};
use sdml_core::model::modules::{Module, ModuleBody};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_BASE, FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_RENAME, FIELD_NAME_VERSION_INFO,
    FIELD_NAME_VERSION_URI, NODE_KIND_ANNOTATION, NODE_KIND_DEFINITION, NODE_KIND_IDENTIFIER,
    NODE_KIND_IMPORT_STATEMENT, NODE_KIND_LINE_COMMENT, NODE_KIND_MEMBER_IMPORT,
    NODE_KIND_MODULE_BODY, NODE_KIND_MODULE_IMPORT,
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

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;
    context.module = Some(name.clone());
    context.is_library = Identifier::is_library_module_name(&name);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_MODULE_BODY
    );
    let body = parse_module_body(context, &mut child.walk())?;

    let mut module = Module::new(name, body).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BASE) {
        context.check_if_error(&child, RULE_NAME)?;
        let uri = parse_uri(context, &child)?;
        module.set_base_uri(HeaderValue::from(uri).with_source_span(child.into()));
    }

    if let Some(child) = node.child_by_field_name(FIELD_NAME_VERSION_INFO) {
        context.check_if_error(&child, RULE_NAME)?;
        let info = parse_quoted_string(context, &child)?;
        module.set_version_info(HeaderValue::from(info).with_source_span(child.into()));
    }

    if let Some(child) = node.child_by_field_name(FIELD_NAME_VERSION_URI) {
        context.check_if_error(&child, RULE_NAME)?;
        let uri = parse_uri(context, &child)?;
        module.set_version_uri(HeaderValue::from(uri).with_source_span(child.into()));
    }

    Ok(module)
}

#[inline(always)]
fn parse_uri<'a>(context: &mut ParseContext<'a>, node: &Node<'a>) -> Result<Url, Error> {
    let value = context.node_source(node)?;
    // TODO: turn into real error!
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
    body.set_library_status(&context.module.clone().unwrap());

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_IMPORT_STATEMENT => {
                body.add_to_imports(parse_import_statement(context, &mut node.walk())?);
            }
            NODE_KIND_ANNOTATION => {
                body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
            }
            NODE_KIND_DEFINITION => {
                body.add_to_definitions(parse_definition(context, &mut node.walk())?)?;
            }
            NODE_KIND_LINE_COMMENT => {
                let comment = parse_comment(context, &node)?;
                context.push_comment(comment);
            }
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
                imported.set_source_span(node.into());

                if let Some(child) = node.child_by_field_name(FIELD_NAME_VERSION_URI) {
                    context.check_if_error(&child, RULE_NAME)?;
                    let uri = parse_uri(context, &child)?;
                    imported.set_version_uri(HeaderValue::from(uri).with_source_span(child.into()));
                }

                if let Some(child) = optional_node_field_named!(
                    context,
                    RULE_NAME,
                    node,
                    FIELD_NAME_RENAME,
                    NODE_KIND_IDENTIFIER
                ) {
                    imported.set_rename_as(parse_identifier(context, &child)?)
                }

                let imported: Import = imported.into();
                context.add_import(&imported)?;
                import.add_to_imports(imported);
            }
            NODE_KIND_MEMBER_IMPORT => {
                let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
                context.check_if_error(&child, RULE_NAME)?;
                let mut imported: MemberImport =
                    parse_qualified_identifier(context, &mut child.walk())?.into();
                imported.set_source_span(child.into());

                if let Some(child) = optional_node_field_named!(
                    context,
                    RULE_NAME,
                    node,
                    FIELD_NAME_RENAME,
                    NODE_KIND_IDENTIFIER
                ) {
                    imported.set_rename_as(parse_identifier(context, &child)?)
                }

                let imported: Import = imported.into();
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
