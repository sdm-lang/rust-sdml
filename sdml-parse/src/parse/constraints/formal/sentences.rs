use super::terms::parse_term;
use crate::parse::identifiers::parse_identifier;
use crate::parse::ParseContext;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::constraints::{
    AtomicSentence, BinaryBooleanSentence, BooleanSentence, ConstraintSentence, Equation,
    InequalityRelation, Inequation, QuantifiedSentence, QuantifiedVariable,
    QuantifiedVariableBinding, Quantifier, SimpleSentence, Term, UnaryBooleanSentence,
};
use sdml_core::syntax::{
    FIELD_NAME_ARGUMENT, FIELD_NAME_BINDING, FIELD_NAME_BODY, FIELD_NAME_LHS, FIELD_NAME_NAME,
    FIELD_NAME_OPERATOR, FIELD_NAME_PREDICATE, FIELD_NAME_QUANTIFIER, FIELD_NAME_RELATION,
    FIELD_NAME_RHS, FIELD_NAME_SOURCE, NODE_KIND_ATOMIC_SENTENCE, NODE_KIND_BICONDITIONAL,
    NODE_KIND_BINARY_BOOLEAN_SENTENCE, NODE_KIND_BOOLEAN_SENTENCE, NODE_KIND_CONJUNCTION,
    NODE_KIND_CONSTRAINT_SENTENCE, NODE_KIND_DISJUNCTION, NODE_KIND_EQUATION,
    NODE_KIND_EXCLUSIVE_DISJUNCTION, NODE_KIND_IMPLICATION, NODE_KIND_INEQUATION,
    NODE_KIND_LINE_COMMENT, NODE_KIND_NEGATION, NODE_KIND_QUANTIFIED_SENTENCE,
    NODE_KIND_QUANTIFIED_VARIABLE_BINDING, NODE_KIND_RESERVED_SELF, NODE_KIND_SIMPLE_SENTENCE,
    NODE_KIND_TERM, NODE_KIND_UNARY_BOOLEAN_SENTENCE,
};
use sdml_error::Error;
use std::str::FromStr;
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_constraint_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ConstraintSentence, Error> {
    rule_fn!("constraint_sentence", cursor.node());

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_CONSTRAINT_SENTENCE => {
                return parse_constraint_sentence(context, &mut node.walk());
            }
            NODE_KIND_SIMPLE_SENTENCE => {
                return Ok(parse_simple_sentence(context, &mut node.walk())?.into());
            }
            NODE_KIND_BOOLEAN_SENTENCE => {
                return Ok(parse_boolean_sentence(context, &mut node.walk())?.into());
            }
            NODE_KIND_QUANTIFIED_SENTENCE => {
                return Ok(parse_quantified_sentence(context, &mut node.walk())?.into());
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [
                        NODE_KIND_SIMPLE_SENTENCE,
                        NODE_KIND_BOOLEAN_SENTENCE,
                        NODE_KIND_QUANTIFIED_SENTENCE,
                    ]
                );
            }
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}

fn parse_simple_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<SimpleSentence, Error> {
    rule_fn!("simple_sentence", cursor.node());

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_ATOMIC_SENTENCE => {
                return Ok(parse_atomic_sentence(context, &mut node.walk())?.into());
            }
            NODE_KIND_EQUATION => {
                return Ok(parse_equation(context, &mut node.walk())?.into());
            }
            NODE_KIND_INEQUATION => {
                return Ok(parse_inequation(context, &mut node.walk())?.into());
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [NODE_KIND_ATOMIC_SENTENCE, NODE_KIND_EQUATION,]
                );
            }
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}

