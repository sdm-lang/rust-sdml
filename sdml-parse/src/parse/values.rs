use crate::parse::identifiers::parse_identifier_reference;
use rust_decimal::Decimal;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::values::{
    Binary, LanguageString, LanguageTag, MappingValue, SequenceOfValues, SimpleValue, Value,
    ValueConstructor,
};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_BYTE, FIELD_NAME_DOMAIN, FIELD_NAME_NAME, FIELD_NAME_RANGE, FIELD_NAME_VALUE,
    NODE_KIND_BINARY, NODE_KIND_BOOLEAN, NODE_KIND_DECIMAL, NODE_KIND_DOUBLE,
    NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_INTEGER, NODE_KIND_IRI, NODE_KIND_LANGUAGE_TAG,
    NODE_KIND_LINE_COMMENT, NODE_KIND_MAPPING_VALUE, NODE_KIND_QUOTED_STRING,
    NODE_KIND_SEQUENCE_OF_VALUES, NODE_KIND_SIMPLE_VALUE, NODE_KIND_STRING, NODE_KIND_UNSIGNED,
    NODE_KIND_VALUE_CONSTRUCTOR,
};
use sdml_errors::Error;
use std::str::FromStr;
use tree_sitter::TreeCursor;
use url::Url;

use super::ParseContext;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_value<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Value, Error> {
    rule_fn!("value", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_SIMPLE_VALUE => {
                        return Ok(parse_simple_value(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_VALUE_CONSTRUCTOR => {
                        return Ok(parse_value_constructor(context, cursor)?.into());
                    }
                    NODE_KIND_IDENTIFIER_REFERENCE => {
                        return Ok(parse_identifier_reference(context, cursor)?.into());
                    }
                    NODE_KIND_MAPPING_VALUE => {
                        return Ok(parse_mapping_value(context, cursor)?.into());
                    }
                    NODE_KIND_SEQUENCE_OF_VALUES => {
                        return Ok(parse_sequence_of_values(context, cursor)?.into());
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_SIMPLE_VALUE,
                                NODE_KIND_VALUE_CONSTRUCTOR,
                                NODE_KIND_IDENTIFIER_REFERENCE,
                                NODE_KIND_SEQUENCE_OF_VALUES,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}

pub(crate) fn parse_simple_value<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<SimpleValue, Error> {
    rule_fn!("simple_value", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_BOOLEAN => {
                        let value = context.node_source(&node)?;
                        return if value == "⊤" {
                            Ok(SimpleValue::Boolean(true))
                        } else if value == "⊥" {
                            Ok(SimpleValue::Boolean(false))
                        } else {
                            match bool::from_str(value) {
                                Ok(value) => Ok(SimpleValue::Boolean(value)),
                                Err(err) => {
                                    invalid_value_for_node_type!(
                                        context,
                                        RULE_NAME,
                                        node,
                                        value,
                                        Some(err)
                                    );
                                }
                            }
                        };
                    }
                    NODE_KIND_UNSIGNED => {
                        let value = context.node_source(&node)?;
                        return match u64::from_str(value) {
                            Ok(value) => Ok(SimpleValue::Unsigned(value)),
                            Err(err) => {
                                invalid_value_for_node_type!(
                                    context,
                                    RULE_NAME,
                                    node,
                                    value,
                                    Some(err)
                                );
                            }
                        };
                    }
                    NODE_KIND_INTEGER => {
                        let value = context.node_source(&node)?;
                        return match i64::from_str(value) {
                            Ok(value) => Ok(SimpleValue::Integer(value)),
                            Err(err) => {
                                invalid_value_for_node_type!(
                                    context,
                                    RULE_NAME,
                                    node,
                                    value,
                                    Some(err)
                                );
                            }
                        };
                    }
                    NODE_KIND_DECIMAL => {
                        let value = context.node_source(&node)?;
                        return match Decimal::from_str(value) {
                            Ok(value) => Ok(SimpleValue::Decimal(value)),
                            Err(err) => {
                                invalid_value_for_node_type!(
                                    context,
                                    RULE_NAME,
                                    node,
                                    value,
                                    Some(err)
                                );
                            }
                        };
                    }
                    NODE_KIND_DOUBLE => {
                        let value = context.node_source(&node)?;
                        return match f64::from_str(value) {
                            Ok(value) => Ok(SimpleValue::Double(value.into())),
                            Err(err) => {
                                invalid_value_for_node_type!(
                                    context,
                                    RULE_NAME,
                                    node,
                                    value,
                                    Some(err)
                                );
                            }
                        };
                    }
                    NODE_KIND_STRING => {
                        let value = parse_string(context, cursor)?;
                        return Ok(SimpleValue::String(value));
                    }
                    NODE_KIND_IRI => {
                        let value = context.node_source(&node)?;
                        return match Url::from_str(&value[1..(value.len() - 1)]) {
                            Ok(value) => Ok(SimpleValue::IriReference(value)),
                            Err(err) => {
                                invalid_value_for_node_type!(
                                    context,
                                    RULE_NAME,
                                    node,
                                    value,
                                    Some(err)
                                );
                            }
                        };
                    }
                    NODE_KIND_BINARY => {
                        let value = parse_binary(context, cursor)?;
                        return Ok(SimpleValue::Binary(value));
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_STRING,
                                NODE_KIND_DOUBLE,
                                NODE_KIND_DECIMAL,
                                NODE_KIND_INTEGER,
                                NODE_KIND_BOOLEAN,
                                NODE_KIND_IRI,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    rule_unreachable!(RULE_NAME, cursor);
}

fn parse_string<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<LanguageString, Error> {
    rule_fn!("string", cursor.node());
    let root_node = cursor.node();
    let mut has_next = cursor.goto_first_child();
    if has_next {
        let mut value = String::new();
        let mut language = None;
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_QUOTED_STRING => {
                        let node_value = context.node_source(&node)?;
                        value = node_value[1..(node_value.len() - 1)].to_string();
                    }
                    NODE_KIND_LANGUAGE_TAG => {
                        let node_value = context.node_source(&node)?;
                        language = Some(
                            LanguageTag::new_unchecked(&node_value[1..])
                                .with_source_span(node.into()),
                        );
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [NODE_KIND_QUOTED_STRING, NODE_KIND_LANGUAGE_TAG,]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
        return Ok(LanguageString::new(&value, language).with_source_span(root_node.into()));
    }
    unreachable!()
}

fn parse_binary<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Binary, Error> {
    rule_fn!("binary", cursor.node());
    let mut result: Vec<u8> = Default::default();

    for node in cursor
        .node()
        .children_by_field_name(FIELD_NAME_BYTE, cursor)
    {
        context.check_if_error(&node, RULE_NAME)?;
        let value = context.node_source(&node)?;
        let value = u8::from_str_radix(value, 16).expect("Invalid value for Byte");
        result.push(value);
    }

    Ok(result.into())
}

fn parse_value_constructor<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ValueConstructor, Error> {
    let node = cursor.node();
    rule_fn!("value_constructor", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier_reference(context, &mut child.walk())?;

    let child = node.child_by_field_name(FIELD_NAME_VALUE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let value = parse_simple_value(context, &mut child.walk())?;

    Ok(ValueConstructor::new(name, value).with_source_span(node.into()))
}

fn parse_mapping_value<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<MappingValue, Error> {
    let node = cursor.node();
    rule_fn!("mapping_value", node);

    let child = node.child_by_field_name(FIELD_NAME_DOMAIN).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let domain = parse_simple_value(context, &mut child.walk())?;

    let child = node.child_by_field_name(FIELD_NAME_RANGE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let range = parse_value(context, &mut child.walk())?;

    Ok(MappingValue::new(domain, range).with_source_span(node.into()))
}

fn parse_sequence_of_values<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<SequenceOfValues, Error> {
    rule_fn!("sequence_of_values", cursor.node());
    let mut sequence = SequenceOfValues::default().with_source_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_SIMPLE_VALUE => {
                        sequence.push(parse_simple_value(context, &mut node.walk())?);
                    }
                    NODE_KIND_VALUE_CONSTRUCTOR => {
                        sequence.push(parse_value_constructor(context, cursor)?);
                    }
                    NODE_KIND_IDENTIFIER_REFERENCE => {
                        sequence.push(parse_identifier_reference(context, cursor)?);
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_SIMPLE_VALUE,
                                NODE_KIND_VALUE_CONSTRUCTOR,
                                NODE_KIND_IDENTIFIER_REFERENCE,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(sequence)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
