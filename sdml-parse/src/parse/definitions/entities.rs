use crate::parse::annotations::parse_annotation;
use crate::parse::identifiers::parse_identifier;
use crate::parse::members::{
    parse_by_reference_member, parse_by_value_member, parse_identity_member,
};
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::{EntityBody, EntityDef, EntityGroup, HasGroups, HasMembers};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_IDENTITY, FIELD_NAME_NAME, NODE_KIND_ANNOTATION,
    NODE_KIND_ENTITY_GROUP, NODE_KIND_IDENTITY_MEMBER, NODE_KIND_LINE_COMMENT,
    NODE_KIND_MEMBER_BY_REFERENCE, NODE_KIND_MEMBER_BY_VALUE,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_entity_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityDef, Error> {
    let node = cursor.node();
    rule_fn!("entity_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut entity = EntityDef::new(name).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_entity_body(context, &mut child.walk())?;
        entity.set_body(body);
    }

    context.end_type();
    Ok(entity)
}

fn parse_entity_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityBody, Error> {
    let node = cursor.node();
    rule_fn!("entity_body", node);

    let child = node.child_by_field_name(FIELD_NAME_IDENTITY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let identity = parse_identity_member(context, &mut child.walk())?;
    let mut body = EntityBody::new(identity).with_source_span(node.into());

    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_IDENTITY_MEMBER => {
                        // ignore: this is the identity field above
                    }
                    NODE_KIND_ANNOTATION => {
                        body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_MEMBER_BY_VALUE => {
                        body.add_to_members(
                            parse_by_value_member(context, &mut node.walk())?.into(),
                        );
                    }
                    NODE_KIND_MEMBER_BY_REFERENCE => {
                        body.add_to_members(
                            parse_by_reference_member(context, &mut node.walk())?.into(),
                        );
                    }
                    NODE_KIND_ENTITY_GROUP => {
                        body.add_to_groups(parse_entity_group(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_MEMBER_BY_VALUE,
                                NODE_KIND_MEMBER_BY_REFERENCE,
                                NODE_KIND_ENTITY_GROUP,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_entity_group<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityGroup, Error> {
    rule_fn!("entity_group", cursor.node());
    let mut group = EntityGroup::default().with_source_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        group.add_to_annotations(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_MEMBER_BY_VALUE => {
                        group.add_to_members(parse_by_value_member(context, cursor)?.into());
                    }
                    NODE_KIND_MEMBER_BY_REFERENCE => {
                        group.add_to_members(parse_by_reference_member(context, cursor)?.into());
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_MEMBER_BY_VALUE,
                                NODE_KIND_MEMBER_BY_REFERENCE,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(group)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
