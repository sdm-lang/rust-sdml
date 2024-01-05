use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::parse_identifier;
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::definitions::{RdfDef, RdfDefBody};
use sdml_core::model::{HasBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_TYPE, NODE_KIND_RDF_TYPE_CLASS,
    NODE_KIND_RDF_TYPE_PROPERTY,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_rdf_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<RdfDef, Error> {
    let node = cursor.node();
    rule_fn!("rdf_def", node);

    let child = node.child_by_field_name(FIELD_NAME_TYPE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let rdf_type = context.node_source(&child)?.to_string();

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let mut rdf_def =
        RdfDefBody::new(parse_identifier(context, &child)?).with_source_span(node.into());

    let child = node.child_by_field_name(FIELD_NAME_BODY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    rdf_def.set_body(parse_annotation_only_body(context, &mut child.walk())?);

    match rdf_type.as_str() {
        NODE_KIND_RDF_TYPE_CLASS => Ok(RdfDef::Class(rdf_def)),
        NODE_KIND_RDF_TYPE_PROPERTY => Ok(RdfDef::Property(rdf_def)),
        _ => {
            panic!(); // rule_unreachable!(RULE_NAME, cursor);
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
