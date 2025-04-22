use crate::parse::{
    constraints::parse_term,
    identifiers::{parse_identifier, parse_identifier_reference},
    ParseContext,
};
use sdml_core::{
    error::Error,
    load::ModuleLoader as ModuleLoaderTrait,
    model::constraints::{
        AtomicSentence, BinaryBooleanSentence, BooleanSentence, ConstraintSentence, Equation,
        InequalityRelation, Inequation, QuantifiedSentence, QuantifiedVariable,
        QuantifiedVariableBinding, Quantifier, SimpleSentence, Term, UnaryBooleanSentence,
        Variable,
    },
    syntax::{
        FIELD_NAME_ARGUMENT, FIELD_NAME_BINDING, FIELD_NAME_BODY, FIELD_NAME_LHS, FIELD_NAME_NAME,
        FIELD_NAME_OPERATOR, FIELD_NAME_PREDICATE, FIELD_NAME_QUANTIFIER, FIELD_NAME_RANGE,
        FIELD_NAME_RELATION, FIELD_NAME_RHS, FIELD_NAME_SOURCE, FIELD_NAME_VARIABLE,
        NODE_KIND_ATOMIC_SENTENCE, NODE_KIND_BINARY_BOOLEAN_SENTENCE, NODE_KIND_BOOLEAN_SENTENCE,
        NODE_KIND_CONSTRAINT_SENTENCE, NODE_KIND_EQUATION, NODE_KIND_INEQUATION,
        NODE_KIND_LINE_COMMENT, NODE_KIND_LOGICAL_OP_BICONDITIONAL,
        NODE_KIND_LOGICAL_OP_CONJUNCTION, NODE_KIND_LOGICAL_OP_DISJUNCTION,
        NODE_KIND_LOGICAL_OP_EXCLUSIVE_DISJUNCTION, NODE_KIND_LOGICAL_OP_IMPLICATION,
        NODE_KIND_LOGICAL_OP_NEGATION, NODE_KIND_QUANTIFIED_SENTENCE,
        NODE_KIND_QUANTIFIED_VARIABLE_BINDING, NODE_KIND_SIMPLE_SENTENCE, NODE_KIND_TERM,
        NODE_KIND_UNARY_BOOLEAN_SENTENCE,
    },
};
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

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_OPERATOR);
    assert!(child.kind() == NODE_KIND_LOGICAL_OP_NEGATION);

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_RHS);
    let rhs = parse_constraint_sentence(context, &mut child.walk())?;

    Ok(UnaryBooleanSentence::new(rhs))
}

fn parse_binary_boolean_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<BinaryBooleanSentence, Error> {
    let node = cursor.node();
    rule_fn!("binary_boolean_sentence", node);

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_LHS);
    let lhs = parse_constraint_sentence(context, &mut child.walk())?;

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_OPERATOR);
    let relation_kind = child.kind().to_string();

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_RHS);
    let rhs = parse_constraint_sentence(context, &mut child.walk())?;

    match relation_kind.as_str() {
        NODE_KIND_LOGICAL_OP_CONJUNCTION => Ok(BinaryBooleanSentence::and(lhs, rhs)),
        NODE_KIND_LOGICAL_OP_DISJUNCTION => Ok(BinaryBooleanSentence::or(lhs, rhs)),
        NODE_KIND_LOGICAL_OP_EXCLUSIVE_DISJUNCTION => Ok(BinaryBooleanSentence::xor(lhs, rhs)),
        NODE_KIND_LOGICAL_OP_IMPLICATION => Ok(BinaryBooleanSentence::implies(lhs, rhs)),
        NODE_KIND_LOGICAL_OP_BICONDITIONAL => Ok(BinaryBooleanSentence::iff(lhs, rhs)),
        _ => {
            unexpected_node!(
                context,
                RULE_NAME,
                node,
                [
                    NODE_KIND_LOGICAL_OP_CONJUNCTION,
                    NODE_KIND_LOGICAL_OP_DISJUNCTION,
                    NODE_KIND_LOGICAL_OP_EXCLUSIVE_DISJUNCTION,
                    NODE_KIND_LOGICAL_OP_IMPLICATION,
                    NODE_KIND_LOGICAL_OP_BICONDITIONAL,
                ]
            );
        }
    }
}

fn parse_atomic_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<AtomicSentence, Error> {
    let node = cursor.node();
    rule_fn!("atomic_sentence", node);

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_PREDICATE);
    let predicate = parse_term(context, &mut child.walk())?;

    let mut arguments: Vec<Term> = Default::default();
    for argument in node.children_by_field_name(FIELD_NAME_ARGUMENT, &mut node.walk()) {
        arguments.push(parse_term(context, &mut argument.walk())?);
    }

    Ok(AtomicSentence::new_with_arguments(predicate, arguments))
}

fn parse_equation<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Equation, Error> {
    let node = cursor.node();
    rule_fn!("equation", node);

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_LHS, NODE_KIND_TERM);
    let lhs = parse_term(context, &mut child.walk())?;

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_RHS, NODE_KIND_TERM);
    let rhs = parse_term(context, &mut child.walk())?;

    Ok(Equation::new(lhs, rhs))
}

fn parse_inequation<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Inequation, Error> {
    let node = cursor.node();
    rule_fn!("inequation", node);

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_LHS, NODE_KIND_TERM);
    let lhs = parse_term(context, &mut child.walk())?;

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_RELATION);
    let relation = InequalityRelation::from_str(context.node_source(&child)?)?;

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_RHS, NODE_KIND_TERM);
    let rhs = parse_term(context, &mut child.walk())?;

    Ok(Inequation::new(lhs, relation, rhs))
}

pub(crate) fn parse_quantified_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<QuantifiedSentence, Error> {
    let node = cursor.node();
    rule_fn!("quantified_sentence", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BINDING,
        NODE_KIND_QUANTIFIED_VARIABLE_BINDING
    );
    let binding = parse_quantified_variable_binding(context, &mut child.walk())?;

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_BODY);
    let body = parse_constraint_sentence(context, &mut child.walk())?;

    Ok(QuantifiedSentence::new(binding, body))
}

pub(crate) fn parse_quantified_variable_binding<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<QuantifiedVariableBinding, Error> {
    let node = cursor.node();
    rule_fn!("quantified_variable_binding", cursor.node());

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_QUANTIFIER);
    let quantifier = Quantifier::from_str(context.node_source(&child)?)?;

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_BINDING);
    let binding = parse_quantified_variable(context, &mut child.walk())?;

    Ok(QuantifiedVariableBinding::new(quantifier, binding))
}

fn parse_quantified_variable<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<QuantifiedVariable, Error> {
    let node = cursor.node();
    rule_fn!("quantified_variable", cursor.node());

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_VARIABLE);
    let variable = parse_variable(context, &mut child.walk())?;

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_SOURCE);
    let source = parse_term(context, &mut child.walk())?;

    Ok(QuantifiedVariable::new(variable, source))
}

pub(crate) fn parse_variable<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Variable, Error> {
    let node = cursor.node();
    rule_fn!("variable", cursor.node());

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_NAME);
    let variable = Variable::new(parse_identifier(context, &child)?);

    if let Some(range) = optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_RANGE) {
        let range = parse_identifier_reference(context, &mut range.walk())?;
        Ok(variable.with_range(range))
    } else {
        Ok(variable)
    }
}