fn parse_boolean_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<BooleanSentence, Error> {
    let node = cursor.node();
    rule_fn!("boolean_sentence", node);

    for node in node.named_children(cursor) {
        match node.kind() {
            NODE_KIND_UNARY_BOOLEAN_SENTENCE => {
                return Ok(parse_unary_boolean_sentence(context, &mut node.walk())?.into());
            }
            NODE_KIND_BINARY_BOOLEAN_SENTENCE => {
                return Ok(parse_binary_boolean_sentence(context, &mut node.walk())?.into());
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [NODE_KIND_ATOMIC_SENTENCE, NODE_KIND_EQUATION,]
                );
            }
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}

fn parse_unary_boolean_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<UnaryBooleanSentence, Error> {
    let node = cursor.node();
    rule_fn!("unary_boolean_sentence", node);

    let child = node_child_named!(node, FIELD_NAME_OPERATOR, context, RULE_NAME);
    assert!(child.kind() == NODE_KIND_NEGATION);

    let child = node_child_named!(node, FIELD_NAME_RHS, context, RULE_NAME);
    let rhs = parse_constraint_sentence(context, &mut child.walk())?;

    Ok(UnaryBooleanSentence::new(rhs))
}

fn parse_binary_boolean_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<BinaryBooleanSentence, Error> {
    let node = cursor.node();
    rule_fn!("binary_boolean_sentence", node);

    let child = node_child_named!(node, FIELD_NAME_LHS, context, RULE_NAME);
    let lhs = parse_constraint_sentence(context, &mut child.walk())?;

    let child = node_child_named!(node, FIELD_NAME_OPERATOR, context, RULE_NAME);
    let relation_kind = child.kind().to_string();

    let child = node_child_named!(node, FIELD_NAME_RHS, context, RULE_NAME);
    let rhs = parse_constraint_sentence(context, &mut child.walk())?;

    match relation_kind.as_str() {
        NODE_KIND_CONJUNCTION => Ok(BinaryBooleanSentence::and(lhs, rhs)),
        NODE_KIND_DISJUNCTION => Ok(BinaryBooleanSentence::or(lhs, rhs)),
        NODE_KIND_EXCLUSIVE_DISJUNCTION => Ok(BinaryBooleanSentence::xor(lhs, rhs)),
        NODE_KIND_IMPLICATION => Ok(BinaryBooleanSentence::implies(lhs, rhs)),
        NODE_KIND_BICONDITIONAL => Ok(BinaryBooleanSentence::iff(lhs, rhs)),
        _ => {
            unexpected_node!(
                context,
                RULE_NAME,
                node,
                [
                    NODE_KIND_CONJUNCTION,
                    NODE_KIND_DISJUNCTION,
                    NODE_KIND_EXCLUSIVE_DISJUNCTION,
                    NODE_KIND_IMPLICATION,
                    NODE_KIND_BICONDITIONAL,
                ]
            );
        }
    }
}

pub(crate) fn parse_quantified_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<QuantifiedSentence, Error> {
    let node = cursor.node();
    rule_fn!("quantified_sentence", node);

    let child = node_child_named!(
        node,
        FIELD_NAME_BINDING,
        NODE_KIND_QUANTIFIED_VARIABLE_BINDING,
        context,
        RULE_NAME
    );
    let binding = parse_quantified_variable_binding(context, &mut child.walk())?;

    let child = node_child_named!(node, FIELD_NAME_BODY, context, RULE_NAME);
    let body = parse_constraint_sentence(context, &mut child.walk())?;

    Ok(QuantifiedSentence::new(binding, body))
}

fn parse_atomic_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<AtomicSentence, Error> {
    let node = cursor.node();
    rule_fn!("atomic_sentence", node);

    let child = node_child_named!(node, FIELD_NAME_PREDICATE, context, RULE_NAME);
    let predicate = parse_term(context, &mut child.walk())?;

    let arguments = {
        let mut arguments: Vec<Term> = Default::default();
        for argument in node.children_by_field_name(FIELD_NAME_ARGUMENT, cursor) {
            arguments.push(parse_term(context, &mut argument.walk())?);
        }
        arguments
    };

    Ok(AtomicSentence::new_with_arguments(predicate, arguments))
}

fn parse_equation<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Equation, Error> {
    let node = cursor.node();
    rule_fn!("equation", node);

    let child = node_child_named!(node, FIELD_NAME_LHS, NODE_KIND_TERM, context, RULE_NAME);
    let lhs = parse_term(context, &mut child.walk())?;

    let child = node_child_named!(node, FIELD_NAME_RHS, NODE_KIND_TERM, context, RULE_NAME);
    let rhs = parse_term(context, &mut child.walk())?;

    Ok(Equation::new(lhs, rhs))
}

fn parse_inequation<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Inequation, Error> {
    let node = cursor.node();
    rule_fn!("inequation", node);

    let child = node_child_named!(node, FIELD_NAME_LHS, NODE_KIND_TERM, context, RULE_NAME);
    let lhs = parse_term(context, &mut child.walk())?;

    let child = node_child_named!(node, FIELD_NAME_RELATION, context, RULE_NAME);
    let relation = InequalityRelation::from_str(context.node_source(&child)?)?;

    let child = node_child_named!(node, FIELD_NAME_RHS, NODE_KIND_TERM, context, RULE_NAME);
    let rhs = parse_term(context, &mut child.walk())?;

    Ok(Inequation::new(lhs, relation, rhs))
}

pub(crate) fn parse_quantified_variable_binding<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<QuantifiedVariableBinding, Error> {
    let node = cursor.node();
    rule_fn!("quantified_variable_binding", cursor.node());

    let quantifier = if let Some(child) = node.child_by_field_name(FIELD_NAME_QUANTIFIER) {
        context.check_if_error(&child, RULE_NAME)?;
        Quantifier::from_str(context.node_source(&child)?)?
    } else {
        Quantifier::default()
    };

    let child = node_child_named!(node, FIELD_NAME_BINDING, context, RULE_NAME);

    if let Some(binding) = parse_quantified_variable(context, &mut child.walk())? {
        Ok(QuantifiedVariableBinding::new(quantifier, binding))
    } else {
        Ok(QuantifiedVariableBinding::new_self(quantifier))
    }
}

fn parse_quantified_variable<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Option<QuantifiedVariable>, Error> {
    let node = cursor.node();
    rule_fn!("quantified_variable", cursor.node());

    let child = node_child_named!(node, FIELD_NAME_SOURCE, context, RULE_NAME);

    if child.kind() == NODE_KIND_RESERVED_SELF {
        Ok(None)
    } else if child.kind() == NODE_KIND_TERM {
        let source = parse_term(context, &mut child.walk())?;

        let child = node_child_named!(node, FIELD_NAME_NAME, context, RULE_NAME);
        let name = parse_identifier(context, &child)?;

        Ok(Some(QuantifiedVariable::new(name, source)))
    } else {
        unexpected_node!(
            context,
            RULE_NAME,
            node,
            [NODE_KIND_RESERVED_SELF, NODE_KIND_TERM,]
        );
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
