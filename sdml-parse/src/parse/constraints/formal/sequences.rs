use crate::parse::{
    constraints::formal::{parse_quantified_sentence, parse_variable},
    ParseContext,
};
use sdml_core::{
    load::ModuleLoader as ModuleLoaderTrait,
    model::constraints::SequenceBuilder,
    syntax::{FIELD_NAME_BODY, FIELD_NAME_VARIABLE, NODE_KIND_VARIABLE},
};
use sdml_errors::Error;
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_sequence_builder<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<SequenceBuilder, Error> {
    let node = cursor.node();
    rule_fn!("sequence_builder", node);

    let mut variables = Vec::default();
    for variable in node.children_by_field_name(FIELD_NAME_VARIABLE, cursor) {
        check_node!(context, RULE_NAME, node, NODE_KIND_VARIABLE);
        variables.push(parse_variable(context, &mut variable.walk())?);
    }

    let child = node_field_named!(context, RULE_NAME, node, FIELD_NAME_BODY);
    let body = parse_quantified_sentence(context, &mut child.walk())?;

    Ok(SequenceBuilder::new(variables, body))
}
