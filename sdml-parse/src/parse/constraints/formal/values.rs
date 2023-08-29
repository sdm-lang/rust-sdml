use crate::parse::identifiers::parse_identifier_reference;
use crate::parse::values::parse_simple_value;
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::constraints::{PredicateValue, SequenceOfPredicateValues};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_ELEMENT, NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_LINE_COMMENT,
    NODE_KIND_SEQUENCE_OF_PREDICATE_VALUES, NODE_KIND_SIMPLE_VALUE,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_predicate_value<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<PredicateValue, Error> {
    rule_fn!("predicate_value", cursor.node());

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_SIMPLE_VALUE => {
                return Ok(parse_simple_value(context, &mut node.walk())?.into());
            }
            NODE_KIND_SEQUENCE_OF_PREDICATE_VALUES => {
                return Ok(parse_sequence_of_predicate_values(context, &mut node.walk())?.into());
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [
                        NODE_KIND_SIMPLE_VALUE,
                        NODE_KIND_SEQUENCE_OF_PREDICATE_VALUES,
                    ]
                );
            }
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}

fn parse_sequence_of_predicate_values<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<SequenceOfPredicateValues, Error> {
    rule_fn!("sequence_of_predicate_values", cursor.node());

    let mut sequence = SequenceOfPredicateValues::default().with_source_span(cursor.node().into());

    for value in cursor
        .node()
        .children_by_field_name(FIELD_NAME_ELEMENT, cursor)
    {
        context.check_if_error(&value, RULE_NAME)?;
        match value.kind() {
            NODE_KIND_SIMPLE_VALUE => {
                sequence.push(parse_simple_value(context, &mut value.walk())?);
            }
            NODE_KIND_IDENTIFIER_REFERENCE => {
                sequence.push(parse_identifier_reference(context, &mut value.walk())?);
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    value,
                    [NODE_KIND_SIMPLE_VALUE, NODE_KIND_IDENTIFIER_REFERENCE,]
                );
            }
        }
    }
    Ok(sequence)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
