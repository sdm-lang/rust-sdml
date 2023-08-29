use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::definitions::DatatypeDef;
use sdml_core::model::identifiers::{Identifier, IdentifierReference, QualifiedIdentifier};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BASE, FIELD_NAME_BODY, FIELD_NAME_NAME, NAME_SDML, NODE_KIND_BUILTIN_SIMPLE_TYPE,
    NODE_KIND_IDENTIFIER_REFERENCE,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_data_type_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<DatatypeDef, Error> {
    let node = cursor.node();
    rule_fn!("data_type_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_BASE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let base_type = match child.kind() {
        NODE_KIND_IDENTIFIER_REFERENCE => parse_identifier_reference(context, &mut child.walk())?,
        NODE_KIND_BUILTIN_SIMPLE_TYPE => {
            let module = Identifier::new_unchecked(NAME_SDML);
            let member = parse_identifier(context, &child)?.with_source_span(child.into());
            IdentifierReference::QualifiedIdentifier(QualifiedIdentifier::new(module, member))
        }
        _ => {
            rule_unreachable!(RULE_NAME, cursor);
        }
    };

    context.start_type(&name)?;
    let mut data_type = DatatypeDef::new(name, base_type).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_annotation_only_body(context, &mut child.walk())?;
        data_type.set_body(body);
    }

    context.end_type();
    Ok(data_type)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
