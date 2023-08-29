use crate::parse::annotations::parse_annotation;
use crate::parse::identifiers::parse_identifier;
use crate::parse::members::parse_by_value_member;
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::{
    HasGroups, HasMembers, StructureBody, StructureDef, StructureGroup,
};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_NAME, NODE_KIND_ANNOTATION, NODE_KIND_LINE_COMMENT,
    NODE_KIND_MEMBER_BY_VALUE, NODE_KIND_STRUCTURE_GROUP, NODE_KIND_STRUCTURE_MEMBER,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_structure_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureDef, Error> {
    let node = cursor.node();
    rule_fn!("structure_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut structure = StructureDef::new(name).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_structure_body(context, &mut child.walk())?;
        structure.set_body(body);
    }

    context.end_type();
    Ok(structure)
}

pub(crate) fn parse_structure_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureBody, Error> {
    rule_fn!("structure_body", cursor.node());
    let mut body = StructureBody::default().with_source_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_MEMBER_BY_VALUE => {
                        body.add_to_members(parse_by_value_member(context, &mut node.walk())?);
                    }
                    NODE_KIND_STRUCTURE_GROUP => {
                        body.add_to_groups(parse_structure_group(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_STRUCTURE_MEMBER,
                                NODE_KIND_STRUCTURE_GROUP,
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

fn parse_structure_group<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureGroup, Error> {
    rule_fn!("structure_group", cursor.node());
    let mut group = StructureGroup::default().with_source_span(cursor.node().into());
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
                        group.add_to_members(parse_by_value_member(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [NODE_KIND_ANNOTATION, NODE_KIND_STRUCTURE_MEMBER,]
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
