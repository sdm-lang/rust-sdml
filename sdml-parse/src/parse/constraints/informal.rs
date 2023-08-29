use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::constraints::{ControlledLanguageString, ControlledLanguageTag};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{FIELD_NAME_LANGUAGE, FIELD_NAME_VALUE};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_informal_constraint<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<ControlledLanguageString, Error> {
    let node = cursor.node();
    rule_fn!("informal_constraint", node);

    let child = node.child_by_field_name(FIELD_NAME_VALUE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let node_value = context.node_source(&node)?;
    let value = node_value[1..(node_value.len() - 1)].to_string();

    let constraint = if let Some(child) = node.child_by_field_name(FIELD_NAME_LANGUAGE) {
        context.check_if_error(&child, RULE_NAME)?;
        let node_value = context.node_source(&node)?;
        let language = ControlledLanguageTag::new_unchecked(node_value);
        ControlledLanguageString::new(value, language)
    } else {
        ControlledLanguageString::from(value)
    }
    .with_source_span(node.into());

    Ok(constraint)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
