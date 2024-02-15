use crate::parse::constraints::formal::parse_quantified_sentence;
use crate::parse::identifiers::parse_identifier;
use crate::parse::ParseContext;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::constraints::{MappingVariable, NamedVariables, SequenceBuilder, Variables};
use sdml_core::model::identifiers::Identifier;
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_DOMAIN, FIELD_NAME_RANGE, FIELD_NAME_VARIABLE,
    NODE_KIND_IDENTIFIER, NODE_KIND_LINE_COMMENT, NODE_KIND_MAPPING_VARIABLE,
    NODE_KIND_NAMED_VARIABLE_SET, NODE_KIND_QUANTIFIED_SENTENCE,
};
use sdml_error::Error;
use std::collections::HashSet;
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

    let child = node.child_by_field_name(FIELD_NAME_VARIABLE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;

    let variables: Variables = match child.kind() {
        NODE_KIND_NAMED_VARIABLE_SET => parse_named_variable_set(context, &mut child.walk())?,
        NODE_KIND_MAPPING_VARIABLE => parse_mapping_variable(context, &mut child.walk())?,
        // should check for NODE_KIND_LINE_COMMENT => {},
        _ => {
            unexpected_node!(
                context,
                RULE_NAME,
                child,
                [NODE_KIND_NAMED_VARIABLE_SET, NODE_KIND_MAPPING_VARIABLE,]
            );
        }
    };

    // returns `sequence_builder_body`
    let body = node.child_by_field_name(FIELD_NAME_BODY).unwrap();
    context.check_if_error(&body, RULE_NAME)?;

    for child in body.named_children(&mut body.walk()) {
        match child.kind() {
            NODE_KIND_QUANTIFIED_SENTENCE => {
                let body = parse_quantified_sentence(context, &mut child.walk())?;
                return Ok(SequenceBuilder::new(variables, body));
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(context, RULE_NAME, child, NODE_KIND_QUANTIFIED_SENTENCE);
            }
        }
    }
    unreachable!();
}

pub(crate) fn parse_named_variable_set<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Variables, Error> {
    rule_fn!("named_variable_set", cursor.node());

    let names = {
        let mut names: HashSet<Identifier> = Default::default();
        for name in cursor.node().named_children(cursor) {
            match name.kind() {
                NODE_KIND_IDENTIFIER => {
                    names.insert(parse_identifier(context, &name)?);
                }
                NODE_KIND_LINE_COMMENT => {}
                _ => {
                    unexpected_node!(context, RULE_NAME, name, NODE_KIND_IDENTIFIER);
                }
            }
        }
        names
    };

    Ok(Variables::Named(NamedVariables::new(names)))
}

pub(crate) fn parse_mapping_variable<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Variables, Error> {
    let node = cursor.node();
    rule_fn!("mapping_variable", node);

    let child = node.child_by_field_name(FIELD_NAME_DOMAIN).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let domain = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_RANGE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let range = parse_identifier(context, &child)?;

    Ok(Variables::Mapping(MappingVariable::new(domain, range)))
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
