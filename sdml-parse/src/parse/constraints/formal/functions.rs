use crate::parse::{
    constraints::formal::{sentences::parse_constraint_sentence, terms::parse_term},
    identifiers::{parse_identifier, parse_identifier_reference},
    members::parse_mapping_type,
    ParseContext,
};
use sdml_core::{
    error::Error,
    load::ModuleLoader as ModuleLoaderTrait,
    model::{
        constraints::{
            FunctionBody, FunctionCardinality, FunctionDef, FunctionParameter, FunctionSignature,
            FunctionType, FunctionTypeReference,
        },
        identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
        members::{CardinalityRange, Ordering, Uniqueness},
        HasSourceSpan,
    },
    syntax::{
        FIELD_NAME_BODY, FIELD_NAME_CARDINALITY, FIELD_NAME_MAX, FIELD_NAME_MIN, FIELD_NAME_NAME,
        FIELD_NAME_ORDERING, FIELD_NAME_PARAMETER, FIELD_NAME_SIGNATURE, FIELD_NAME_TARGET,
        FIELD_NAME_UNIQUENESS, NAME_SDML, NODE_KIND_BUILTIN_SIMPLE_TYPE,
        NODE_KIND_CONSTRAINT_SENTENCE, NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_LINE_COMMENT,
        NODE_KIND_MAPPING_TYPE, NODE_KIND_TERM, NODE_KIND_UNSIGNED, NODE_KIND_WILDCARD,
    },
};
use sdml_errors::diagnostics::functions::invalid_value_for_type_named;
use std::str::FromStr;
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_function_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FunctionDef, Error> {
    let node = cursor.node();
    rule_fn!("function_def", node);

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_SIGNATURE);
    let signature = parse_function_signature(context, &mut child.walk())?;

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_BODY);
    let body = parse_function_body(context, &mut child.walk())?;

    Ok(FunctionDef::new(signature, body))
}

pub(crate) fn parse_function_signature<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FunctionSignature, Error> {
    let node = cursor.node();
    rule_fn!("function_signature", node);

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_NAME);
    let name = parse_identifier(context, &child)?;

    context.start_member(&name)?;

    let parameters = {
        let mut parameters: Vec<FunctionParameter> = Default::default();
        for binding in node.children_by_field_name(FIELD_NAME_PARAMETER, cursor) {
            check_node!(context, RULE_NAME, &node);
            parameters.push(parse_function_parameter(context, &mut binding.walk())?);
        }
        parameters
    };

    let child = optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_CARDINALITY);
    let cardinality = if let Some(child) = child {
        parse_function_cardinality_expression(context, &mut child.walk())?
    } else {
        FunctionCardinality::new_wildcard()
    };

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_TARGET);
    let fn_type = parse_function_type_reference(context, &mut child.walk())?;
    let fn_type = FunctionType::new(cardinality, fn_type);
    Ok(FunctionSignature::new(name, parameters, fn_type))
}

