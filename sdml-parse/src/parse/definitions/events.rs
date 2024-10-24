use crate::parse::annotations::parse_annotation;
use crate::parse::definitions::dimensions::parse_source_entity;
use crate::parse::identifiers::parse_identifier;
use crate::parse::members::parse_member;
use crate::parse::{parse_comment, ParseContext};
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::{EventBody, EventDef, HasMembers};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_IDENTITY, FIELD_NAME_NAME, NODE_KIND_ANNOTATION,
    NODE_KIND_IDENTIFIER, NODE_KIND_LINE_COMMENT, NODE_KIND_MEMBER, NODE_KIND_SOURCE_ENTITY,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_event_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EventDef, Error> {
    let node = cursor.node();
    rule_fn!("event_def", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut event = EventDef::new(name).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_event_body(context, &mut child.walk())?;
        event.set_body(body);
    }

    context.end_type();
    Ok(event)
}

fn parse_event_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EventBody, Error> {
    let node = cursor.node();
    rule_fn!("event_body", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_IDENTITY,
        NODE_KIND_SOURCE_ENTITY
    );
    let event_source = parse_source_entity(context, &mut child.walk())?;

    let mut body = EventBody::new(event_source).with_source_span(cursor.node().into());

    for node in node.named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_ANNOTATION => {
                body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
            }
            NODE_KIND_MEMBER => {
                body.add_to_members(parse_member(context, &mut node.walk())?);
            }
            NODE_KIND_SOURCE_ENTITY => {}
            NODE_KIND_LINE_COMMENT => {
                let comment = parse_comment(context, &node)?;
                context.push_comment(comment);
            }
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [NODE_KIND_ANNOTATION, NODE_KIND_MEMBER,]
                );
            }
        }
    }

    Ok(body)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
