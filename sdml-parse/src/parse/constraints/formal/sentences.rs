use super::terms::parse_term;
use crate::parse::constraints::formal::parse_function_composition;
use crate::parse::constraints::parse_sequence_builder;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::constraints::{
    AtomicSentence, BinaryBooleanSentence, BooleanSentence, ConstraintSentence, Equation,
    InequalityRelation, Inequation, IteratorSource, QuantifiedBinding, QuantifiedSentence,
    QuantifiedVariableBinding, Quantifier, QuantifierBoundNames, SequenceIterator, SimpleSentence,
    Term, TypeIterator, UnaryBooleanSentence,
};
use sdml_core::model::identifiers::Identifier;
use sdml_core::syntax::{
    FIELD_NAME_ARGUMENT, FIELD_NAME_BINDING, FIELD_NAME_BODY, FIELD_NAME_LHS, FIELD_NAME_NAME,
    FIELD_NAME_OPERATOR, FIELD_NAME_PREDICATE, FIELD_NAME_QUANTIFIER, FIELD_NAME_RELATION,
    FIELD_NAME_RHS, FIELD_NAME_SOURCE, NODE_KIND_ATOMIC_SENTENCE, NODE_KIND_BICONDITIONAL,
    NODE_KIND_BINARY_BOOLEAN_SENTENCE, NODE_KIND_BOOLEAN_SENTENCE, NODE_KIND_CONJUNCTION,
    NODE_KIND_DISJUNCTION, NODE_KIND_EQUATION, NODE_KIND_EXCLUSIVE_DISJUNCTION,
    NODE_KIND_FUNCTION_COMPOSITION, NODE_KIND_IDENTIFIER, NODE_KIND_IDENTIFIER_REFERENCE,
    NODE_KIND_IMPLICATION, NODE_KIND_INEQUATION, NODE_KIND_LINE_COMMENT, NODE_KIND_NEGATION,
    NODE_KIND_QUANTIFIED_SENTENCE, NODE_KIND_RESERVED_SELF, NODE_KIND_RESERVED_SELF_TYPE,
    NODE_KIND_SEQUENCE_BUILDER, NODE_KIND_SEQUENCE_ITERATOR, NODE_KIND_SIMPLE_SENTENCE,
    NODE_KIND_TYPE_ITERATOR, NODE_KIND_UNARY_BOOLEAN_SENTENCE,
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

    let child = node.child_by_field_name(FIELD_NAME_OPERATOR).unwrap();
    assert!(child.kind() == NODE_KIND_NEGATION);

    let child = node.child_by_field_name(FIELD_NAME_RHS).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let rhs = parse_constraint_sentence(context, &mut child.walk())?;

    Ok(UnaryBooleanSentence::new(rhs))
}

