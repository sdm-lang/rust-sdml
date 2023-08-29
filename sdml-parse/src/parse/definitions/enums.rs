use crate::parse::annotations::parse_annotation;
use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::parse_identifier;
use crate::parse::ParseContext;
use sdml_core::error::{invalid_value_for_type, Error};
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::{EnumBody, EnumDef, HasVariants, ValueVariant};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_VALUE, NODE_KIND_ANNOTATION,
    NODE_KIND_LINE_COMMENT, NODE_KIND_VALUE_VARIANT,
};
use std::str::FromStr;
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

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut new_enum = EnumDef::new(name).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
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
    rule_fn!("parse_enum_body", cursor.node());
    let mut body = EnumBody::default().with_source_span(cursor.node().into());
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
                    NODE_KIND_VALUE_VARIANT => {
                        body.add_to_variants(parse_value_variant(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {}
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
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

fn parse_value_variant<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ValueVariant, Error> {
    let node = cursor.node();
    rule_fn!("value_variant", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_VALUE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let text = context.node_source(&child)?;
    let value = u32::from_str(text).map_err(|_| invalid_value_for_type(text, "unsigned"))?;

    context.start_member(&name)?;
    let mut enum_variant = ValueVariant::new(name, value).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        enum_variant.set_body(body);
    }

    Ok(enum_variant)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
