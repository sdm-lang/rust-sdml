use crate::parse::{
    annotations::parse_annotation,
    identifiers::{parse_identifier, parse_identifier_reference},
    parse_comment, ParseContext,
};
use sdml_core::{
    error::Error,
    load::ModuleLoader as ModuleLoaderTrait,
    model::{
        annotations::{AnnotationOnlyBody, HasAnnotations},
        definitions::{Definition, FromDefinition},
        HasSourceSpan,
    },
    syntax::{
        FIELD_NAME_FROM, FIELD_NAME_MEMBER, NODE_KIND_ANNOTATION, NODE_KIND_DATA_TYPE_DEF,
        NODE_KIND_DIMENSION_DEF, NODE_KIND_ENTITY_DEF, NODE_KIND_ENUM_DEF, NODE_KIND_EVENT_DEF,
        NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_LINE_COMMENT, NODE_KIND_PROPERTY_DEF,
        NODE_KIND_RDF_DEF, NODE_KIND_STRUCTURE_DEF, NODE_KIND_TYPE_CLASS_DEF, NODE_KIND_UNION_DEF,
    },
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_definition<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Definition, Error> {
    let node = cursor.node();
    rule_fn!("definition", node);

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
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
                return Ok(parse_rdf_def(context, &mut node.walk())?.into());
            }
            NODE_KIND_TYPE_CLASS_DEF => {
                return Ok(parse_type_class_def(context, &mut node.walk())?.into());
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
    rule_unreachable!(RULE_NAME, cursor);
}

pub(crate) fn parse_annotation_only_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<AnnotationOnlyBody, Error> {
    let node = cursor.node();
    rule_fn!("annotation_only_body", node);

    let mut body = AnnotationOnlyBody::default().with_source_span(cursor.node().into());

    for node in cursor.node().named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
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
    Ok(body)
}

pub(crate) fn parse_from_definition_clause<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<FromDefinition, Error> {
    let node = cursor.node();
    rule_fn!("from_definition_clause", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_FROM,
        NODE_KIND_IDENTIFIER_REFERENCE
    );
    let defn_id = parse_identifier_reference(context, &mut child.walk())?;
    let mut from = FromDefinition::from(defn_id);

    // Note:
    // if the grammar matches "_", the wildcard, then there will be no
    // members, this loop does nothing and the result is correct.
    for child in node.children_by_field_name(FIELD_NAME_MEMBER, cursor) {
        check_node!(context, RULE_NAME, &node);
        from.add_to_members(parse_identifier(context, &child)?);
    }

    Ok(from)
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
