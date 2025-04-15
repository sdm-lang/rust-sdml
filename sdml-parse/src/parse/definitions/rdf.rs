use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader;
use sdml_core::model::annotations::{AnnotationProperty, HasAnnotations};
use sdml_core::model::definitions::RdfDef;
use sdml_core::model::identifiers::{Identifier, QualifiedIdentifier};
use sdml_core::model::{HasBody, HasSourceSpan};
use sdml_core::stdlib;
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_NAME, FIELD_NAME_TYPE, FIELD_NAME_TYPES,
    NODE_KIND_ANNOTATION_ONLY_BODY, NODE_KIND_IDENTIFIER, NODE_KIND_RDF_TYPES,
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

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let mut rdf_def =
        RdfDef::individual(parse_identifier(context, &child)?).with_source_span(node.into());

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_ANNOTATION_ONLY_BODY
    );
    rdf_def.set_body(parse_annotation_only_body(context, &mut child.walk())?);

    if let Some(types) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_TYPES,
        NODE_KIND_RDF_TYPES
    ) {
        for child in types.children_by_field_name(FIELD_NAME_TYPE, &mut types.walk()) {
            check_node!(context, RULE_NAME, &child);
            let rdf_type = QualifiedIdentifier::new(
                Identifier::new_unchecked(stdlib::rdf::MODULE_NAME),
                Identifier::new_unchecked(stdlib::rdf::TYPE),
            );
            rdf_def
                .body_mut()
                .add_to_annotations(AnnotationProperty::new(
                    rdf_type,
                    parse_identifier_reference(context, &mut child.walk())?,
                ))
        }
    }

    Ok(rdf_def)
}
