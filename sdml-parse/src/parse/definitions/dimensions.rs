use crate::parse::annotations::parse_annotation;
use crate::parse::definitions::entities::parse_entity_identity;
use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::members::parse_member;
use crate::parse::{parse_comment, ParseContext};
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::{
    DimensionBody, DimensionDef, DimensionIdentity, DimensionParent, HasMembers, SourceEntity,
};

use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_ENTITY, FIELD_NAME_IDENTITY, FIELD_NAME_MEMBER, FIELD_NAME_NAME,
    NODE_KIND_ANNOTATION, NODE_KIND_DIMENSION_PARENT, NODE_KIND_ENTITY_IDENTITY,
    NODE_KIND_IDENTIFIER, NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_LINE_COMMENT, NODE_KIND_MEMBER,
    NODE_KIND_SOURCE_ENTITY,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_dimension_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<DimensionDef, Error> {
    let node = cursor.node();
    rule_fn!("dimension_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut event = DimensionDef::new(name).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_dimension_body(context, &mut child.walk())?;
        event.set_body(body);
    }

    context.end_type();
    Ok(event)
}

fn parse_dimension_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<DimensionBody, Error> {
    let node = cursor.node();
    rule_fn!("dimension_body", node);

    let child = node.child_by_field_name(FIELD_NAME_IDENTITY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;

    let event_source: DimensionIdentity = if child.kind() == NODE_KIND_SOURCE_ENTITY {
        parse_source_entity(context, &mut child.walk())?.into()
    } else if child.kind() == NODE_KIND_ENTITY_IDENTITY {
        parse_entity_identity(context, &mut child.walk())?.into()
    } else {
        unexpected_node!(
            context,
            RULE_NAME,
            node,
            [NODE_KIND_SOURCE_ENTITY, NODE_KIND_ENTITY_IDENTITY,]
        );
    };

    let mut body = DimensionBody::new(event_source).with_source_span(cursor.node().into());

    for node in node.named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_ANNOTATION => {
                body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
            }
            NODE_KIND_DIMENSION_PARENT => {
                body.add_to_parents(parse_dimension_parent(context, &mut node.walk())?);
            }
            NODE_KIND_MEMBER => {
                body.add_to_members(parse_member(context, &mut node.walk())?);
            }
            NODE_KIND_LINE_COMMENT => {
                let comment = parse_comment(context, &node)?;
                context.push_comment(comment);
            }
            // These node kinds are handled above
            NODE_KIND_SOURCE_ENTITY | NODE_KIND_ENTITY_IDENTITY => {}
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

fn parse_dimension_parent<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<DimensionParent, Error> {
    let node = cursor.node();
    rule_fn!("dimension_parent", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_ENTITY,
        NODE_KIND_IDENTIFIER_REFERENCE
    );
    let target_entity = parse_identifier_reference(context, &mut child.walk())?;

    let mut parent = DimensionParent::new(name, target_entity);

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        parent.set_body(body);
    }

    Ok(parent)
}

pub(crate) fn parse_source_entity<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<SourceEntity, Error> {
    let node = cursor.node();
    rule_fn!("source_entity", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_ENTITY,
        NODE_KIND_IDENTIFIER_REFERENCE
    );
    let target_entity = parse_identifier_reference(context, &mut child.walk())?;

    let mut source = SourceEntity::new(target_entity);

    for child in node.children_by_field_name(FIELD_NAME_MEMBER, cursor) {
        source.add_to_members(parse_identifier(context, &child)?);
    }

    Ok(source)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
