use crate::parse::constraints::formal::functions::parse_function_def;
use crate::parse::constraints::formal::sentences::parse_constraint_sentence;
use crate::parse::constraints::formal::values::parse_predicate_value;
use crate::parse::identifiers::parse_identifier;
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::model::constraints::EnvironmentDef;
use sdml_core::model::identifiers::Identifier;
use sdml_core::syntax::{
    FIELD_NAME_BODY, FIELD_NAME_NAME, NODE_KIND_CONSTANT_DEF, NODE_KIND_CONSTRAINT_ENVIRONMENT_END,
    NODE_KIND_CONSTRAINT_SENTENCE, NODE_KIND_ENVIRONMENT_DEF, NODE_KIND_FUNCTION_DEF,
    NODE_KIND_LINE_COMMENT, NODE_KIND_PREDICATE_VALUE,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_constraint_environment<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Vec<EnvironmentDef>, Error> {
    rule_fn!("constraint_environment", cursor.node());

    let mut environment: Vec<EnvironmentDef> = Default::default();

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_ENVIRONMENT_DEF => {
                environment.push(parse_environment_def(context, &mut node.walk())?);
            }
            NODE_KIND_CONSTRAINT_ENVIRONMENT_END => {
                break;
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [
                        NODE_KIND_ENVIRONMENT_DEF,
                        NODE_KIND_CONSTRAINT_ENVIRONMENT_END,
                    ]
                );
            }
        }
    }
    Ok(environment)
}

fn parse_environment_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EnvironmentDef, Error> {
    let node = cursor.node();
    rule_fn!("environment_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let child = node.child_by_field_name(FIELD_NAME_BODY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    Ok(match child.kind() {
        NODE_KIND_FUNCTION_DEF => {
            let body = parse_function_def(context, &mut child.walk())?;
            EnvironmentDef::new(name, body.into())
        }
        NODE_KIND_CONSTANT_DEF => parse_constant_def(name, context, &mut child.walk())?,
        _ => {
            unexpected_node!(
                context,
                RULE_NAME,
                child,
                [NODE_KIND_FUNCTION_DEF, NODE_KIND_CONSTANT_DEF,]
            );
        }
    })
}

fn parse_constant_def<'a>(
    name: Identifier,
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<EnvironmentDef, Error> {
    let node = cursor.node();
    rule_fn!("constant_def", node);

    let child = node.child_by_field_name(FIELD_NAME_BODY).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    Ok(match child.kind() {
        NODE_KIND_PREDICATE_VALUE => {
            let body = parse_predicate_value(context, &mut child.walk())?;
            EnvironmentDef::new(name, body.into())
        }
        NODE_KIND_CONSTRAINT_SENTENCE => {
            let body = parse_constraint_sentence(context, &mut child.walk())?;
            EnvironmentDef::new(name, body.into())
        }
        _ => {
            unexpected_node!(
                context,
                RULE_NAME,
                child,
                [NODE_KIND_PREDICATE_VALUE, NODE_KIND_CONSTRAINT_SENTENCE,]
            );
        }
    })
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