fn parse_binary_boolean_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<BinaryBooleanSentence, Error> {
    let node = cursor.node();
    rule_fn!("binary_boolean_sentence", node);

    let child = node.child_by_field_name(FIELD_NAME_LHS).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let lhs = parse_constraint_sentence(context, &mut child.walk())?;

    let child = node.child_by_field_name(FIELD_NAME_OPERATOR).unwrap();
    let relation_kind = child.kind().to_string();

    let child = node.child_by_field_name(FIELD_NAME_RHS).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
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

fn parse_quantified_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<QuantifiedSentence, Error> {
    let node = cursor.node();
    rule_fn!("quantified_sentence", node);

    let bindings = {
        let mut bindings: Vec<QuantifiedVariableBinding> = Default::default();
        for binding in node.children_by_field_name(FIELD_NAME_BINDING, cursor) {
            bindings.push(parse_quantified_variable_binding(
                context,
                &mut binding.walk(),
            )?);
        }
        bindings
    };

    let child = node.child_by_field_name(FIELD_NAME_BODY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let body = parse_constraint_sentence(context, &mut child.next_named_sibling().unwrap().walk())?;

    Ok(QuantifiedSentence::new(bindings, body))
}

fn parse_atomic_sentence<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<AtomicSentence, Error> {
    let node = cursor.node();
    rule_fn!("atomic_sentence", node);

    let child = node.child_by_field_name(FIELD_NAME_PREDICATE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
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

    let child = node.child_by_field_name(FIELD_NAME_LHS).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let lhs = parse_term(context, &mut child.walk())?;

    let child = node.child_by_field_name(FIELD_NAME_RHS).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let rhs = parse_term(context, &mut child.walk())?;

    Ok(Equation::new(lhs, rhs))
}

fn parse_inequation<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Inequation, Error> {
    let node = cursor.node();
    rule_fn!("inequation", node);

    let child = node.child_by_field_name(FIELD_NAME_LHS).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let lhs = parse_term(context, &mut child.walk())?;

    let child = node.child_by_field_name(FIELD_NAME_RELATION).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let relation = InequalityRelation::from_str(context.node_source(&child)?)?;

    let child = node.child_by_field_name(FIELD_NAME_RHS).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
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

    let bindings = {
        let mut bindings: Vec<QuantifiedBinding> = Default::default();
        for binding in node.children_by_field_name(FIELD_NAME_BINDING, cursor) {
            bindings.push(parse_quantifier_bound_names(context, &mut binding.walk())?);
        }
        bindings
    };

    Ok(QuantifiedVariableBinding::new(quantifier, bindings))
}

fn parse_quantifier_bound_names<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<QuantifiedBinding, Error> {
    let node = cursor.node();
    rule_fn!("quantifier_bound_names", cursor.node());

    let child = node.child_by_field_name(FIELD_NAME_SOURCE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;

    if child.kind() == NODE_KIND_RESERVED_SELF {
        Ok(QuantifiedBinding::ReservedSelf)
    } else {
        let source: IteratorSource = match child.kind() {
            NODE_KIND_TYPE_ITERATOR => parse_type_iterator(context, &mut child.walk())?.into(),
            NODE_KIND_SEQUENCE_ITERATOR => {
                parse_sequence_iterator(context, &mut child.walk())?.into()
            }
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    child,
                    [
                        NODE_KIND_RESERVED_SELF,
                        NODE_KIND_TYPE_ITERATOR,
                        NODE_KIND_SEQUENCE_ITERATOR,
                    ]
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

        Ok(QuantifiedBinding::Named(QuantifierBoundNames::new(
            names, source,
        )))
    }
}

fn parse_type_iterator<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeIterator, Error> {
    rule_fn!("type_iterator", cursor.node());

    let child = cursor
        .node()
        .child_by_field_name(FIELD_NAME_SOURCE)
        .unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    match child.kind() {
        NODE_KIND_RESERVED_SELF_TYPE => Ok(TypeIterator::SelfType),
        NODE_KIND_IDENTIFIER_REFERENCE => Ok(TypeIterator::Type(parse_identifier_reference(
            context,
            &mut child.walk(),
        )?)),
        _ => {
            unexpected_node!(
                context,
                RULE_NAME,
                child,
                [NODE_KIND_RESERVED_SELF_TYPE, NODE_KIND_IDENTIFIER_REFERENCE,]
            );
        }
    }
}

fn parse_sequence_iterator<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<SequenceIterator, Error> {
    rule_fn!("sequence_iterator", cursor.node());

    let child = cursor
        .node()
        .child_by_field_name(FIELD_NAME_SOURCE)
        .unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    match child.kind() {
        NODE_KIND_FUNCTION_COMPOSITION => Ok(SequenceIterator::Call(parse_function_composition(
            context,
            &mut child.walk(),
        )?)),
        NODE_KIND_IDENTIFIER => Ok(SequenceIterator::Variable(parse_identifier(
            context, &child,
        )?)),
        NODE_KIND_SEQUENCE_BUILDER => Ok(SequenceIterator::Builder(parse_sequence_builder(
            context,
            &mut child.walk(),
        )?)),
        _ => {
            unexpected_node!(
                context,
                RULE_NAME,
                child,
                [
                    NODE_KIND_FUNCTION_COMPOSITION,
                    NODE_KIND_IDENTIFIER,
                    NODE_KIND_SEQUENCE_BUILDER,
                ]
            );
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
