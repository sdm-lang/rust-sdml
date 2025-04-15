use crate::parse::constraints::formal::functions::parse_function_def;
use crate::parse::ParseContext;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::constraints::{FormalConstraint, FunctionDef};
use sdml_core::syntax::{
    NODE_KIND_CONSTRAINT_ENVIRONMENT, NODE_KIND_CONSTRAINT_SENTENCE, NODE_KIND_FUNCTION_DEF,
    NODE_KIND_LINE_COMMENT,
};
use sdml_errors::Error;
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_formal_constraint<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FormalConstraint, Error> {
    rule_fn!("parse_formal_constraint", cursor.node());

    let mut environment: Vec<FunctionDef> = Default::default();

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_CONSTRAINT_ENVIRONMENT => {
                environment = parse_constraint_environment(context, &mut node.walk())?;
            }
            NODE_KIND_CONSTRAINT_SENTENCE => {
                let body = parse_constraint_sentence(context, &mut node.walk())?;
                return Ok(FormalConstraint::new(body).with_environment(environment));
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [
                        NODE_KIND_CONSTRAINT_ENVIRONMENT,
                        NODE_KIND_CONSTRAINT_SENTENCE,
                    ]
                );
            }
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}

pub(crate) fn parse_constraint_environment<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Vec<FunctionDef>, Error> {
    rule_fn!("constraint_environment", cursor.node());

    let mut environment: Vec<FunctionDef> = Default::default();

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_FUNCTION_DEF => {
                environment.push(parse_function_def(context, &mut node.walk())?);
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(context, RULE_NAME, node, [NODE_KIND_FUNCTION_DEF,]);
            }
        }
    }
    Ok(environment)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod functions;
pub(crate) use functions::{
    parse_function_body, parse_function_cardinality_expression, parse_function_signature,
};

mod sentences;
pub(crate) use sentences::{parse_constraint_sentence, parse_quantified_sentence, parse_variable};

mod sequences;
pub(crate) use sequences::parse_sequence_builder;

mod terms;

mod values;
pub(crate) use values::parse_predicate_value;
