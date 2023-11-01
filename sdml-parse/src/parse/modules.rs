use crate::parse::annotations::parse_annotation;
use crate::parse::definitions::parse_definition;
use crate::parse::identifiers::{parse_identifier, parse_qualified_identifier};
use sdml_core::error::Error;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::modules::{Import, ImportStatement};
use sdml_core::model::modules::{Module, ModuleBody};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_NAME, NODE_KIND_ANNOTATION, NODE_KIND_DEFINITION,
    NODE_KIND_IMPORT_STATEMENT, NODE_KIND_LINE_COMMENT, NODE_KIND_MEMBER_IMPORT,
    NODE_KIND_MODULE_IMPORT,
};
use tree_sitter::TreeCursor;

use super::ParseContext;

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

    Ok(Module::new(name, body).with_source_span(node.into()))
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
                let node = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
                context.check_if_error(&node, RULE_NAME)?;
                let name: Import = parse_identifier(context, &node)?.into();
                context.add_import(&name)?;
                import.add_to_imports(name);
            }
            NODE_KIND_MEMBER_IMPORT => {
                let node = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
                context.check_if_error(&node, RULE_NAME)?;
                let name: Import = parse_qualified_identifier(context, &mut node.walk())?.into();
                context.add_import(&name)?;
                import.add_to_imports(name);
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
