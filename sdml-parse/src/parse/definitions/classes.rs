use crate::parse::{
    annotations::parse_annotation,
    constraints::{
        parse_function_body, parse_function_cardinality_expression, parse_function_signature,
    },
    definitions::{parse_from_definition_clause, parse_annotation_only_body},
    identifiers::{parse_identifier, parse_identifier_reference},
    parse_comment, ParseContext,
};
use sdml_core::{
    error::Error,
    load::ModuleLoader as ModuleLoaderTrait,
    model::{
        annotations::{Annotation, HasAnnotations},
        definitions::{
            HasOptionalFromDefinition, MethodDef, TypeClassArgument, TypeClassBody, TypeClassDef, TypeClassReference,
            TypeVariable,
        },
        HasOptionalBody, HasSourceSpan,
    },
    syntax::{
        FIELD_NAME_ARGUMENTS, FIELD_NAME_BODY, FIELD_NAME_CARDINALITY, FIELD_NAME_NAME,
        FIELD_NAME_RESTRICTION, FIELD_NAME_SIGNATURE, FIELD_NAME_VARIABLE, FIELD_NAME_WILDCARD,
        NODE_KIND_ANNOTATION, NODE_KIND_ANNOTATION_ONLY_BODY, NODE_KIND_FUNCTION_BODY,
        NODE_KIND_FUNCTION_SIGNATURE, NODE_KIND_IDENTIFIER, NODE_KIND_IDENTIFIER_REFERENCE,
        NODE_KIND_LINE_COMMENT, NODE_KIND_METHOD_DEF, NODE_KIND_TYPE_CLASS_BODY,
        NODE_KIND_TYPE_CLASS_REFERENCE, NODE_KIND_TYPE_OP_COMBINER,NODE_KIND_FROM_DEFINITION_CLAUSE,
    },
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

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;

    let mut variables: Vec<TypeVariable> = Default::default();

    for child in node.children_by_field_name(FIELD_NAME_VARIABLE, cursor) {
        check_node!(context, RULE_NAME, &node);
        variables.push(parse_type_variable(context, &mut child.walk())?);
    }

    let mut type_class = TypeClassDef::new(name.clone(), variables).with_source_span(node.into());

    context.start_type(&name)?;

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_TYPE_CLASS_BODY
    ) {
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

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER
    );
    let name = parse_identifier(context, &child)?;
    context.start_type(&name)?;

    let mut variable = TypeVariable::new(name).with_source_span(node.into());

    if let Some(child) = node.child_by_field_name(FIELD_NAME_CARDINALITY) {
        context.check_if_error(&child, RULE_NAME)?;
        let cardinality = parse_function_cardinality_expression(context, &mut child.walk())?;
        variable.set_cardinality(cardinality);
    }

    if let Some(child) =
        optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_RESTRICTION)
    {
        let restrictions = parse_type_variable_restriction(context, &mut child.walk())?;
        variable.extend_restrictions(restrictions);
    }

    Ok(variable)
}

fn parse_type_variable_restriction<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Vec<TypeClassReference>, Error> {
    let node = cursor.node();
    rule_fn!("type_variable_restriction", node);
    let mut restrictions = Vec::default();

    for child in node.named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        match child.kind() {
            NODE_KIND_TYPE_CLASS_REFERENCE => {
                restrictions.push(parse_type_class_reference(context, &mut child.walk())?);
            }
            NODE_KIND_TYPE_OP_COMBINER => {}
            NODE_KIND_LINE_COMMENT => {}
            _ => {
                unexpected_node!(
                    context,
                    RULE_NAME,
                    node,
                    [NODE_KIND_TYPE_CLASS_REFERENCE, NODE_KIND_TYPE_OP_COMBINER,]
                );
            }
        }
    }

    Ok(restrictions)
}

fn parse_type_class_reference<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<TypeClassReference, Error> {
    let node = cursor.node();
    rule_fn!("type_class_reference", node);

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_NAME,
        NODE_KIND_IDENTIFIER_REFERENCE
    );
    let name = parse_identifier_reference(context, &mut child.walk())?;
    let mut result = TypeClassReference::new(name).with_source_span(node.into());

    if let Some(child) = optional_node_field_named!(context, RULE_NAME, node, FIELD_NAME_ARGUMENTS)
    {
        if node.child_by_field_name(FIELD_NAME_WILDCARD).is_some() {
            result.add_to_arguments(TypeClassArgument::Wildcard);
        } else {
            for node in child.children_by_field_name(FIELD_NAME_VARIABLE, cursor) {
                check_node!(context, RULE_NAME, &node);
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
        check_node!(context, RULE_NAME, &node);
        match node.kind() {
            NODE_KIND_ANNOTATION => {
                body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
            }
            NODE_KIND_FROM_DEFINITION_CLAUSE => {
                body.set_from_definition(parse_from_definition_clause(context, &mut node.walk())?);
            }
            NODE_KIND_METHOD_DEF => {
                body.add_to_methods(parse_method_def(context, &mut node.walk())?);
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

    let child = node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_SIGNATURE,
        NODE_KIND_FUNCTION_SIGNATURE
    );
    let signature = parse_function_signature(context, &mut child.walk())?;
    let mut method = MethodDef::new(signature).with_source_span(node.into());

    if let Some(child) = optional_node_field_named!(
        context,
        RULE_NAME,
        node,
        FIELD_NAME_BODY,
        NODE_KIND_FUNCTION_BODY
    ) {
        method.set_body(parse_function_body(context, &mut child.walk())?);
    }

    for child in node.named_children(cursor) {
        check_node!(context, RULE_NAME, &node);
        if child.kind() == NODE_KIND_ANNOTATION_ONLY_BODY {
            let ann_body = parse_annotation_only_body(context, &mut child.walk())?;
            let annotations: Vec<Annotation> = ann_body.into();
            method.extend_annotations(annotations);
            break;
        }
    }

    Ok(method)
}
