use crate::parse::identifiers::parse_identifier;
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::constraints::Constraint;
use sdml_core::syntax::{
    FIELD_NAME_NAME, NODE_KIND_FORMAL_CONSTRAINT, NODE_KIND_IDENTIFIER,
    NODE_KIND_INFORMAL_CONSTRAINT, NODE_KIND_LINE_COMMENT,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_constraint<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Constraint, Error> {
    rule_fn!("constraint", cursor.node());

    let child = cursor.node().child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let constraint_name = parse_identifier(context, &child)?;

    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_IDENTIFIER => {
                        // ignore: this is the name field above
                    }
                    NODE_KIND_INFORMAL_CONSTRAINT => {
                        return Ok(Constraint::new(
                            constraint_name,
                            parse_informal_constraint(context, &mut node.walk())?,
                        ))
                    }
                    NODE_KIND_FORMAL_CONSTRAINT => {
                        return Ok(Constraint::new(
                            constraint_name,
                            parse_formal_constraint(context, &mut node.walk())?,
                        ))
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_IDENTIFIER,
                                NODE_KIND_INFORMAL_CONSTRAINT,
                                NODE_KIND_FORMAL_CONSTRAINT,
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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod informal;
pub(crate) use informal::parse_informal_constraint;

mod formal;
pub(crate) use formal::{parse_formal_constraint, parse_predicate_value, parse_sequence_builder, parse_function_cardinality_expression, parse_function_signature, parse_constraint_sentence};
