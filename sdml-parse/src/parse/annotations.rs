use super::ParseContext;
use crate::parse::constraints::parse_constraint;
use crate::parse::identifiers::parse_identifier_reference;
use crate::parse::parse_comment;
use crate::parse::values::parse_value;
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::annotations::{Annotation, AnnotationProperty};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_NAME, FIELD_NAME_VALUE, NODE_KIND_ANNOTATION_PROPERTY, NODE_KIND_CONSTRAINT,
    NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_LINE_COMMENT, NODE_KIND_VALUE,
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
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_ANNOTATION_PROPERTY => {
                return Ok(parse_annotation_property(context, &mut node.walk())?.into())
            }
            NODE_KIND_CONSTRAINT => return Ok(parse_constraint(context, &mut node.walk())?.into()),
            NODE_KIND_LINE_COMMENT => {
                let comment = parse_comment(context, &node)?;
                context.push_comment(comment);
            }
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

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER_REFERENCE
    );
    let name = parse_identifier_reference(context, &mut child.walk())?;

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_VALUE, NODE_KIND_VALUE);
    let value = parse_value(context, &mut child.walk())?;

    Ok(AnnotationProperty::new(name, value).with_source_span(node.into()))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
