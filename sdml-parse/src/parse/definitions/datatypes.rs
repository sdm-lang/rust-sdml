use crate::parse::{
    definitions::parse_annotation_only_body,
    identifiers::{parse_identifier, parse_identifier_reference},
    ParseContext,
};
use sdml_core::{
    error::Error,
    load::ModuleLoader,
    model::{
        definitions::{DatatypeDef, ExplicitTimezoneFlag, RestrictionFacet},
        identifiers::{Identifier, IdentifierReference, QualifiedIdentifier},
        HasOptionalBody, HasSourceSpan,
    },
    syntax::{
        FIELD_NAME_BASE, FIELD_NAME_BODY, FIELD_NAME_FACET, FIELD_NAME_IS_FIXED, FIELD_NAME_NAME,
        FIELD_NAME_RESTRICTION, FIELD_NAME_VALUE, NAME_SDML, NODE_KIND_ANNOTATION_ONLY_BODY,
        NODE_KIND_BUILTIN_SIMPLE_TYPE, NODE_KIND_DATATYPE_DEF_RESTRICTION,
        NODE_KIND_DIGIT_RESTRICTION_FACET, NODE_KIND_IDENTIFIER, NODE_KIND_IDENTIFIER_REFERENCE,
        NODE_KIND_LENGTH_RESTRICTION_FACET, NODE_KIND_PATTERN_RESTRICTION_FACET,
        NODE_KIND_TZ_RESTRICTION_FACET, NODE_KIND_UNSIGNED, NODE_KIND_VALUE_RESTRICTION_FACET,
    },
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_data_type_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<DatatypeDef, Error> {
    let node = cursor.node();
    rule_fn!("data_type_def", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_BASE);
    let base_type = match child.kind() {
        NODE_KIND_IDENTIFIER_REFERENCE => parse_identifier_reference(context, &mut child.walk())?,
        NODE_KIND_BUILTIN_SIMPLE_TYPE => {
            // TODO: lookup in map to determine true type.
            let module = Identifier::new_unchecked(NAME_SDML);
            let member = parse_identifier(context, &child)?.with_source_span(child.into());
            IdentifierReference::QualifiedIdentifier(QualifiedIdentifier::new(module, member))
        }
        _ => {
            rule_unreachable!(RULE_NAME, cursor);
        }
    };

    context.start_type(&name)?;
    let mut data_type = DatatypeDef::new(name, base_type).with_source_span(node.into());

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_RESTRICTION,
        NODE_KIND_DATATYPE_DEF_RESTRICTION
    ) {
        data_type.extend_restrictions(parse_datatype_def_restriction(context, &mut child.walk())?);
    }

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_ANNOTATION_ONLY_BODY
    ) {
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        data_type.set_body(body);
    }

    context.end_type();
    Ok(data_type)
}

pub(super) fn parse_datatype_def_restriction<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Vec<RestrictionFacet>, Error> {
    let node = cursor.node();
    rule_fn!("datatype_def_restriction", node);

    let mut facets = Vec::default();
    for child in node.named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        facets.push(parse_restriction_facet(context, &mut child.walk())?);
    }
    Ok(facets)
}

pub(super) fn parse_restriction_facet<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<RestrictionFacet, Error> {
    let node = cursor.node();
    rule_fn!("_restriction_facet", node);

    let fixed = if let Some(_) =
        optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_IS_FIXED)
    {
        true
    } else {
        false
    };

    match node.kind() {
        NODE_KIND_LENGTH_RESTRICTION_FACET => {
            let value = if let Some(child) = optional_node_field_named!(
                context,
                RULE_NAME,
                node,
                FIELD_NAME_VALUE,
                NODE_KIND_UNSIGNED
            ) {
                context.node_source(&child)?.parse::<u32>().unwrap() // TODO: don't panic
            } else {
                panic!()
            };
            if let Some(child) =
                optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_FACET)
            {
                match context.node_source(&child)? {
                    "length" => return Ok(RestrictionFacet::Length(value, fixed)),
                    "maxLength" => return Ok(RestrictionFacet::MaxLength(value, fixed)),
                    "minLength" => return Ok(RestrictionFacet::MinLength(value, fixed)),
                    _ => panic!(),
                }
            } else {
                panic!()
            };
        }
        NODE_KIND_DIGIT_RESTRICTION_FACET => {
            let value = if let Some(child) =
                optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_VALUE)
            {
                context.node_source(&child)?.parse::<u32>().unwrap() // TODO: don't panic
            } else {
                panic!()
            };
            if let Some(child) =
                optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_FACET)
            {
                match context.node_source(&child)? {
                    "fractionDigits" => return Ok(RestrictionFacet::FractionDigits(value, fixed)),
                    "totalDigits" => return Ok(RestrictionFacet::TotalDigits(value, fixed)),
                    _ => panic!(),
                }
            } else {
                panic!()
            };
        }
        NODE_KIND_VALUE_RESTRICTION_FACET => todo!(),
        NODE_KIND_TZ_RESTRICTION_FACET => {
            let value = if let Some(child) =
                optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_VALUE)
            {
                context
                    .node_source(&child)?
                    .parse::<ExplicitTimezoneFlag>()
                    .unwrap() // TODO: don't panic
            } else {
                panic!()
            };
            return Ok(RestrictionFacet::ExplicitTimezone(value, fixed));
        }
        NODE_KIND_PATTERN_RESTRICTION_FACET => {
            let mut values = Vec::default();
            for child in node.children_by_field_name(FIELD_NAME_VALUE, &mut node.walk()) {
                check_node!(context, RULE_NAME, &node);
                values.push(context.node_source(&child)?.to_string());
            }
            return Ok(RestrictionFacet::Pattern(values));
        }
        _ => {
            unexpected_node!(
                context,
                RULE_NAME,
                node,
                [
                    NODE_KIND_LENGTH_RESTRICTION_FACET,
                    NODE_KIND_DIGIT_RESTRICTION_FACET,
                    NODE_KIND_VALUE_RESTRICTION_FACET,
                    NODE_KIND_TZ_RESTRICTION_FACET,
                    NODE_KIND_PATTERN_RESTRICTION_FACET,
                ]
            );
        }
    }
}