fn parse_function_parameter<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FunctionParameter, Error> {
    let node = cursor.node();
    rule_fn!("function_parameter", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    let name = parse_identifier(context, &child)?;

    let cardinality = if let Some(child) = node.child_by_field_name(FIELD_NAME_CARDINALITY) {
        parse_function_cardinality_expression(context, &mut child.walk())?
    } else {
        FunctionCardinality::new_wildcard()
    };

    let child = node.child_by_field_name(FIELD_NAME_TARGET).unwrap();
    let fn_type = parse_function_type_reference(context, &mut child.walk())?;
    let fn_type = FunctionType::new(cardinality, fn_type);

    Ok(FunctionParameter::new(name, fn_type))
}

pub(crate) fn parse_function_cardinality_expression<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FunctionCardinality, Error> {
    let node = cursor.node();
    rule_fn!("function_cardinality_expression", node);

    let ordering = if let Some(child) = node.child_by_field_name(FIELD_NAME_ORDERING) {
        context.check_if_error(&child, RULE_NAME)?;
        Some(Ordering::from_str(context.node_source(&child)?)?)
    } else {
        None
    };

    let uniqueness = if let Some(child) = node.child_by_field_name(FIELD_NAME_UNIQUENESS) {
        context.check_if_error(&child, RULE_NAME)?;
        Some(Uniqueness::from_str(context.node_source(&child)?)?)
    } else {
        None
    };

    let range = if let Some(child) = node.child_by_field_name(FIELD_NAME_MIN) {
        context.check_if_error(&child, RULE_NAME)?;
        let text = context.node_source(&child)?;
        let min = u32::from_str(text).map_err(|err| {
            invalid_value_for_type_named(
                context.file_id,
                Some(child.byte_range()),
                text,
                NODE_KIND_UNSIGNED,
                Some(err),
            )
        })?;

        Some(if let Some(child) = node.child_by_field_name("range") {
            if let Some(child) = child.child_by_field_name(FIELD_NAME_MAX) {
                context.check_if_error(&child, RULE_NAME)?;
                let text = context.node_source(&child)?;
                let max = u32::from_str(text).map_err(|err| {
                    invalid_value_for_type_named(
                        context.file_id,
                        Some(child.byte_range()),
                        text,
                        NODE_KIND_UNSIGNED,
                        Some(err),
                    )
                })?;
                CardinalityRange::new_range(min, max)
            } else {
                CardinalityRange::new_unbounded(min)
            }
        } else {
            CardinalityRange::new_single(min)
        })
    } else {
        None
    };

    Ok(FunctionCardinality::new(ordering, uniqueness, range))
}

fn parse_function_type_reference<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FunctionTypeReference, Error> {
    let node = cursor.node();
    rule_fn!("function_type_reference", node);

    for node in node.named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;

        match node.kind() {
            NODE_KIND_WILDCARD => {
                return Ok(FunctionTypeReference::Wildcard.into());
            }
            NODE_KIND_IDENTIFIER_REFERENCE => {
                let ident = parse_identifier_reference(context, &mut node.walk())?;
                return Ok(FunctionTypeReference::Reference(ident).into());
            }
            NODE_KIND_BUILTIN_SIMPLE_TYPE => {
                let module = Identifier::new_unchecked(NAME_SDML);
                let member = parse_identifier(context, &node)?.with_source_span(node.into());
                let ident = IdentifierReference::QualifiedIdentifier(QualifiedIdentifier::new(
                    module, member,
                ));
                return Ok(FunctionTypeReference::Reference(ident).into());
            }
            NODE_KIND_MAPPING_TYPE => {
                let mapping_type = parse_mapping_type(context, &mut node.walk())?;
                return Ok(FunctionTypeReference::MappingType(mapping_type).into());
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [
                        NODE_KIND_WILDCARD,
                        NODE_KIND_IDENTIFIER_REFERENCE,
                        NODE_KIND_BUILTIN_SIMPLE_TYPE,
                        NODE_KIND_MAPPING_TYPE,
                    ]
                );
            }
        }
    }
    missing_node!(
        context,
        RULE_NAME,
        node,
        [
            NODE_KIND_WILDCARD,
            NODE_KIND_IDENTIFIER_REFERENCE,
            NODE_KIND_BUILTIN_SIMPLE_TYPE,
            NODE_KIND_MAPPING_TYPE,
        ]
        .join(" | "),
        "type"
    );
}

pub(crate) fn parse_function_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FunctionBody, Error> {
    let node = cursor.node();
    rule_fn!("function_body", node);

    let child = node.child_by_field_name(FIELD_NAME_BODY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;

    let body: FunctionBody = match child.kind() {
        NODE_KIND_CONSTRAINT_SENTENCE => {
            parse_constraint_sentence(context, &mut child.walk())?.into()
        }
        NODE_KIND_TERM => parse_term(context, &mut child.walk())?.into(),
        _ => {
            unexpected_node!(
                context,
                RULE_NAME,
                node,
                [
                    NODE_KIND_WILDCARD,
                    NODE_KIND_IDENTIFIER_REFERENCE,
                    NODE_KIND_BUILTIN_SIMPLE_TYPE,
                    NODE_KIND_MAPPING_TYPE,
                ]
            );
        }
    };

    Ok(body)
}
