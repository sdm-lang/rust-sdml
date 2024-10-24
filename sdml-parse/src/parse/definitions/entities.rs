use crate::parse::annotations::parse_annotation;
use crate::parse::identifiers::parse_identifier;
use crate::parse::members::parse_member;
use crate::parse::{parse_comment, ParseContext};
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::{EntityBody, EntityDef, HasMembers};
use sdml_core::model::members::Member;
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_IDENTITY, FIELD_NAME_NAME, NODE_KIND_ANNOTATION,
    NODE_KIND_ENTITY_IDENTITY, NODE_KIND_IDENTIFIER, NODE_KIND_LINE_COMMENT, NODE_KIND_MEMBER,
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

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
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

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_IDENTITY,
        NODE_KIND_ENTITY_IDENTITY
    );
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
                        body.add_to_members(parse_member(context, &mut node.walk())?);
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
                            [NODE_KIND_ANNOTATION, NODE_KIND_MEMBER,]
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

pub(crate) fn parse_entity_identity<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Member, Error> {
    let node = cursor.node();
    rule_fn!("entity_identity", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_IDENTITY,
        NODE_KIND_MEMBER
    );
    parse_member(context, &mut child.walk())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
