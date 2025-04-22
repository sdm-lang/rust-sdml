use crate::parse::{members::parse_member_def, ParseContext};
use sdml_core::{
    error::Error,
    load::ModuleLoader,
    model::{definitions::PropertyDef, HasSourceSpan},
    syntax::{FIELD_NAME_MEMBER, NODE_KIND_MEMBER_DEF},
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_property_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<PropertyDef, Error> {
    let node = cursor.node();
    rule_fn!("property_def", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_MEMBER,
        NODE_KIND_MEMBER_DEF
    );
    let member_def = parse_member_def(context, &mut child.walk())?.with_source_span(child.into());

    context.end_type();
    Ok(member_def.into())
}
