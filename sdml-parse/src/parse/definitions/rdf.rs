use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::annotations::{AnnotationProperty, HasAnnotations};
use sdml_core::model::definitions::RdfDef;
use sdml_core::model::identifiers::{Identifier, QualifiedIdentifier};
use sdml_core::model::{HasBody, HasSourceSpan};
use sdml_core::stdlib;
use sdml_core::syntax::{FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_TYPE, FIELD_NAME_TYPES};
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

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let mut rdf_def =
        RdfDef::individual(parse_identifier(context, &child)?).with_source_span(node.into());

    let child = node.child_by_field_name(FIELD_NAME_BODY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    rdf_def.set_body(parse_annotation_only_body(context, &mut child.walk())?);

    if let Some(types) = node.child_by_field_name(FIELD_NAME_TYPES) {
        context.check_if_error(&types, RULE_NAME)?;
        for child in types.children_by_field_name(FIELD_NAME_TYPE, &mut types.walk()) {
            context.check_if_error(&child, RULE_NAME)?;
            let rdf_type = QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdf::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdf::PROP_TYPE_NAME),
            );
            rdf_def
                .body_mut()
                .add_to_annotations(AnnotationProperty::new(
                    rdf_type.into(),
                    parse_identifier_reference(context, &mut child.walk())?.into(),
                ))
        }
    }

    Ok(rdf_def)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
