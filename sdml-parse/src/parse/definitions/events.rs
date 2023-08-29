use sdml_core::error::Error;
use sdml_core::model::definitions::EventDef;
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_SOURCE};
use tree_sitter::TreeCursor;

use crate::parse::definitions::structures::parse_structure_body;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::ParseContext;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_event_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EventDef, Error> {
    let node = cursor.node();
    rule_fn!("event_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_SOURCE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let event_source = parse_identifier_reference(context, &mut child.walk())?;

    context.start_type(&name)?;
    let mut event = EventDef::new(name, event_source).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_structure_body(context, &mut child.walk())?;
        event.set_body(body);
    }

    context.end_type();
    Ok(event)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
