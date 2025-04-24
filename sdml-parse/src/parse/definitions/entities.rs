use crate::parse::{
    annotations::parse_annotation, identifiers::parse_identifier, members::parse_member,
    definitions::parse_from_definition_clause,
    ParseContext,
};
use sdml_core::{
    error::Error,
    load::ModuleLoader as ModuleLoaderTrait,
    model::{
        annotations::HasAnnotations,
        definitions::{HasOptionalFromDefinition, EntityBody, EntityDef},
        members::Member,
        HasOptionalBody, HasSourceSpan,
    },
    syntax::{
        FIELD_NAME_BODY, FIELD_NAME_IDENTITY, FIELD_NAME_NAME, NODE_KIND_ANNOTATION,
        NODE_KIND_ENTITY_BODY, NODE_KIND_ENTITY_IDENTITY, NODE_KIND_IDENTIFIER,
        NODE_KIND_LINE_COMMENT, NODE_KIND_MEMBER,NODE_KIND_FROM_DEFINITION_CLAUSE,
    },
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

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_ENTITY_BODY
    ) {
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

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_ENTITY_IDENTITY => {
                // ignore: this is the identity field above
            }
            NODE_KIND_ANNOTATION => {
                body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
            }
            NODE_KIND_FROM_DEFINITION_CLAUSE => {
                body.set_from_definition(parse_from_definition_clause(context, &mut node.walk())?);
            }
            NODE_KIND_MEMBER => {
                body.add_to_members(parse_member(context, &mut node.walk())?);
            }
            NODE_KIND_LINE_COMMENT => {}
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
