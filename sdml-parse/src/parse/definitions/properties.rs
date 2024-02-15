use crate::parse::annotations::parse_annotation;
use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::parse_identifier;
use crate::parse::members::{parse_cardinality_expression, parse_type_reference};
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::annotations::HasAnnotations;
use sdml_core::model::definitions::{EntityIdentityDef, PropertyBody, PropertyDef, PropertyRole};
use sdml_core::model::members::{HasCardinality, MemberDef};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_CARDINALITY, FIELD_NAME_INVERSE_NAME, FIELD_NAME_NAME,
    FIELD_NAME_TARGET, NODE_KIND_ANNOTATION, NODE_KIND_IDENTITY_ROLE, NODE_KIND_LINE_COMMENT,
    NODE_KIND_MEMBER_ROLE,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_property_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<PropertyDef, Error> {
    let node = cursor.node();
    rule_fn!("property_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_type(&name)?;
    let mut new_prop = PropertyDef::new(name).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_property_body(context, &mut child.walk())?;
        new_prop.set_body(body);
    }

    context.end_type();
    Ok(new_prop)
}

fn parse_property_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<PropertyBody, Error> {
    rule_fn!("property_body", cursor.node());
    let mut body = PropertyBody::default().with_source_span(cursor.node().into());
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
                    NODE_KIND_IDENTITY_ROLE => {
                        body.add_to_roles(parse_identity_role(context, &mut node.walk())?);
                    }
                    NODE_KIND_MEMBER_ROLE => {
                        body.add_to_roles(parse_member_role(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_ANNOTATION,
                                NODE_KIND_IDENTITY_ROLE,
                                NODE_KIND_MEMBER_ROLE,
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

fn parse_identity_role<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<PropertyRole, Error> {
    let node = cursor.node();
    rule_fn!("identity_role", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;
    context.start_member(&name)?;

    let child = node.child_by_field_name(FIELD_NAME_TARGET).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let type_reference = parse_type_reference(context, &mut child.walk(), false)?;
    let mut def = EntityIdentityDef::new(type_reference).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        def.set_body(body);
    }

    Ok(PropertyRole::new(name, def).with_source_span(node.into()))
}

fn parse_member_role<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<PropertyRole, Error> {
    let node = cursor.node();
    rule_fn!("role_by_reference", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;
    context.start_member(&name)?;

    let child = node.child_by_field_name(FIELD_NAME_TARGET).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let type_reference = parse_type_reference(context, &mut child.walk(), false)?;
    let mut def = MemberDef::new(type_reference).with_source_span(node.into());

    let child = node.child_by_field_name(FIELD_NAME_CARDINALITY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let cardinality = parse_cardinality_expression(context, &mut child.walk())?;
    def.set_target_cardinality(cardinality);

    if let Some(child) = node.child_by_field_name(FIELD_NAME_INVERSE_NAME) {
        context.check_if_error(&child, RULE_NAME)?;
        let inverse_name = parse_identifier(context, &child)?;
        def.set_inverse_name(inverse_name);
    }

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        def.set_body(body);
    }

    Ok(PropertyRole::new(name, def).with_source_span(node.into()))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
