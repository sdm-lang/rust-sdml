use crate::parse::annotations::parse_annotation;
use crate::parse::constraints::{
    parse_constraint_sentence, parse_function_cardinality_expression, parse_function_signature,
};
use crate::parse::definitions::parse_annotation_only_body;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use crate::parse::ParseContext;
use sdml_core::error::Error;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::annotations::{Annotation, HasAnnotations};
use sdml_core::model::definitions::{
    MethodDef, TypeClassArgument, TypeClassBody, TypeClassDef, TypeClassReference, TypeVariable,
};
use sdml_core::model::{HasOptionalBody, HasSourceSpan};
use sdml_core::syntax::{
    FIELD_NAME_ARGUMENTS, FIELD_NAME_BODY, FIELD_NAME_CARDINALITY, FIELD_NAME_NAME,
    FIELD_NAME_SIGNATURE, FIELD_NAME_VARIABLE, FIELD_NAME_WILDCARD, NODE_KIND_ANNOTATION,
    NODE_KIND_ANNOTATION_ONLY_BODY, NODE_KIND_FUNCTION_CARDINALITY_EXPRESSION,
    NODE_KIND_IDENTIFIER, NODE_KIND_LINE_COMMENT, NODE_KIND_METHOD_DEF,
    NODE_KIND_TYPE_CLASS_REFERENCE,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_type_class_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeClassDef, Error> {
    let node = cursor.node();
    rule_fn!("type_class_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    let mut variables: Vec<TypeVariable> = Default::default();

    for child in node.children_by_field_name(FIELD_NAME_VARIABLE, cursor) {
        context.check_if_error(&child, RULE_NAME)?;
        variables.push(parse_type_variable(context, &mut child.walk())?);
    }

    let mut type_class = TypeClassDef::new(name.clone(), variables).with_source_span(node.into());

    context.start_type(&name)?;

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        let body = parse_type_class_body(context, &mut child.walk())?;
        type_class.set_body(body);
    }

    context.end_type();
    Ok(type_class)
}

fn parse_type_variable<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeVariable, Error> {
    let node = cursor.node();
    rule_fn!("type_variable", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;
    context.start_type(&name)?;

    let mut variable = TypeVariable::new(name).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_CARDINALITY) {
        context.check_if_error(&child, RULE_NAME)?;
        let cardinality = parse_function_cardinality_expression(context, &mut child.walk())?;
        variable.set_cardinality(cardinality);
    }

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_TYPE_CLASS_REFERENCE => {
                variable
                    .add_to_restrictions(parse_type_class_reference(context, &mut node.walk())?);
            }
            NODE_KIND_LINE_COMMENT => {}
            NODE_KIND_IDENTIFIER => {}
            NODE_KIND_FUNCTION_CARDINALITY_EXPRESSION => {}
            _ => {
                unexpected_node!(context, RULE_NAME, node, [NODE_KIND_TYPE_CLASS_REFERENCE,]);
            }
        }
    }

    Ok(variable)
}

fn parse_type_class_reference<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeClassReference, Error> {
    let node = cursor.node();
    rule_fn!("type_class_reference", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier_reference(context, &mut child.walk())?;

    let mut result = TypeClassReference::new(name).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_ARGUMENTS) {
        context.check_if_error(&child, RULE_NAME)?;

        if node.child_by_field_name(FIELD_NAME_WILDCARD).is_some() {
            result.add_to_arguments(TypeClassArgument::Wildcard);
        } else {
            for node in child.children_by_field_name(FIELD_NAME_VARIABLE, cursor) {
                context.check_if_error(&node, RULE_NAME)?;
                let variable = parse_type_class_reference(context, &mut node.walk())?;
                result.add_to_arguments(TypeClassArgument::Reference(Box::new(variable)));
            }
        }
    }

    Ok(result)
}

pub(crate) fn parse_type_class_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeClassBody, Error> {
    rule_fn!("type_class_body", cursor.node());
    let mut body = TypeClassBody::default().with_source_span(cursor.node().into());

    for node in cursor.node().named_children(cursor) {
        context.check_if_error(&node, RULE_NAME)?;
        match node.kind() {
            NODE_KIND_ANNOTATION => {
                body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
            }
            NODE_KIND_METHOD_DEF => {
                body.add_to_methods(parse_method_def(context, &mut node.walk())?);
            }
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [NODE_KIND_ANNOTATION, NODE_KIND_METHOD_DEF,]
                );
            }
        }
    }

    Ok(body)
}

fn parse_method_def<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<MethodDef, Error> {
    let node = cursor.node();
    rule_fn!("method_def", node);

    let child = node.child_by_field_name(FIELD_NAME_NAME).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let name = parse_identifier(context, &child)?;

    context.start_member(&name)?;

    let child = node.child_by_field_name(FIELD_NAME_SIGNATURE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let signature = parse_function_signature(context, &mut child.walk())?;

    let mut method = MethodDef::new(name, signature).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_BODY) {
        context.check_if_error(&child, RULE_NAME)?;
        method.set_body(parse_constraint_sentence(context, &mut child.walk())?);
    }

    for child in node.named_children(cursor) {
        context.check_if_error(&child, RULE_NAME)?;

        if child.kind() == NODE_KIND_ANNOTATION_ONLY_BODY {
            let ann_body = parse_annotation_only_body(context, &mut child.walk())?;
            let annotations: Vec<Annotation> = ann_body.into();
            method.extend_annotations(annotations);
            break;
        }
    }

    Ok(method)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
