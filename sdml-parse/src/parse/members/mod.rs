use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::ParseContext;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::identifiers::{Identifier, QualifiedIdentifier};
use sdml_core::model::members::{Cardinality, MappingType, Ordering, TypeReference, Uniqueness};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_DOMAIN, FIELD_NAME_MAX, FIELD_NAME_MIN, FIELD_NAME_ORDERING, FIELD_NAME_RANGE,
    FIELD_NAME_UNIQUENESS, NAME_SDML, NODE_KIND_BUILTIN_SIMPLE_TYPE,
    NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_MAPPING_TYPE, NODE_KIND_SEQUENCE_ORDERING,
    NODE_KIND_SEQUENCE_UNIQUENESS, NODE_KIND_TYPE_REFERENCE, NODE_KIND_UNKNOWN_TYPE,
    NODE_KIND_UNSIGNED,
};
use sdml_errors::diagnostics::functions::invalid_value_for_type_named;
use sdml_errors::Error;
use std::str::FromStr;
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_cardinality_expression<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Cardinality, Error> {
    let node = cursor.node();
    rule_fn!("cardinality_expression", node);

    let ordering = if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_ORDERING,
        NODE_KIND_SEQUENCE_ORDERING
    ) {
        Some(Ordering::from_str(context.node_source(&child)?)?)
    } else {
        None
    };

    let uniqueness = if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_UNIQUENESS,
        NODE_KIND_SEQUENCE_UNIQUENESS
    ) {
        Some(Uniqueness::from_str(context.node_source(&child)?)?)
    } else {
        None
    };

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_MIN, NODE_KIND_UNSIGNED);
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

    let expr = if let Some(range) =
        optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_RANGE)
    {
        if let Some(child) = optional_node_field_named!(
            context,
            RULE_NAME,
            range,
            FIELD_NAME_MAX,
            NODE_KIND_UNSIGNED
        ) {
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
            Cardinality::new_range(min, max)
        } else {
            Cardinality::new_unbounded(min)
        }
    } else {
        Cardinality::new_single(min)
    };
    Ok(expr
        .with_source_span(node.into())
        .with_ordering(ordering)
        .with_uniqueness(uniqueness))
}

pub(crate) fn parse_type_reference<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeReference, Error> {
    let node = cursor.node();
    rule_fn!("type_reference", node);

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_UNKNOWN_TYPE => {
                return Ok(TypeReference::Unknown);
            }
            NODE_KIND_IDENTIFIER_REFERENCE => {
                return Ok(TypeReference::Type(parse_identifier_reference(
                    context,
                    &mut node.walk(),
                )?));
            }
            NODE_KIND_BUILTIN_SIMPLE_TYPE => {
                let module = Identifier::new_unchecked(NAME_SDML);
                let member = parse_identifier(context, &node)?.with_source_span(node.into());
                return Ok(TypeReference::Type(
                    QualifiedIdentifier::new(module, member)
                        .with_source_span(node.into())
                        .into(),
                ));
            }
            NODE_KIND_MAPPING_TYPE => {
                let mapping = parse_mapping_type(context, &mut node.walk())?;
                return Ok(TypeReference::MappingType(mapping));
            }
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [
                        NODE_KIND_UNKNOWN_TYPE,
                        NODE_KIND_IDENTIFIER_REFERENCE,
                        NODE_KIND_BUILTIN_SIMPLE_TYPE,
                    ]
                );
            }
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}

pub(crate) fn parse_mapping_type<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<MappingType, Error> {
    let node = cursor.node();
    rule_fn!("mapping_type", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_DOMAIN,
        NODE_KIND_TYPE_REFERENCE
    );
    let domain = parse_type_reference(context, &mut child.walk())?;

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_RANGE,
        NODE_KIND_TYPE_REFERENCE
    );
    let range = parse_type_reference(context, &mut child.walk())?;

    Ok(MappingType::new(domain, range).with_source_span(node.into()))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod member;
pub(crate) use member::{parse_member, parse_member_def};
