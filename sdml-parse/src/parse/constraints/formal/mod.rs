use crate::parse::constraints::formal::environments::parse_constraint_environment;
use crate::parse::constraints::formal::sentences::parse_constraint_sentence;
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::constraints::{EnvironmentDef, FormalConstraint};
use sdml_core::syntax::{
    NODE_KIND_CONSTRAINT_ENVIRONMENT, NODE_KIND_CONSTRAINT_SENTENCE, NODE_KIND_LINE_COMMENT,
};
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

mod sentences;
pub(crate) use sentences::parse_quantified_variable_binding;

mod sequences;
pub(crate) use sequences::parse_sequence_builder;

mod terms;
pub(crate) use terms::parse_function_composition;

mod values;
pub(crate) use values::parse_predicate_value;
