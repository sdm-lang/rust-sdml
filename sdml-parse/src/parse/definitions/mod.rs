use super::ParseContext;
use crate::parse::annotations::parse_annotation;
use crate::parse::parse_comment;
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::annotations::{AnnotationOnlyBody, HasAnnotations};
use sdml_core::model::definitions::Definition;
use sdml_core::model::{HasName, HasSourceSpan};
use sdml_core::syntax::{
    NODE_KIND_ANNOTATION, NODE_KIND_DATA_TYPE_DEF, NODE_KIND_DIMENSION_DEF, NODE_KIND_ENTITY_DEF,
    NODE_KIND_ENUM_DEF, NODE_KIND_EVENT_DEF, NODE_KIND_LINE_COMMENT, NODE_KIND_PROPERTY_DEF,
    NODE_KIND_RDF_DEF, NODE_KIND_STRUCTURE_DEF, NODE_KIND_TYPE_CLASS_DEF, NODE_KIND_UNION_DEF,
};
use sdml_errors::diagnostics::functions::library_definition_not_allowed;
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_definition<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Definition, Error> {
    rule_fn!("definition", cursor.node());

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        if node.is_named() {
            match node.kind() {
                NODE_KIND_DATA_TYPE_DEF => {
                    return Ok(parse_data_type_def(context, &mut node.walk())?.into());
                }
                NODE_KIND_DIMENSION_DEF => {
                    return Ok(parse_dimension_def(context, &mut node.walk())?.into());
                }
                NODE_KIND_ENTITY_DEF => {
                    return Ok(parse_entity_def(context, &mut node.walk())?.into());
                }
                NODE_KIND_ENUM_DEF => {
                    return Ok(parse_enum_def(context, &mut node.walk())?.into());
                }
                NODE_KIND_EVENT_DEF => {
                    return Ok(parse_event_def(context, &mut node.walk())?.into());
                }
                NODE_KIND_PROPERTY_DEF => {
                    return Ok(parse_property_def(context, &mut node.walk())?.into());
                }
                NODE_KIND_STRUCTURE_DEF => {
                    return Ok(parse_structure_def(context, &mut node.walk())?.into());
                }
                NODE_KIND_UNION_DEF => {
                    return Ok(parse_union_def(context, &mut node.walk())?.into());
                }
                NODE_KIND_RDF_DEF => {
                    let rdf_def = parse_rdf_def(context, &mut node.walk())?;
                    return if context.is_library {
                        Ok(rdf_def.into())
                    } else {
                        Err(library_definition_not_allowed(
                            context.file_id,
                            rdf_def.source_span().map(|s| s.into()),
                            rdf_def.name(),
                        )
                        .into())
                    };
                }
                NODE_KIND_TYPE_CLASS_DEF => {
                    let type_class = parse_type_class_def(context, &mut node.walk())?;
                    return if context.is_library {
                        Ok(type_class.into())
                    } else {
                        Err(library_definition_not_allowed(
                            context.file_id,
                            type_class.source_span().map(|s| s.into()),
                            type_class.name(),
                        )
                        .into())
                    };
                }
                NODE_KIND_LINE_COMMENT => {
                    let comment = parse_comment(context, &node)?;
                    context.push_comment(comment);
                }
                _ => {
                    unexpected_node!(
                        context,
                        RULE_NAME,
                        node,
                        [
                            NODE_KIND_DATA_TYPE_DEF,
                            NODE_KIND_ENTITY_DEF,
                            NODE_KIND_ENUM_DEF,
                            NODE_KIND_EVENT_DEF,
                            NODE_KIND_STRUCTURE_DEF,
                            NODE_KIND_UNION_DEF,
                        ]
                    );
                }
            }
        }
    }
    rule_unreachable!(RULE_NAME, cursor);
}

pub(crate) fn parse_annotation_only_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<AnnotationOnlyBody, Error> {
    rule_fn!("annotation_only_body", cursor.node());
    let mut body = AnnotationOnlyBody::default().with_source_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {
                        let comment = parse_comment(context, &node)?;
                        context.push_comment(comment);
                    }
                    _ => {
                        unexpected_node!(context, RULE_NAME, node, NODE_KIND_ANNOTATION);
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod classes;
use classes::parse_type_class_def;

mod datatypes;
use datatypes::parse_data_type_def;

mod dimensions;
use dimensions::parse_dimension_def;

mod entities;
use entities::parse_entity_def;

mod enums;
use enums::parse_enum_def;

mod events;
use events::parse_event_def;

mod properties;
use properties::parse_property_def;

mod rdf;
use rdf::parse_rdf_def;

mod structures;
use structures::parse_structure_def;

mod unions;
use unions::parse_union_def;
