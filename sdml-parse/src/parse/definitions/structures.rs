use crate::parse::{
    annotations::parse_annotation, definitions::parse_from_definition_clause,
    identifiers::parse_identifier, members::parse_member, parse_comment, ParseContext,
};
use sdml_core::{
    error::Error,
    load::ModuleLoader as ModuleLoaderTrait,
    model::{
        annotations::HasAnnotations,
        definitions::{HasOptionalFromDefinition, StructureBody, StructureDef},
        HasOptionalBody, HasSourceSpan,
    },
    syntax::{
        FIELD_NAME_BODY, FIELD_NAME_NAME, NODE_KIND_ANNOTATION, NODE_KIND_FROM_DEFINITION_CLAUSE,
        NODE_KIND_IDENTIFIER, NODE_KIND_LINE_COMMENT, NODE_KIND_MEMBER, NODE_KIND_STRUCTURE_BODY,
    },
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

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut structure = StructureDef::new(name).with_source_span(node.into());

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_STRUCTURE_BODY
    ) {
        let body = parse_structure_body(context, &mut child.walk())?;
        structure.set_body(body);
    }

    context.end_type();
    Ok(structure)
}

fn parse_structure_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<StructureBody, Error> {
    rule_fn!("structure_body", cursor.node());
    let mut body = StructureBody::default().with_source_span(cursor.node().into());

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_ANNOTATION => {
                body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
            }
            NODE_KIND_FROM_DEFINITION_CLAUSE => {
                body.set_from_definition(parse_from_definition_clause(context, &mut node.walk())?);
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
    Ok(body)
}
