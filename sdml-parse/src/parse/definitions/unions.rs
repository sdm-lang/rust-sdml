use crate::parse::annotations::parse_annotation;
use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::{parse_comment, ParseContext};
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::{HasVariants, TypeVariant, UnionBody, UnionDef};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_RENAME, NODE_KIND_ANNOTATION,
    NODE_KIND_LINE_COMMENT, NODE_KIND_TYPE_VARIANT,
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

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut new_enum = UnionDef::new(name).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_union_body(context, &mut child.walk())?;
        new_enum.set_body(body);
    }

    context.end_type();
    Ok(new_enum)
}

fn parse_union_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<UnionBody, Error> {
    rule_fn!("union_body", cursor.node());
    let mut body = UnionBody::default().with_source_span(cursor.node().into());
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
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_type_variant<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeVariant, Error> {
    let node = cursor.node();
    rule_fn!("type_variant", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier_reference(context, &mut child.walk())?;

    let type_variant = TypeVariant::new(name).with_source_span(node.into());

    let mut type_variant = if let Some(child) = node.child_by_field_name(FIELD_NAME_RENAME) {
        context.check_if_error(&child, RULE_NAME)?;
        let rename = parse_identifier(context, &child)?;
        context.start_variant(&rename)?;
        type_variant.with_rename(rename)
    } else {
        context.start_variant(type_variant.name())?;
        type_variant
    };

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        type_variant.set_body(body);
    }

    Ok(type_variant)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
