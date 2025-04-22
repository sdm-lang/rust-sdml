use crate::parse::{
    definitions::parse_annotation_only_body,
    identifiers::{parse_identifier, parse_identifier_reference},
    members::{parse_cardinality_expression, parse_type_reference},
    ParseContext,
};
use sdml_core::{
    error::Error,
    load::ModuleLoader,
    model::{
        identifiers::IdentifierReference,
        members::{Member, MemberDef},
        HasOptionalBody, HasSourceSpan,
    },
    syntax::{
        FIELD_NAME_BODY, FIELD_NAME_CARDINALITY, FIELD_NAME_NAME, FIELD_NAME_PROPERTY,
        FIELD_NAME_TARGET, NODE_KIND_ANNOTATION_ONLY_BODY, NODE_KIND_CARDINALITY_EXPRESSION,
        NODE_KIND_IDENTIFIER, NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_LINE_COMMENT,
        NODE_KIND_MEMBER_DEF, NODE_KIND_PROPERTY_REF, NODE_KIND_TYPE_REFERENCE,
    },
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_member<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Member, Error> {
    let node = cursor.node();
    rule_fn!("member", node);

    for child in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match child.kind() {
            NODE_KIND_MEMBER_DEF => {
                return Ok(parse_member_def(context, &mut child.walk())?
                    .with_source_span(node.into())
                    .into());
            }
            NODE_KIND_PROPERTY_REF => {
                return Ok(parse_property_ref(context, &mut child.walk())?
                    .with_source_span(node.into())
                    .into());
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [NODE_KIND_PROPERTY_REF, NODE_KIND_MEMBER_DEF,]
                );
            }
        }
    }

    rule_unreachable!(RULE_NAME, cursor);
}

pub(crate) fn parse_member_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<MemberDef, Error> {
    let node = cursor.node();
    rule_fn!("member_def", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;
    context.start_member(&name)?;

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_TARGET,
        NODE_KIND_TYPE_REFERENCE
    );
    let type_reference = parse_type_reference(context, &mut child.walk())?;

    let mut member_def = MemberDef::new(name, type_reference).with_source_span(node.into());

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_CARDINALITY,
        NODE_KIND_CARDINALITY_EXPRESSION
    ) {
        let cardinality = parse_cardinality_expression(context, &mut child.walk())?;
        member_def.set_target_cardinality(cardinality);
    }

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_ANNOTATION_ONLY_BODY
    ) {
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        member_def.set_body(body);
    }

    Ok(member_def)
}

pub(crate) fn parse_property_ref<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<IdentifierReference, Error> {
    let node = cursor.node();
    rule_fn!("property_ref", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_PROPERTY,
        NODE_KIND_IDENTIFIER_REFERENCE
    );
    let id_ref = parse_identifier_reference(context, &mut child.walk())?;

    context.start_member(id_ref.member())?;

    Ok(id_ref)
}
