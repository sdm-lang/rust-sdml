use crate::parse::annotations::parse_annotation;
use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::members::{
    parse_member, parse_member_group, parse_type_reference,
};
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::{EntityBody, EntityDef, EntityIdentity, EntityIdentityDef, HasGroups, HasMembers};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_IDENTITY, FIELD_NAME_NAME, NODE_KIND_ANNOTATION, FIELD_NAME_PROPERTY, FIELD_NAME_TARGET,
    NODE_KIND_MEMBER, NODE_KIND_MEMBER_GROUP, NODE_KIND_ENTITY_IDENTITY, NODE_KIND_LINE_COMMENT,
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
    let identity = parse_entity_identity(context, &mut child.walk())?;
    let mut body = EntityBody::new(identity).with_source_span(node.into());

    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ENTITY_IDENTITY => {
                        // ignore: this is the identity field above
                    }
                    NODE_KIND_ANNOTATION => {
                        body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_MEMBER => {
                        body.add_to_members(
                            parse_member(context, &mut node.walk())?.into(),
                        );
                    }
                    NODE_KIND_MEMBER_GROUP => {
                        body.add_to_groups(parse_member_group(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_MEMBER,
                                NODE_KIND_MEMBER_GROUP,
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

fn parse_entity_identity<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EntityIdentity, Error> {
    let node = cursor.node();
    rule_fn!("identity_member", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;
    context.start_member(&name)?;

    if let Some(child) = node.child_by_field_name(FIELD_NAME_TARGET) {
        context.check_if_error(&child, RULE_NAME)?;
        let type_reference = parse_type_reference(context, &mut child.walk(), false)?;
        let mut member_def = EntityIdentityDef::new(type_reference).with_source_span(node.into());

        if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
            context.check_if_error(&child, RULE_NAME)?;
            let body = parse_annotation_only_body(context, &mut child.walk())?;
            member_def.set_body(body);
        };

        Ok(EntityIdentity::new_definition(name, member_def).with_source_span(node.into()))
    } else if let Some(child) = node.child_by_field_name(FIELD_NAME_PROPERTY) {
        context.check_if_error(&child, RULE_NAME)?;
        let property = parse_identifier_reference(context, &mut child.walk())?;

        Ok(EntityIdentity::new_property_reference(name, property).with_source_span(node.into()))
    } else {
        rule_unreachable!(RULE_NAME, cursor);
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
