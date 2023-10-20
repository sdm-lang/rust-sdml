use super::ParseContext;
use crate::parse::constraints::parse_constraint;
use crate::parse::identifiers::parse_identifier_reference;
use crate::parse::values::parse_value;
use sdml_core::error::Error;
use sdml_core::model::annotations::{Annotation, AnnotationProperty};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_NAME, FIELD_NAME_VALUE, NODE_KIND_ANNOTATION_PROPERTY, NODE_KIND_CONSTRAINT,
    NODE_KIND_LINE_COMMENT,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_annotation<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Annotation, Error> {
    rule_fn!("annotation", cursor.node());

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_ANNOTATION_PROPERTY => {
                return Ok(parse_annotation_property(context, &mut node.walk())?.into())
            }
            NODE_KIND_CONSTRAINT => {
                return Ok(parse_constraint(context, &mut node.walk())?.into())
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [NODE_KIND_ANNOTATION_PROPERTY, NODE_KIND_CONSTRAINT,]
                );
            }
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}

fn parse_annotation_property<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<AnnotationProperty, Error> {
    let node = cursor.node();
    rule_fn!("annotation_property", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier_reference(context, &mut child.walk())?;

    let child = node.child_by_field_name(FIELD_NAME_VALUE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let value = parse_value(context, &mut child.walk())?;

    Ok(AnnotationProperty::new(name, value).with_source_span(node.into()))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
