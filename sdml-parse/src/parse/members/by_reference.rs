use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::members::{parse_cardinality_expression, parse_type_reference};
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::members::{ByReferenceMember, ByReferenceMemberDef, HasCardinality};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_CARDINALITY, FIELD_NAME_INVERSE_NAME, FIELD_NAME_NAME,
    FIELD_NAME_PROPERTY, FIELD_NAME_TARGET, NODE_KIND_MEMBER_INVERSE_NAME,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_by_reference_member<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ByReferenceMember, Error> {
    let node = cursor.node();
    rule_fn!("by_reference_member", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;
    context.start_member(&name)?;

    if let Some(child) = node.child_by_field_name(FIELD_NAME_TARGET) {
        context.check_if_error(&child, RULE_NAME)?;
        let type_reference = parse_type_reference(context, &mut child.walk())?;
        let mut member_def =
            ByReferenceMemberDef::new(type_reference).with_source_span(node.into());

        if let Some(child) = node.child_by_field_name(FIELD_NAME_INVERSE_NAME) {
            context.check_if_error(&child, RULE_NAME)?;
            if child.kind() == NODE_KIND_MEMBER_INVERSE_NAME {
                let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
                let inverse_name = parse_identifier(context, &child)?;
                member_def.set_inverse_name(inverse_name);
            } else {
                rule_unreachable!(RULE_NAME, cursor);
            }
        }

        if let Some(child) = node.child_by_field_name(FIELD_NAME_CARDINALITY) {
            context.check_if_error(&child, RULE_NAME)?;
            let cardinality = parse_cardinality_expression(context, &mut child.walk())?;
            member_def.set_target_cardinality(cardinality);
        }

        if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
            context.check_if_error(&child, RULE_NAME)?;
            let body = parse_annotation_only_body(context, &mut child.walk())?;
            member_def.set_body(body);
        }

        Ok(ByReferenceMember::new_with_definition(name, member_def).with_source_span(node.into()))
    } else if let Some(child) = node.child_by_field_name(FIELD_NAME_PROPERTY) {
        context.check_if_error(&child, RULE_NAME)?;
        let property = parse_identifier_reference(context, &mut child.walk())?;

        Ok(ByReferenceMember::new_with_role(name, property).with_source_span(node.into()))
    } else {
        rule_unreachable!(RULE_NAME, cursor);
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
