use crate::parse::{
    annotations::parse_annotation,
    definitions::{parse_annotation_only_body, parse_from_definition_clause},
    identifiers::{parse_identifier, parse_identifier_reference},
    parse_comment, ParseContext,
};
use sdml_core::{
    error::Error,
    load::ModuleLoader as ModuleLoaderTrait,
    model::{
        annotations::HasAnnotations,
        definitions::{HasOptionalFromDefinition, TypeVariant, UnionBody, UnionDef},
        HasOptionalBody, HasSourceSpan,
    },
    syntax::{
        FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_RENAME, NODE_KIND_ANNOTATION,
        NODE_KIND_ANNOTATION_ONLY_BODY, NODE_KIND_IDENTIFIER, NODE_KIND_IDENTIFIER_REFERENCE,
        NODE_KIND_LINE_COMMENT, NODE_KIND_TYPE_VARIANT, NODE_KIND_UNION_BODY,NODE_KIND_FROM_DEFINITION_CLAUSE,
    },
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_union_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<UnionDef, Error> {
    let node = cursor.node();
    rule_fn!("union_def", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut new_union = UnionDef::new(name).with_source_span(node.into());

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_UNION_BODY
    ) {
        let body = parse_union_body(context, &mut child.walk())?;
        new_union.set_body(body);
    }

    context.end_type();
    Ok(new_union)
}

fn parse_union_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<UnionBody, Error> {
    rule_fn!("union_body", cursor.node());

    let mut body = UnionBody::default().with_source_span(cursor.node().into());

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_ANNOTATION => {
                body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
            }
            NODE_KIND_FROM_DEFINITION_CLAUSE => {
                body.set_from_definition(parse_from_definition_clause(context, &mut node.walk())?);
            }
            NODE_KIND_TYPE_VARIANT => {
                body.add_to_variants(parse_type_variant(context, &mut node.walk())?);
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
                    [NODE_KIND_ANNOTATION, NODE_KIND_TYPE_VARIANT,]
                );
            }
        }
    }
    Ok(body)
}

fn parse_type_variant<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeVariant, Error> {
    let node = cursor.node();
    rule_fn!("type_variant", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER_REFERENCE
    );
    let name = parse_identifier_reference(context, &mut child.walk())?;
    let type_variant = TypeVariant::new(name).with_source_span(node.into());

    let mut type_variant = if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_RENAME,
        NODE_KIND_IDENTIFIER
    ) {
        let rename = parse_identifier(context, &child)?;
        context.start_variant(&rename)?;
        type_variant.with_rename(rename)
    } else {
        context.start_variant(type_variant.name())?;
        type_variant
    };

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_ANNOTATION_ONLY_BODY
    ) {
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        type_variant.set_body(body);
    }

    Ok(type_variant)
}
