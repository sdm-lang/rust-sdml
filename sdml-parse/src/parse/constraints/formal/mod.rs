use crate::parse::constraints::formal::environments::parse_constraint_environment;
use crate::parse::ParseContext;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::constraints::{EnvironmentDef, FormalConstraint};
use sdml_core::syntax::{
    NODE_KIND_CONSTRAINT_ENVIRONMENT, NODE_KIND_CONSTRAINT_SENTENCE, NODE_KIND_LINE_COMMENT,
};
use sdml_error::Error;
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_formal_constraint<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FormalConstraint, Error> {
    rule_fn!("parse_formal_constraint", cursor.node());

    let mut environment: Vec<EnvironmentDef> = Default::default();

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod environments;

mod functions;
pub(crate) use functions::{parse_function_cardinality_expression, parse_function_signature};

mod sentences;
pub(crate) use sentences::{parse_constraint_sentence, parse_quantified_sentence};

mod sequences;
pub(crate) use sequences::parse_sequence_builder;

mod terms;

mod values;
pub(crate) use values::parse_predicate_value;
