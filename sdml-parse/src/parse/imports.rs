use super::ParseContext;
use crate::parse::identifiers::{parse_identifier, parse_qualified_identifier};
use sdml_core::error::Error;
use sdml_core::model::modules::{Import, ImportStatement};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_NAME, NODE_KIND_IMPORT, NODE_KIND_LINE_COMMENT, NODE_KIND_MEMBER_IMPORT,
    NODE_KIND_MODULE_IMPORT,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_import_statement<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ImportStatement, Error> {
    rule_fn!("import_statement", cursor.node());
    let mut import = ImportStatement::default().with_source_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
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
                        let name: Import =
                            parse_qualified_identifier(context, &mut node.walk())?.into();
                        context.add_import(&name)?;
                        import.add_to_imports(name.into());
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(context, RULE_NAME, node, NODE_KIND_IMPORT);
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(import)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
