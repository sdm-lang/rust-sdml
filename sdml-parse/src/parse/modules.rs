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
use sdml_core::model::modules::Module;
use sdml_core::model::modules::{
    HeaderValue, Import, ImportStatement, MemberImport, ModuleImport, ModulePath,
};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_BASE, FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_RENAME, FIELD_NAME_SEGMENT,
    FIELD_NAME_VERSION_INFO, FIELD_NAME_VERSION_URI, NODE_KIND_ANNOTATION, NODE_KIND_DEFINITION,
    NODE_KIND_FROM_CLAUSE, NODE_KIND_IDENTIFIER, NODE_KIND_IMPORT_STATEMENT, NODE_KIND_IRI,
    NODE_KIND_LINE_COMMENT, NODE_KIND_MEMBER_IMPORT, NODE_KIND_MODULE_BODY,
    NODE_KIND_MODULE_IMPORT, NODE_KIND_MODULE_PATH_ABSOLUTE, NODE_KIND_MODULE_PATH_RELATIVE,
    NODE_KIND_MODULE_PATH_ROOT, NODE_KIND_QUALIFIED_IDENTIFIER, NODE_KIND_QUOTED_STRING,
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
    let mut module = Module::new(name).with_source_span(node.into());

    if let Some(child) =
        optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_BASE, NODE_KIND_IRI)
    {
        let uri = parse_uri(context, &child)?;
        module.set_base_uri(HeaderValue::from(uri).with_source_span(child.into()));
    }

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_VERSION_INFO,
        NODE_KIND_QUOTED_STRING
    ) {
        let info = parse_quoted_string(context, &child)?;
        module.set_version_info(HeaderValue::from(info).with_source_span(child.into()));
    }

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_VERSION_URI,
        NODE_KIND_IRI
    ) {
        let uri = parse_uri(context, &child)?;
        module.set_version_uri(HeaderValue::from(uri).with_source_span(child.into()));
    }

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_MODULE_BODY
    );
    parse_module_body(context, &mut module, &mut child.walk())?;

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
    module: &mut Module,
    cursor: &mut TreeCursor<'a>,
) -> Result<(), Error> {
    rule_fn!("module_body", cursor.node());

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_IMPORT_STATEMENT => {
                module.add_to_imports(parse_import_statement(context, &mut node.walk())?);
            }
            NODE_KIND_ANNOTATION => {
                module.add_to_annotations(parse_annotation(context, &mut node.walk())?);
            }
            NODE_KIND_DEFINITION => {
                module.add_to_definitions(parse_definition(context, &mut node.walk())?)?;
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
    Ok(())
}

fn parse_import_statement<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ImportStatement, Error> {
    rule_fn!("import_statement", cursor.node());

    let mut import = ImportStatement::default().with_source_span(cursor.node().into());

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_FROM_CLAUSE => {
                import.set_from_module_path(parse_from_clause(context, &mut node.walk())?);
            }
            NODE_KIND_MODULE_IMPORT => {
                let child = node_field_named!(
                    context,
                    RULE_NAME,
                    node,
                    FIELD_NAME_NAME,
                    NODE_KIND_IDENTIFIER
                );
                let mut imported: ModuleImport = parse_identifier(context, &child)?.into();
                imported.set_source_span(node.into());

                if let Some(child) = optional_node_field_named!(
                    context,
                    RULE_NAME,
                    node,
                    FIELD_NAME_VERSION_URI,
                    NODE_KIND_IRI
                ) {
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
                let child = node_field_named!(
                    context,
                    RULE_NAME,
                    node,
                    FIELD_NAME_NAME,
                    NODE_KIND_QUALIFIED_IDENTIFIER
                );
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
                    [
                        NODE_KIND_FROM_CLAUSE,
                        NODE_KIND_MODULE_IMPORT,
                        NODE_KIND_MEMBER_IMPORT,
                    ]
                );
            }
        }
    }
    Ok(import)
}

fn parse_from_clause<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ModulePath, Error> {
    rule_fn!("from_clause", cursor.node());

    if let Some(child) = cursor.node().named_children(cursor).next() {
        check_node!(context, RULE_NAME, &child);
        match child.kind() {
            NODE_KIND_MODULE_PATH_ABSOLUTE => {
                return Ok(ModulePath::absolute(parse_module_path(
                    context,
                    &mut child.walk(),
                )?))
            }
            NODE_KIND_MODULE_PATH_RELATIVE => {
                return Ok(ModulePath::relative(parse_module_path(
                    context,
                    &mut child.walk(),
                )?))
            }
            NODE_KIND_MODULE_PATH_ROOT => return Ok(ModulePath::root()),
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    child,
                    [
                        NODE_KIND_MODULE_PATH_ABSOLUTE,
                        NODE_KIND_MODULE_PATH_RELATIVE,
                        NODE_KIND_MODULE_PATH_ROOT,
                    ]
                );
            }
        }
    }
    unreachable!()
}

fn parse_module_path<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Vec<Identifier>, Error> {
    rule_fn!("module_path", cursor.node());
    let mut path = Vec::default();

    for child in cursor
        .node()
        .children_by_field_name(FIELD_NAME_SEGMENT, cursor)
    {
        check_node!(context, RULE_NAME, &child, NODE_KIND_IDENTIFIER);
        path.push(parse_identifier(context, &child)?);
    }

    Ok(path)
}
