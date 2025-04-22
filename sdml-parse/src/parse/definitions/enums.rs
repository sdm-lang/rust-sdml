use crate::parse::{
    annotations::parse_annotation, definitions::{parse_annotation_only_body, parse_from_definition_clause},
    identifiers::parse_identifier, parse_comment, ParseContext,
};
use sdml_core::{
    error::Error,
    load::ModuleLoader as ModuleLoaderTrait,
    model::{
        annotations::HasAnnotations,
        definitions::{EnumBody, EnumDef, ValueVariant, HasOptionalFromDefinition},
        HasOptionalBody, HasSourceSpan,
    },
    syntax::{
        FIELD_NAME_BODY, FIELD_NAME_NAME, NODE_KIND_ANNOTATION, NODE_KIND_ANNOTATION_ONLY_BODY,
        NODE_KIND_ENUM_BODY, NODE_KIND_IDENTIFIER, NODE_KIND_LINE_COMMENT, NODE_KIND_VALUE_VARIANT,
        NODE_KIND_FROM_DEFINITION_CLAUSE,
    },
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_enum_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EnumDef, Error> {
    let node = cursor.node();
    rule_fn!("enum_def", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut new_enum = EnumDef::new(name).with_source_span(node.into());

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_ENUM_BODY
    ) {
        let body = parse_enum_body(context, &mut child.walk())?;
        new_enum.set_body(body);
    }

    context.end_type();
    Ok(new_enum)
}

fn parse_enum_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EnumBody, Error> {
    let node = cursor.node();
    rule_fn!("parse_enum_body", node);

    let mut body = EnumBody::default().with_source_span(cursor.node().into());

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_ANNOTATION => {
                body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
            }
            NODE_KIND_FROM_DEFINITION_CLAUSE => {
                body.set_from_definition(parse_from_definition_clause(context, &mut node.walk())?);
            }
            NODE_KIND_VALUE_VARIANT => {
                body.add_to_variants(parse_value_variant(context, &mut node.walk())?);
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
                    [NODE_KIND_ANNOTATION, NODE_KIND_VALUE_VARIANT,]
                );
            }
        }
    }
    Ok(body)
}

fn parse_value_variant<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ValueVariant, Error> {
    let node = cursor.node();
    rule_fn!("value_variant", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;
    context.start_variant(&name)?;
    let mut enum_variant = ValueVariant::new(name).with_source_span(node.into());

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_ANNOTATION_ONLY_BODY
    ) {
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        enum_variant.set_body(body);
    }

    Ok(enum_variant)
}
