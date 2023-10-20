use crate::parse::constraints::{parse_predicate_value, parse_sequence_builder};
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::constraints::{FunctionComposition, FunctionalTerm, Subject, Term};
use sdml_core::model::identifiers::Identifier;
use sdml_core::syntax::{
    FIELD_NAME_ARGUMENT, FIELD_NAME_FUNCTION, FIELD_NAME_NAME, FIELD_NAME_SUBJECT,
    NODE_KIND_FUNCTIONAL_TERM, NODE_KIND_FUNCTION_COMPOSITION, NODE_KIND_IDENTIFIER,
    NODE_KIND_LINE_COMMENT, NODE_KIND_PREDICATE_VALUE,
    NODE_KIND_RESERVED_SELF, NODE_KIND_SEQUENCE_BUILDER, NODE_KIND_IDENTIFIER_REFERENCE,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_term<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Term, Error> {
    rule_fn!("term", cursor.node());

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_SEQUENCE_BUILDER => {
                return Ok(parse_sequence_builder(context, &mut node.walk())?.into());
            }
            NODE_KIND_FUNCTIONAL_TERM => {
                return Ok(parse_functional_term(context, &mut node.walk())?.into());
            }
            NODE_KIND_FUNCTION_COMPOSITION => {
                return Ok(parse_function_composition(context, &mut node.walk())?.into());
            }
            NODE_KIND_IDENTIFIER_REFERENCE => {
                return Ok(parse_identifier_reference(context, &mut node.walk())?.into());
            }
            NODE_KIND_RESERVED_SELF => {
                return Ok(Term::ReservedSelf);
            }
            NODE_KIND_PREDICATE_VALUE => {
                return Ok(parse_predicate_value(context, &mut node.walk())?.into());
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [
                        NODE_KIND_FUNCTION_COMPOSITION,
                        NODE_KIND_FUNCTIONAL_TERM,
                        NODE_KIND_IDENTIFIER_REFERENCE,
                        NODE_KIND_PREDICATE_VALUE,
                        NODE_KIND_RESERVED_SELF,
                        NODE_KIND_SEQUENCE_BUILDER,
                    ]
                );
            }
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}

pub(crate) fn parse_function_composition<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FunctionComposition, Error> {
    let node = cursor.node();
    rule_fn!("function_composition", node);

    let child = node.child_by_field_name(FIELD_NAME_SUBJECT).unwrap();
    context.check_if_error(&child, RULE_NAME)?;

    let subject: Subject = match child.kind() {
        NODE_KIND_RESERVED_SELF => Subject::ReservedSelf,
        NODE_KIND_IDENTIFIER => Subject::Identifier(parse_identifier(context, &child)?),
        _ => {
            unexpected_node!(
                context,
                RULE_NAME,
                child,
                [NODE_KIND_RESERVED_SELF, NODE_KIND_IDENTIFIER,]
            );
        }
    };

    let names = {
        let mut names: Vec<Identifier> = Default::default();
        for name in node.children_by_field_name(FIELD_NAME_NAME, cursor) {
            names.push(parse_identifier(context, &name)?);
        }
        names
    };

    Ok(FunctionComposition::new(subject, names))
}

fn parse_functional_term<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FunctionalTerm, Error> {
    let node = cursor.node();
    rule_fn!("functional_term", node);

    let child = node.child_by_field_name(FIELD_NAME_FUNCTION).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let function = parse_term(context, &mut child.walk())?;

    let arguments = {
        let mut arguments: Vec<Term> = Default::default();
        for argument in node.children_by_field_name(FIELD_NAME_ARGUMENT, cursor) {
            arguments.push(parse_term(context, &mut argument.walk())?);
        }
        arguments
    };

    Ok(FunctionalTerm::new_with_arguments(function, arguments))
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
